pub(crate) trait Scalar: Copy + PartialEq + 'static {}

impl<T: Copy + PartialEq + 'static> Scalar for T {}
