use crate::base::dimension::U2;
use crate::base::storage::Owned;
use crate::base::Matrix;

pub(crate) type MatrixMN<N, R, C> = Matrix<N, R, C, Owned<N, R, C>>;
pub(crate) type MatrixN<N, D> = MatrixMN<N, D, D>;
pub(crate) type Matrix2<N> = MatrixN<N, U2>;
