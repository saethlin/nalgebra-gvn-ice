use crate::base::array_storage::ArrayStorage;
use crate::base::dimension::DimName;
use crate::base::dimension::{Dim, U1};
use crate::base::storage::ContiguousStorageMut;
use crate::base::Scalar;
use generic_array::ArrayLength;
use std::any::Any;
use std::ops::Mul;
use typenum::Prod;

pub(crate) trait Allocator<N: Copy + PartialEq + 'static, R: Dim, C: Dim = U1>:
    Any + Sized
{
    type Buffer: ContiguousStorageMut<N, R, C> + Clone;

    unsafe fn allocate_uninitialized(nrows: R, ncols: C) -> Self::Buffer;
}

pub(crate) struct DefaultAllocator;

impl<N, R, C> Allocator<N, R, C> for DefaultAllocator
where
    N: Scalar,
    R: DimName,
    C: DimName,
    R::Value: Mul<C::Value>,
    Prod<R::Value, C::Value>: ArrayLength<N>,
{
    type Buffer = ArrayStorage<N, R, C>;

    #[inline]
    unsafe fn allocate_uninitialized(_: R, _: C) -> Self::Buffer {
        loop {}
    }
}
