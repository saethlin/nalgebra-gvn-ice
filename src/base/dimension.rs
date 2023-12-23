use std::any::Any;
use typenum::{self, Bit, UInt, Unsigned};

pub(crate) trait IsNotStaticOne {}

pub(crate) trait Dim: Any + Copy + PartialEq + Send + Sync {
    fn try_to_usize() -> Option<usize>;
    fn value(&self) -> usize;
}

pub(crate) trait DimName: Dim {
    type Value: NamedDim<Name = Self>;

    fn name() -> Self;
}

pub(crate) trait NamedDim: Sized + Any + Unsigned {
    type Name: DimName<Value = Self>;
}
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) struct U1;

impl Dim for U1 {
    #[inline]
    fn try_to_usize() -> Option<usize> {
        loop {}
    }

    #[inline]
    fn value(&self) -> usize {
        loop {}
    }
}

impl DimName for U1 {
    type Value = typenum::U1;

    #[inline]
    fn name() -> Self {
        loop {}
    }
}

impl NamedDim for typenum::U1 {
    type Name = U1;
}

macro_rules! named_dimension(($($D: ident), * $(,) *) => {
    $(#[derive(Copy, Clone, Hash, PartialEq, Eq)] pub struct $D; impl Dim for $D {
        #[inline] fn try_to_usize() -> Option < usize > {
            Some(typenum:: $D:: to_usize())
        }
        #[inline] fn value(&self) -> usize {
            typenum:: $D:: to_usize()
        }
    } impl DimName for $D {
        type Value = typenum:: $D;
        #[inline] fn name() -> Self {
            $D
        }
    } impl NamedDim for typenum:: $D {
        type Name = $D;
    } impl IsNotStaticOne for $D {
    }) *
});

named_dimension!(U2);

impl<U: Unsigned + DimName, B: Bit + Any + Copy + PartialEq + Send + Sync> NamedDim for UInt<U, B> {
    type Name = UInt<U, B>;
}

impl<U: Unsigned + DimName, B: Bit + Any + Copy + PartialEq + Send + Sync> Dim for UInt<U, B> {
    #[inline]
    fn try_to_usize() -> Option<usize> {
        loop {}
    }

    #[inline]
    fn value(&self) -> usize {
        loop {}
    }
}

impl<U: Unsigned + DimName, B: Bit + Any + Copy + PartialEq + Send + Sync> DimName for UInt<U, B> {
    type Value = UInt<U, B>;

    #[inline]
    fn name() -> Self {
        loop {}
    }
}
