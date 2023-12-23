use crate::base::allocator::Allocator;
use crate::base::allocator::DefaultAllocator;
use crate::base::dimension::{Dim, U1};
use crate::base::Scalar;

pub(crate) type Owned<N, R, C = U1> = <DefaultAllocator as Allocator<N, R, C>>::Buffer;

pub(crate) unsafe trait Storage<N: Scalar, R: Dim, C: Dim = U1>: Sized {
    type RStride: Dim;
    type CStride: Dim;

    fn shape(&self) -> (R, C) {
        loop {}
    }
}

pub(crate) unsafe trait StorageMut<N: Scalar, R: Dim, C: Dim = U1>:
    Storage<N, R, C>
{
    #[inline]
    unsafe fn get_unchecked_mut(&mut self, irow: usize, icol: usize) -> &mut N {
        loop {}
    }
}

pub(crate) unsafe trait ContiguousStorage<N: Scalar, R: Dim, C: Dim = U1>:
    Storage<N, R, C>
{
}

pub(crate) unsafe trait ContiguousStorageMut<N: Scalar, R: Dim, C: Dim = U1>:
    ContiguousStorage<N, R, C> + StorageMut<N, R, C>
{
}
