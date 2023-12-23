use crate::base::allocator::Allocator;
use crate::base::allocator::DefaultAllocator;
use crate::base::dimension::{DimName, U1};
use crate::base::storage::{ContiguousStorage, ContiguousStorageMut, Storage, StorageMut};
use crate::base::Scalar;
use generic_array::{ArrayLength, GenericArray};
use std::ops::Mul;
use typenum::Prod;

#[repr(C)]
pub(crate) struct ArrayStorage<N, R, C>
where
    R: DimName,
    C: DimName,
    R::Value: Mul<C::Value>,
    Prod<R::Value, C::Value>: ArrayLength<N>,
{
    data: GenericArray<N, Prod<R::Value, C::Value>>,
}

impl<N, R, C> Clone for ArrayStorage<N, R, C>
where
    N: Clone,
    R: DimName,
    C: DimName,
    R::Value: Mul<C::Value>,
    Prod<R::Value, C::Value>: ArrayLength<N>,
{
    #[inline]
    fn clone(&self) -> Self {
        loop {}
    }
}

unsafe impl<N, R, C> Storage<N, R, C> for ArrayStorage<N, R, C>
where
    N: Scalar,
    R: DimName,
    C: DimName,
    R::Value: Mul<C::Value>,
    Prod<R::Value, C::Value>: ArrayLength<N>,
    DefaultAllocator: Allocator<N, R, C, Buffer = Self>,
{
    type RStride = U1;
    type CStride = R;

    #[inline]
    fn shape(&self) -> (R, C) {
        (R::name(), C::name())
    }
}

unsafe impl<N, R, C> StorageMut<N, R, C> for ArrayStorage<N, R, C>
where
    N: Scalar,
    R: DimName,
    C: DimName,
    R::Value: Mul<C::Value>,
    Prod<R::Value, C::Value>: ArrayLength<N>,
    DefaultAllocator: Allocator<N, R, C, Buffer = Self>,
{
}

unsafe impl<N, R, C> ContiguousStorage<N, R, C> for ArrayStorage<N, R, C>
where
    N: Scalar,
    R: DimName,
    C: DimName,
    R::Value: Mul<C::Value>,
    Prod<R::Value, C::Value>: ArrayLength<N>,
    DefaultAllocator: Allocator<N, R, C, Buffer = Self>,
{
}

unsafe impl<N, R, C> ContiguousStorageMut<N, R, C> for ArrayStorage<N, R, C>
where
    N: Scalar,
    R: DimName,
    C: DimName,
    R::Value: Mul<C::Value>,
    Prod<R::Value, C::Value>: ArrayLength<N>,
    DefaultAllocator: Allocator<N, R, C, Buffer = Self>,
{
}
