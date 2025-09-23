pub mod into_err;
pub mod into_ok;
pub mod into_some;
pub mod ok_or_panic;
pub mod unwrap_never;

pub use into_err::IntoErr;
pub use into_ok::IntoOk;
pub use into_some::IntoSome;
pub use ok_or_panic::OkOrPanic;
pub use unwrap_never::UnwrapNever;
