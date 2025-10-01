pub trait Sealed {}

impl<T, E> Sealed for Result<T, E> {}

impl<T> Sealed for Option<T> {}
