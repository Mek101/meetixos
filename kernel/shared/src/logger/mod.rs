/*! Kernel land logger */

/* re-export log macros */
pub use log::{
    debug as log_debug,
    error as log_error,
    info as log_info,
    trace as log_trace,
    warn as log_warn,
    SetLoggerError
};

pub mod logger;
pub mod writers;
