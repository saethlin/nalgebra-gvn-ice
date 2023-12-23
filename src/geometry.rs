macro_rules! md_assign_impl(
    (
        $Op: ident,
        $op: ident $(where N: $($ScalarBounds: ident), *) *;
        ($R1: ty, $C1: ty),
        ($R2: ty, $C2: ty) for $($Dims: ident: $DimsBound: ident $(< $($BoundParam: ty), *>) *),
        + $(
            where $ConstraintType: ty: $ConstraintBound: ident $(
                < $($ConstraintBoundParams: ty $(= $EqBound: ty) *),
                *>
            ) *
        ) *;
        $lhs: ident: $Lhs: ty,
        $rhs: ident: $Rhs: ty;
        $action: expr;
        $($lives: tt),
        *
    ) => {
        impl < $(
            $lives,
        ) * N $(
            ,
            $Dims: $DimsBound $(< $($BoundParam), *>) *
        ) *> $Op < $Rhs > for $Lhs where N: Scalar + Zero + One + $($(+ $ScalarBounds) *) *,
        DefaultAllocator: Allocator < N,
        $R1,
        $C1 >+ Allocator < N,
        $R2,
        $C2 >,
        $($ConstraintType: $ConstraintBound $(< $($ConstraintBoundParams $(= $EqBound) *), *>) *),
        * {
            #[inline] fn $op(& mut $lhs, $rhs: $Rhs) {
                $action
            }
        }
    }
);

macro_rules! md_assign_impl_all(
    (
        $Op: ident,
        $op: ident $(where N: $($ScalarBounds: ident), *) *;
        ($R1: ty, $C1: ty),
        ($R2: ty, $C2: ty) for $($Dims: ident: $DimsBound: ident $(< $($BoundParam: ty), *>) *),
        + $(
            where $ConstraintType: ty: $ConstraintBound: ident $(
                < $($ConstraintBoundParams: ty $(= $EqBound: ty) *),
                *>
            ) *
        ) *;
        $lhs: ident: $Lhs: ty,
        $rhs: ident: $Rhs: ty;
        [val] => $action_val: expr;
        [ref] => $action_ref: expr;
    ) => {
        md_assign_impl!(
            $Op,
            $op $(where N: $($ScalarBounds), *) *;
            ($R1, $C1),
            ($R2, $C2) for $($Dims: $DimsBound $(< $($BoundParam), *>) *),
            + $(where $ConstraintType: $ConstraintBound $(< $($ConstraintBoundParams $(= $EqBound) *), *>) *) *;
            $lhs: $Lhs,
            $rhs: $Rhs;
            $action_val;
        );
    }
);
use crate::base::allocator::Allocator;
use crate::base::dimension::Dim;
use crate::base::dimension::DimName;
use crate::base::dimension::U2;
use crate::base::storage::Storage;
use crate::base::DefaultAllocator;
use crate::base::Matrix;
use crate::base::{Matrix2, Unit};
use crate::base::{MatrixN, Scalar};
use alga::general::RealField;
use num::{One, Zero};
use num_complex::Complex;
use std::ops::MulAssign;

type Rotation2<N> = Rotation<N, U2>;

pub(crate) type UnitComplex<N> = Unit<Complex<N>>;

impl<N: RealField> UnitComplex<N> {
    #[inline]
    pub(crate) fn to_rotation_matrix(&self) -> Rotation2<N> {
        let r = self.re;
        let i = self.im;
        Rotation2::from_matrix_unchecked(Matrix2::new(r, -i, i, r))
    }
}

impl<'b, N: RealField> MulAssign<&'b UnitComplex<N>> for Rotation<N, U2>
where
    DefaultAllocator: Allocator<N, U2, U2>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: &'b UnitComplex<N>) {
        self.mul_assign(rhs.to_rotation_matrix())
    }
}

#[repr(C)]
pub(crate) struct Rotation<N: Scalar, D: DimName>
where
    DefaultAllocator: Allocator<N, D, D>,
{
    matrix: MatrixN<N, D>,
}

impl<N: Scalar, D: DimName> Rotation<N, D>
where
    DefaultAllocator: Allocator<N, D, D>,
{
    #[inline]
    pub(crate) fn matrix_mut_unchecked(&mut self) -> &mut MatrixN<N, D> {
        &mut self.matrix
    }

    #[inline]
    pub(crate) fn into_inner(self) -> MatrixN<N, D> {
        self.matrix
    }

    #[inline]
    pub(crate) fn from_matrix_unchecked(matrix: MatrixN<N, D>) -> Self {
        assert!(
            matrix.is_square(),
            "Unable to create a rotation from a non-square matrix."
        );
        Self { matrix: matrix }
    }
}

md_assign_impl_all!(
    MulAssign,
    mul_assign;
    (D, D),
    (D, D) for D: DimName;
    self: Rotation < N,
    D >,
    right: Rotation < N,
    D >;
    [val] => self.matrix_mut_unchecked().mul_assign(right.into_inner());
    [ref] => self.matrix_mut_unchecked().mul_assign(right.matrix());
);

impl<N: Scalar, R: Dim, C: Dim, S: Storage<N, R, C>> Matrix<N, R, C, S> {
    #[inline]
    pub(crate) fn is_square(&self) -> bool {
        let (nrows, ncols) = self.shape();
        nrows == ncols
    }
}
