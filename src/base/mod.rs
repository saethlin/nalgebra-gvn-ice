mod alias;
pub(crate) mod allocator;
mod array_storage;
mod construction;
pub(crate) mod dimension;
mod indexing;
mod matrix;
mod ops;
mod scalar;
pub(crate) mod storage;
mod unit;

pub(crate) use self::alias::*;
pub(crate) use self::allocator::DefaultAllocator;
use self::dimension::*;
pub(crate) use self::matrix::*;
pub(crate) use self::scalar::*;
pub(crate) use self::unit::*;
