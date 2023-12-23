use crate::base::storage::{Storage, StorageMut};
use crate::base::{Dim, Matrix, Scalar};

pub(crate) trait MatrixIndexMut<'a, N: Scalar, R: Dim, C: Dim, S: StorageMut<N, R, C>> {
    type OutputMut: 'a;

    #[doc(hidden)]
    unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<N, R, C, S>) -> Self::OutputMut;
}

impl<N: Scalar, R: Dim, C: Dim, S: Storage<N, R, C>> Matrix<N, R, C, S> {
    #[inline]
    pub(crate) unsafe fn get_unchecked_mut<'a, I>(&'a mut self, index: I) -> I::OutputMut
    where
        S: StorageMut<N, R, C>,
        I: MatrixIndexMut<'a, N, R, C, S>,
    {
        index.get_unchecked_mut(self)
    }
}

impl<'a, N, R, C, S> MatrixIndexMut<'a, N, R, C, S> for (usize, usize)
where
    N: Scalar,
    R: Dim,
    C: Dim,
    S: StorageMut<N, R, C>,
{
    type OutputMut = &'a mut N;

    #[doc(hidden)]
    #[inline(always)]
    unsafe fn get_unchecked_mut(self, matrix: &'a mut Matrix<N, R, C, S>) -> Self::OutputMut
    where
        S: StorageMut<N, R, C>,
    {
        let (row, col) = self;
        matrix.data.get_unchecked_mut(row, col)
    }
}
