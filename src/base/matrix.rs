use crate::base::dimension::Dim;
use crate::base::storage::Storage;
use crate::base::Scalar;
use std::marker::PhantomData;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct Matrix<N: Scalar, R: Dim, C: Dim, S> {
    pub(crate) data: S,
    _phantoms: PhantomData<(N, R, C)>,
}

impl<N: Scalar, R: Dim, C: Dim, S> Matrix<N, R, C, S> {
    #[inline]
    pub(crate) unsafe fn from_data_statically_unchecked(data: S) -> Matrix<N, R, C, S> {
        Matrix {
            data: data,
            _phantoms: PhantomData,
        }
    }
}

impl<N: Scalar, R: Dim, C: Dim, S: Storage<N, R, C>> Matrix<N, R, C, S> {
    #[inline]
    pub(crate) fn from_data(data: S) -> Self {
        unsafe { Self::from_data_statically_unchecked(data) }
    }

    #[inline]
    pub(crate) fn shape(&self) -> (usize, usize) {
        let (nrows, ncols) = self.data.shape();
        (nrows.value(), ncols.value())
    }
}
