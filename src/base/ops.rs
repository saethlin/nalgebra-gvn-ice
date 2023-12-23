use crate::base::allocator::Allocator;
use crate::base::dimension::Dim;
use crate::base::storage::{ContiguousStorageMut, Storage};
use crate::base::{DefaultAllocator, Matrix, Scalar};
use num::{One, Zero};
use std::ops::MulAssign;

impl<N, R1, C1, R2, SA, SB> MulAssign<Matrix<N, R2, C1, SB>> for Matrix<N, R1, C1, SA>
where
    R1: Dim,
    C1: Dim,
    R2: Dim,
    N: Scalar + Zero + One,
    SB: Storage<N, R2, C1>,
    SA: ContiguousStorageMut<N, R1, C1> + Clone,
    DefaultAllocator: Allocator<N, R1, C1, Buffer = SA>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Matrix<N, R2, C1, SB>) {
        loop {}
    }
}
