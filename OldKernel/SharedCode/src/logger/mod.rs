/*! OldKernel land logger */

/* re-export log macros */
pub use log::{
    debug,
    error,
    info,
    log,
    trace,
    warn,
    SetLoggerError
};

pub mod logger;
pub mod writers;
