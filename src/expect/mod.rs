/// Assertions for [`Option`] values.
pub mod expect_none;
mod unwrap_never;

pub use expect_none::{dbg, rls};
pub use unwrap_never::UnwrapNever;
