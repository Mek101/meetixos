/*! Kernel land logger implementation */

use core::cell::UnsafeCell;

pub use log::{
    set_logger,
    LevelFilter,
    Log,
    Metadata,
    Record,
    SetLoggerError
};
use log::{
    set_max_level,
    Level
};

use crate::{
    info::args::CmdLineArgs,
    logger::writers::LoggerWriter
};

/* re-export log stuffs */
/**
 * Generic writer backed `Log` implementation
 */
pub struct Logger<W>
    where W: LoggerWriter {
    m_inner: Option<UnsafeCell<W>>
}

impl<W> Logger<W> where W: LoggerWriter {
    const ESC_RED: usize = 31;
    const ESC_GREEN: usize = 32;
    const ESC_YELLOW: usize = 33;
    const ESC_MAGENTA: usize = 35;
    const ESC_CYAN: usize = 36;
    const ESC_WHITE: usize = 37;

    /**
     * Constructs an uninitialized `Logger` which must be initialized with
     * `Logger::enable_as_global()`
     */
    pub const fn new_uninitialized() -> Self {
        Self { m_inner: None }
    }

    /**
     * Initializes the inner writer
     */
    pub fn init(&mut self) {
        self.m_inner = Some(UnsafeCell::new(W::new()));
    }

    /**
     * Sets `self` as global logger with `log::set_logger()`
     */
    pub fn enable_as_global(&'static self) -> Result<(), SetLoggerError> {
        assert!(self.m_inner.is_some());
        set_logger(self)
    }

    /**
     * Calls `Logger::set_max_logging_level` searching for the right cmdline
     * argument key into the given `CmdLineArgs`
     */
    pub fn set_max_logging_level_from_cmdline(&self,
                                              cmdline: &CmdLineArgs,
                                              fallback: LevelFilter) {
        self.set_max_logging_level(cmdline.value_by_key("-log-level", fallback));
    }

    /**
     * Sets the `log::LevelFilter` for the active instance
     */
    pub fn set_max_logging_level(&self, level_filter: LevelFilter) {
        set_max_level(level_filter);
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
            let color_escape = match record.level() {
                Level::Error => Self::ESC_RED,
                Level::Warn => Self::ESC_YELLOW,
                Level::Info => Self::ESC_GREEN,
                Level::Debug => Self::ESC_MAGENTA,
                Level::Trace => Self::ESC_WHITE
            };

            let writer = unsafe { &mut *inner.get() };
            write!(writer,
                   "[\x1b[0;{}m{: >5}\x1b[0m <> \x1b[0;{}m{: <25}\x1b[0m] \
                    \x1b[0;{}m{}\x1b[0m\n",
                   color_escape,
                   record.level(), /* human readable log-level */
                   Self::ESC_CYAN,
                   record.target(), /* path to the rust module relative to the Kernel */
                   color_escape,
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
