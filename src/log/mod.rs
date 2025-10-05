mod log_err;
mod ok_log;
pub mod option_tap_log;
mod result_tap_log;

pub use log_err::LogErr;
pub use ok_log::OkLog;
pub use option_tap_log::OptionTapLog;
pub use result_tap_log::ResultTapLog;

pub use tracing::Level;
