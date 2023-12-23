use crate::base::allocator::Allocator;
use crate::base::dimension::{Dim, DimName, U2};
use crate::base::{DefaultAllocator, MatrixMN, Scalar};

impl<N: Scalar, R: Dim, C: Dim> MatrixMN<N, R, C>
where
    DefaultAllocator: Allocator<N, R, C>,
{
    #[inline]
    pub(crate) unsafe fn new_uninitialized_generic(nrows: R, ncols: C) -> Self {
        Self::from_data(DefaultAllocator::allocate_uninitialized(nrows, ncols))
    }
}

macro_rules! impl_constructors(
    ($($Dims: ty), *; $(=> $DimIdent: ident: $DimBound: ident), *; $($gargs: expr), *; $($args: ident), *) => {
        impl < N: Scalar,
        $($DimIdent: $DimBound,) *> MatrixMN < N $(, $Dims) *> where DefaultAllocator: Allocator < N $(, $Dims) *> {
            #[inline] pub unsafe fn new_uninitialized($($args: usize), *) -> Self {
                Self:: new_uninitialized_generic($($gargs), *)
            }
        }
    }
);

impl_constructors!(
    R,
    C;
    => R: DimName,
    => C: DimName;
    R::name(),
    C::name();
);

macro_rules! componentwise_constructors_impl(
    ($($R: ty, $C: ty, $($args: ident:($irow: expr, $icol: expr)), *); * $(;) *) => {
        $(impl < N > MatrixMN < N, $R, $C > where N: Scalar, DefaultAllocator: Allocator < N, $R, $C > {
            #[doc = " Initializes this matrix from its components."] #[inline] pub fn new($($args: N), *) -> Self {
                unsafe {
                    let mut res = Self::new_uninitialized();
                    $(* res.get_unchecked_mut(($irow, $icol)) = $args;) * res
                }
            }
        }) *
    }
);

componentwise_constructors_impl!(
    U2,
    U2,
    m11:(0, 0),
    m12:(0, 1),
    m21:(1, 0),
    m22:(1, 1);
);
