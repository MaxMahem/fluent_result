pub mod into_err;
pub mod into_ok;
pub mod into_ok_or_panic;
pub mod into_some;
pub mod map_to;
pub mod sink_err;
pub mod unwrap_never;

#[cfg(feature = "tracing")]
pub mod log_err;

pub use into_err::IntoErr;
pub use into_ok::IntoOk;
pub use into_ok_or_panic::IntoOkOrPanic;
pub use into_some::IntoSome;
pub use map_to::MapTo;
pub use sink_err::SinkErr;
pub use unwrap_never::UnwrapNever;

#[cfg(feature = "tracing")]
pub use log_err::LogErr;

mod internal {
    pub trait Sealed {}

    //impl<T> Sealed for Result<T, std::convert::Infallible> {}
    impl<T, E> Sealed for Result<T, E> {}
}
