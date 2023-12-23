#![doc(
    html_favicon_url = "https://nalgebra.org/img/favicon.ico",
    html_root_url = "https://nalgebra.org/rustdoc"
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", not(feature = "std")), feature(alloc))]

#[macro_use]
extern crate approx;
extern crate generic_array;
#[cfg(feature = "std")]
extern crate matrixmultiply;
extern crate num_complex;
extern crate num_traits as num;
extern crate num_rational;
extern crate rand;
#[cfg(feature = "std")]
extern crate rand_distr;
extern crate typenum;

extern crate alga;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[cfg(not(feature = "std"))]
extern crate core as std;

mod base;
pub mod geometry;
mod linalg;

use crate::base::*;
#[cfg(feature = "sparse")]
use crate::sparse::*;

use std::cmp::{self, Ordering, PartialOrd};

use alga::general::{
    Additive, AdditiveGroup, Identity, TwoSidedInverse, JoinSemilattice, Lattice, MeetSemilattice,
    Multiplicative, SupersetOf,
};
use alga::linear::SquareMatrix as AlgaSquareMatrix;
use alga::linear::{EuclideanSpace, FiniteDimVectorSpace, InnerSpace, NormedSpace};
use num::Signed;

use alga::general::{Id, RealField, ComplexField};

/// Gets the ubiquitous multiplicative identity element.
///
/// Same as `Id::new()`.
#[deprecated(note = "use `Id::new()` instead.")]
#[inline]
fn id() -> Id {
    Id::new()
}

/// Gets the multiplicative identity element.
///
/// # See also:
///
/// * [`origin`](../nalgebra/fn.origin.html)
/// * [`zero`](fn.zero.html)
#[inline]
fn one<T: Identity<Multiplicative>>() -> T {
    T::identity()
}

/// Gets the additive identity element.
///
/// # See also:
///
/// * [`one`](fn.one.html)
/// * [`origin`](../nalgebra/fn.origin.html)
#[inline]
fn zero<T: Identity<Additive>>() -> T {
    T::identity()
}

/// Gets the origin of the given point.
///
/// # See also:
///
/// * [`one`](fn.one.html)
/// * [`zero`](fn.zero.html)
///
/// # Deprecated
/// Use [Point::origin] instead.
///
/// Or, use [EuclideanSpace::origin](https://docs.rs/alga/0.7.2/alga/linear/trait.EuclideanSpace.html#tymethod.origin).
#[deprecated(note = "use `Point::origin` instead")]
#[inline]
fn origin<P: EuclideanSpace>() -> P {
    P::origin()
}

/*
 *
 * Dimension
 *
 */
/// The dimension of the given algebraic entity seen as a vector space.
#[inline]
fn dimension<V: FiniteDimVectorSpace>() -> usize {
    V::dimension()
}

/*
 *
 * Ordering
 *
 */
// XXX: this is very naive and could probably be optimized for specific types.
// XXX: also, we might just want to use divisions, but assuming `val` is usually not far from `min`
// or `max`, would it still be more efficient?
/// Wraps `val` into the range `[min, max]` using modular arithmetics.
///
/// The range must not be empty.
#[inline]
fn wrap<T>(mut val: T, min: T, max: T) -> T
where T: Copy + PartialOrd + AdditiveGroup {
    assert!(min < max, "Invalid wrapping bounds.");
    let width = max - min;

    if val < min {
        val += width;

        while val < min {
            val += width
        }

        val
    } else if val > max {
        val -= width;

        while val > max {
            val -= width
        }

        val
    } else {
        val
    }
}

/// Returns a reference to the input value clamped to the interval `[min, max]`.
///
/// In particular:
///     * If `min < val < max`, this returns `val`.
///     * If `val <= min`, this returns `min`.
///     * If `val >= max`, this returns `max`.
#[inline]
fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val > min {
        if val < max {
            val
        } else {
            max
        }
    } else {
        min
    }
}

/// Same as `cmp::max`.
#[inline]
fn max<T: Ord>(a: T, b: T) -> T {
    cmp::max(a, b)
}

/// Same as `cmp::min`.
#[inline]
fn min<T: Ord>(a: T, b: T) -> T {
    cmp::min(a, b)
}

/// The absolute value of `a`.
///
/// Deprecated: Use [Matrix::abs] or [RealField::abs] instead.
#[deprecated(note = "use `Matrix::abs` or `RealField::abs` instead")]
#[inline]
fn abs<T: Signed>(a: &T) -> T {
    a.abs()
}

/// Returns the infimum of `a` and `b`.
#[inline]
fn inf<T: MeetSemilattice>(a: &T, b: &T) -> T {
    a.meet(b)
}

/// Returns the supremum of `a` and `b`.
#[inline]
fn sup<T: JoinSemilattice>(a: &T, b: &T) -> T {
    a.join(b)
}

/// Returns simultaneously the infimum and supremum of `a` and `b`.
#[inline]
fn inf_sup<T: Lattice>(a: &T, b: &T) -> (T, T) {
    a.meet_join(b)
}

/// Compare `a` and `b` using a partial ordering relation.
#[inline]
fn partial_cmp<T: PartialOrd>(a: &T, b: &T) -> Option<Ordering> {
    a.partial_cmp(b)
}

/// Returns `true` iff `a` and `b` are comparable and `a < b`.
#[inline]
fn partial_lt<T: PartialOrd>(a: &T, b: &T) -> bool {
    a.lt(b)
}

/// Returns `true` iff `a` and `b` are comparable and `a <= b`.
#[inline]
fn partial_le<T: PartialOrd>(a: &T, b: &T) -> bool {
    a.le(b)
}

/// Returns `true` iff `a` and `b` are comparable and `a > b`.
#[inline]
fn partial_gt<T: PartialOrd>(a: &T, b: &T) -> bool {
    a.gt(b)
}

/// Returns `true` iff `a` and `b` are comparable and `a >= b`.
#[inline]
fn partial_ge<T: PartialOrd>(a: &T, b: &T) -> bool {
    a.ge(b)
}

/// Return the minimum of `a` and `b` if they are comparable.
#[inline]
fn partial_min<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> Option<&'a T> {
    if let Some(ord) = a.partial_cmp(b) {
        match ord {
            Ordering::Greater => Some(b),
            _ => Some(a),
        }
    } else {
        None
    }
}

/// Return the maximum of `a` and `b` if they are comparable.
#[inline]
fn partial_max<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> Option<&'a T> {
    if let Some(ord) = a.partial_cmp(b) {
        match ord {
            Ordering::Less => Some(b),
            _ => Some(a),
        }
    } else {
        None
    }
}

/// Clamp `value` between `min` and `max`. Returns `None` if `value` is not comparable to
/// `min` or `max`.
#[inline]
fn partial_clamp<'a, T: PartialOrd>(value: &'a T, min: &'a T, max: &'a T) -> Option<&'a T> {
    if let (Some(cmp_min), Some(cmp_max)) = (value.partial_cmp(min), value.partial_cmp(max)) {
        if cmp_min == Ordering::Less {
            Some(min)
        } else if cmp_max == Ordering::Greater {
            Some(max)
        } else {
            Some(value)
        }
    } else {
        None
    }
}

/// Sorts two values in increasing order using a partial ordering.
#[inline]
fn partial_sort2<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> Option<(&'a T, &'a T)> {
    if let Some(ord) = a.partial_cmp(b) {
        match ord {
            Ordering::Less => Some((a, b)),
            _ => Some((b, a)),
        }
    } else {
        None
    }
}

/*
 * Inverse
 */

/// Tries to gets an inverted copy of a square matrix.
///
/// # See also:
///
/// * [`inverse`](fn.inverse.html)
#[deprecated(note = "use the `.try_inverse()` method instead")]
#[inline]
fn try_inverse<M: AlgaSquareMatrix>(m: &M) -> Option<M> {
    m.try_inverse()
}

/// Computes the multiplicative inverse of an (always invertible) algebraic entity.
///
/// # See also:
///
/// * [`try_inverse`](fn.try_inverse.html)
#[deprecated(note = "use the `.inverse()` method instead")]
#[inline]
fn inverse<M: TwoSidedInverse<Multiplicative>>(m: &M) -> M {
    m.two_sided_inverse()
}

/*
 * Inner vector space
 */

/// Computes the dot product of two vectors.
///
/// ## Deprecated
/// Use these methods instead:
///   - [Matrix::dot]
///   - [Quaternion::dot]
///
/// Or, use [FiniteDimVectorSpace::dot](https://docs.rs/alga/0.7.2/alga/linear/trait.FiniteDimVectorSpace.html#tymethod.dot).
#[deprecated(note = "use `Matrix::dot` or `Quaternion::dot` instead")]
#[inline]
fn dot<V: FiniteDimVectorSpace>(a: &V, b: &V) -> V::Field {
    a.dot(b)
}

/// Computes the smallest angle between two vectors.
///
/// ## Deprecated
/// Use [Matrix::angle] instead.
///
/// Or, use [InnerSpace::angle](https://docs.rs/alga/0.7.2/alga/linear/trait.InnerSpace.html#method.angle).
#[deprecated(note = "use `Matrix::angle` instead")]
#[inline]
fn angle<V: InnerSpace>(a: &V, b: &V) -> V::RealField {
    a.angle(b)
}

/*
 * Normed space
 */

/// Computes the L2 (Euclidean) norm of a vector.
///
/// # See also:
///
/// * [`magnitude`](fn.magnitude.html)
/// * [`magnitude_squared`](fn.magnitude_squared.html)
/// * [`norm_squared`](fn.norm_squared.html)
///
/// # Deprecated
/// Use these methods instead:
/// * [Matrix::norm]
/// * [Quaternion::norm]
///
/// Or, use [NormedSpace::norm](https://docs.rs/alga/0.7.2/alga/linear/trait.NormedSpace.html#tymethod.norm).
#[deprecated(note = "use `Matrix::norm` or `Quaternion::norm` instead")]
#[inline]
fn norm<V: NormedSpace>(v: &V) -> V::RealField {
    v.norm()
}

/// Computes the squared L2 (Euclidean) norm of the vector `v`.
///
/// # See also:
///
/// * [`magnitude`](fn.magnitude.html)
/// * [`magnitude_squared`](fn.magnitude_squared.html)
/// * [`norm`](fn.norm.html)
///
/// # Deprecated
/// Use these methods instead:
/// * [Matrix::norm_squared]
/// * [Quaternion::norm_squared]
///
/// Or, use [NormedSpace::norm_squared](https://docs.rs/alga/0.7.2/alga/linear/trait.NormedSpace.html#tymethod.norm_squared).
#[deprecated(note = "use `Matrix::norm_squared` or `Quaternion::norm_squared` instead")]
#[inline]
fn norm_squared<V: NormedSpace>(v: &V) -> V::RealField {
    v.norm_squared()
}

/// A synonym for [`norm`](fn.norm.html), aka length.
///
/// # See also:
///
/// * [`magnitude_squared`](fn.magnitude_squared.html)
/// * [`norm`](fn.norm.html)
/// * [`norm_squared`](fn.norm_squared.html)
///
/// # Deprecated
/// Use these methods instead:
/// * [Matrix::magnitude]
/// * [Quaternion::magnitude]
///
/// Or, use [NormedSpace::norm](https://docs.rs/alga/0.7.2/alga/linear/trait.NormedSpace.html#tymethod.norm).
#[deprecated(note = "use `Matrix::magnitude` or `Quaternion::magnitude` instead")]
#[inline]
fn magnitude<V: NormedSpace>(v: &V) -> V::RealField {
    v.norm()
}

/// A synonym for [`norm_squared`](fn.norm_squared.html),
///  aka length squared.
///
/// # See also:
///
/// * [`magnitude`](fn.magnitude.html)
/// * [`norm`](fn.norm.html)
/// * [`norm_squared`](fn.norm_squared.html)
///
/// # Deprecated
/// Use these methods instead:
/// * [Matrix::magnitude_squared]
/// * [Quaternion::magnitude_squared]
///
/// Or, use [NormedSpace::norm_squared](https://docs.rs/alga/0.7.2/alga/linear/trait.NormedSpace.html#tymethod.norm_squared).
#[deprecated(note = "use `Matrix::magnitude_squared` or `Quaternion::magnitude_squared` instead")]
#[inline]
fn magnitude_squared<V: NormedSpace>(v: &V) -> V::RealField {
    v.norm_squared()
}

/// Computes the normalized version of the vector `v`.
///
/// # Deprecated
/// Use these methods instead:
/// * [Matrix::normalize]
/// * [Quaternion::normalize]
///
/// Or, use [NormedSpace::normalize](https://docs.rs/alga/0.7.2/alga/linear/trait.NormedSpace.html#tymethod.normalize).
#[deprecated(note = "use `Matrix::normalize` or `Quaternion::normalize` instead")]
#[inline]
fn normalize<V: NormedSpace>(v: &V) -> V {
    v.normalize()
}

/// Computes the normalized version of the vector `v` or returns `None` if its norm is smaller than `min_norm`.
///
/// # Deprecated
/// Use these methods instead:
/// * [Matrix::try_normalize]
/// * [Quaternion::try_normalize]
///
/// Or, use [NormedSpace::try_normalize](https://docs.rs/alga/0.7.2/alga/linear/trait.NormedSpace.html#tymethod.try_normalize).
#[deprecated(note = "use `Matrix::try_normalize` or `Quaternion::try_normalize` instead")]
#[inline]
fn try_normalize<V: NormedSpace>(v: &V, min_norm: V::RealField) -> Option<V> {
    v.try_normalize(min_norm)
}

/*
 *
 * Point operations.
 *
 */
/// The center of two points.
///
/// # See also:
///
/// * [distance](fn.distance.html)
/// * [distance_squared](fn.distance_squared.html)
#[inline]
fn center<P: EuclideanSpace>(p1: &P, p2: &P) -> P {
    P::from_coordinates((p1.coordinates() + p2.coordinates()) * convert(0.5))
}

/// The distance between two points.
///
/// # See also:
///
/// * [center](fn.center.html)
/// * [distance_squared](fn.distance_squared.html)
#[inline]
fn distance<P: EuclideanSpace>(p1: &P, p2: &P) -> P::RealField {
    (p2.coordinates() - p1.coordinates()).norm()
}

/// The squared distance between two points.
///
/// # See also:
///
/// * [center](fn.center.html)
/// * [distance](fn.distance.html)
#[inline]
fn distance_squared<P: EuclideanSpace>(p1: &P, p2: &P) -> P::RealField {
    (p2.coordinates() - p1.coordinates()).norm_squared()
}

/*
 * Cast
 */
/// Converts an object from one type to an equivalent or more general one.
///
/// See also [`try_convert`](fn.try_convert.html) for conversion to more specific types.
///
/// # See also:
///
/// * [convert_ref](fn.convert_ref.html)
/// * [convert_ref_unchecked](fn.convert_ref_unchecked.html)
/// * [is_convertible](../nalgebra/fn.is_convertible.html)
/// * [try_convert](fn.try_convert.html)
/// * [try_convert_ref](fn.try_convert_ref.html)
#[inline]
fn convert<From, To: SupersetOf<From>>(t: From) -> To {
    To::from_subset(&t)
}

/// Attempts to convert an object to a more specific one.
///
/// See also [`convert`](fn.convert.html) for conversion to more general types.
///
/// # See also:
///
/// * [convert](fn.convert.html)
/// * [convert_ref](fn.convert_ref.html)
/// * [convert_ref_unchecked](fn.convert_ref_unchecked.html)
/// * [is_convertible](../nalgebra/fn.is_convertible.html)
/// * [try_convert_ref](fn.try_convert_ref.html)
#[inline]
fn try_convert<From: SupersetOf<To>, To>(t: From) -> Option<To> {
    t.to_subset()
}

/// Indicates if [`try_convert`](fn.try_convert.html) will succeed without
/// actually performing the conversion.
///
/// # See also:
///
/// * [convert](fn.convert.html)
/// * [convert_ref](fn.convert_ref.html)
/// * [convert_ref_unchecked](fn.convert_ref_unchecked.html)
/// * [try_convert](fn.try_convert.html)
/// * [try_convert_ref](fn.try_convert_ref.html)
#[inline]
 fn is_convertible<From: SupersetOf<To>, To>(t: &From) -> bool {
    t.is_in_subset()
}

/// Use with care! Same as [`try_convert`](fn.try_convert.html) but
/// without any property checks.
///
/// # See also:
///
/// * [convert](fn.convert.html)
/// * [convert_ref](fn.convert_ref.html)
/// * [convert_ref_unchecked](fn.convert_ref_unchecked.html)
/// * [is_convertible](../nalgebra/fn.is_convertible.html)
/// * [try_convert](fn.try_convert.html)
/// * [try_convert_ref](fn.try_convert_ref.html)
#[inline]
unsafe fn convert_unchecked<From: SupersetOf<To>, To>(t: From) -> To {
    t.to_subset_unchecked()
}

/// Converts an object from one type to an equivalent or more general one.
///
/// # See also:
///
/// * [convert](fn.convert.html)
/// * [convert_ref_unchecked](fn.convert_ref_unchecked.html)
/// * [is_convertible](../nalgebra/fn.is_convertible.html)
/// * [try_convert](fn.try_convert.html)
/// * [try_convert_ref](fn.try_convert_ref.html)
#[inline]
fn convert_ref<From, To: SupersetOf<From>>(t: &From) -> To {
    To::from_subset(t)
}

/// Attempts to convert an object to a more specific one.
///
/// # See also:
///
/// * [convert](fn.convert.html)
/// * [convert_ref](fn.convert_ref.html)
/// * [convert_ref_unchecked](fn.convert_ref_unchecked.html)
/// * [is_convertible](../nalgebra/fn.is_convertible.html)
/// * [try_convert](fn.try_convert.html)
#[inline]
fn try_convert_ref<From: SupersetOf<To>, To>(t: &From) -> Option<To> {
    t.to_subset()
}

/// Use with care! Same as [`try_convert`](fn.try_convert.html) but
/// without any property checks.
///
/// # See also:
///
/// * [convert](fn.convert.html)
/// * [convert_ref](fn.convert_ref.html)
/// * [is_convertible](../nalgebra/fn.is_convertible.html)
/// * [try_convert](fn.try_convert.html)
/// * [try_convert_ref](fn.try_convert_ref.html)
#[inline]
unsafe fn convert_ref_unchecked<From: SupersetOf<To>, To>(t: &From) -> To {
    t.to_subset_unchecked()
}
