use std::mem;
use std::ops::Deref;

#[repr(transparent)]
#[derive(Eq, PartialEq, Clone, Hash, Copy)]
pub(crate) struct Unit<T> {
    value: T,
}

impl<T> Deref for Unit<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { mem::transmute(self) }
    }
}
