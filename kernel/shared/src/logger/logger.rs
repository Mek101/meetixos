/*! Kernel land logger implementation */

use core::cell::UnsafeCell;

/* re-export log stuffs */
pub use log::{
    set_logger,
    LevelFilter,
    Log,
    Metadata,
    Record,
    SetLoggerError
};

use log::set_max_level;

use crate::logger::writers::LoggerWriter;

/**
 * Generic writer backed `Log` implementation
 */
pub struct Logger<W>
    where W: LoggerWriter {
    m_inner: Option<UnsafeCell<W>>
}

impl<W> Logger<W> where W: LoggerWriter {
    /**
     * Constructs an uninitialized `Logger` which must be initialized with
     * `Logger::enable_as_global()`
     */
    pub const fn new_uninitialized() -> Self {
        Self { m_inner: None }
    }

    /**
     * Initializes the inner instance and sets `self` as global logger with
     * `log::set_logger()`
     */
    pub fn enable_as_global(&'static mut self) -> Result<(), SetLoggerError> {
        self.m_inner = Some(UnsafeCell::new(W::new()));
        set_logger(self)
    }

    /**
     * Sets the `log::LevelFilter` for the active instance
     */
    pub fn set_max_logging_level(&self, log_level: LevelFilter) {
        set_max_level(log_level);
    }

    /**
     * Returns the mutable reference to the inner writer
     */
    pub unsafe fn writer_mut(&self) -> Option<&mut W> {
        self.m_inner.as_ref().map(|writer_cell| &mut *writer_cell.get())
    }
}

impl<W> Log for Logger<W> where W: LoggerWriter {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if let Some(ref inner) = self.m_inner {
            let writer = unsafe { &mut *inner.get() };
            write!(writer,
                   "[{: >5} <> {: <20}] {}\n",
                   record.level(),  /* human readable log-level */
                   record.target(), /* path to the rust module relative to the kernel */
                   record.args()).unwrap();
        }
    }

    fn flush(&self) {
        /* the implementation manages by itself the buffering */
    }
}

unsafe impl<W> Sync for Logger<W> where W: LoggerWriter {
    /* Nothing to implement, just a marker */
}
