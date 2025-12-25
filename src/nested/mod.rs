#[cfg(feature = "alloc")]
mod box_err;
mod flatten_err;

#[cfg(feature = "alloc")]
pub use box_err::BoxErr;
pub use flatten_err::{FlattenErr, NestedError};
