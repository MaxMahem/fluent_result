/// A trait that wraps a value in an `Option<Self>`.
pub trait IntoSome {
    /// Moves a value into an `Option<Self>`.
    fn into_some(self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self)
    }

    /// Wraps a borrowed value in an `Option<Self>`.
    fn as_some(&self) -> Option<&Self> {
        Some(self)
    }
}

impl<T> IntoSome for T {}
