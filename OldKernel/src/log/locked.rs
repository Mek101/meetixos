/*! Locked logger implementation */

use shared::{
    info::args::CmdLineArgs,
    logger::{
        logger::{
            set_logger,
            LevelFilter,
            Log,
            Logger,
            Metadata,
            Record
        },
        writers::LoggerWriter,
        SetLoggerError
    }
};
use sync::mutex::{
    ConstCreatBackRawMutex,
    Mutex
};

use crate::log::writer::BufferedWriter;

/**
 * Buffered `LoggerWriter` implementation.
 *
 * Relies on another `LoggerWriter` to flush the buffer
 */
pub(super) struct LockedBufferedLogger<L, W>
    where L: ConstCreatBackRawMutex,
          W: LoggerWriter {
    m_inner: Mutex<L, Logger<BufferedWriter<W>>>
}

impl<L, W> LockedBufferedLogger<L, W>
    where L: ConstCreatBackRawMutex,
          W: LoggerWriter
{
    /**
     * Constructs an uninitialized `LockedBufferedLogger`
     */
    pub const fn new_uninitialized() -> Self {
        Self { m_inner: Mutex::const_new(Logger::new_uninitialized()) }
    }

    /**
     * Initializes the inner writer
     */
    pub fn init(&mut self) {
        self.m_inner.lock().init();
    }

    /**
     * Sets `self` as global logger with `log::set_logger()`
     */
    pub fn enable_as_global(&'static self) -> Result<(), SetLoggerError> {
        set_logger(self)
    }

    /**
     * Calls `Logger::set_max_logging_level` searching for the right cmdline
     * argument key into the given `CmdLineArgs`
     */
    pub fn set_max_logging_level_from_cmdline(&self,
                                              cmdline: &CmdLineArgs,
                                              fallback: LevelFilter) {
        self.m_inner.lock().set_max_logging_level_from_cmdline(cmdline, fallback);
    }

    /*
     * Sets the `log::LevelFilter` for the active instance
     */
    //pub fn set_max_logging_level(&self, log_level: LevelFilter) {
    //    self.m_inner.lock().set_max_logging_level(log_level)
    //}

    /**
     * Enables the line-buffering
     */
    pub fn enable_buffering(&self, use_previous_buffer_if_any: bool, buffer_size: usize) {
        unsafe {
            if let Some(writer) = self.m_inner.lock().writer_mut() {
                writer.enable_buffering(use_previous_buffer_if_any, buffer_size)
            }
        }
    }

    /*
     * Disables the line-buffering
     */
    //pub fn disable_buffering(&self, keep_buffer: bool) {
    //   unsafe {
    //        if let Some(writer) = self.m_inner.lock().writer_mut() {
    //            writer.disable_buffering(keep_buffer)
    //        }
    //    }
    //}
}

impl<L, W> Log for LockedBufferedLogger<L, W>
    where L: ConstCreatBackRawMutex,
          W: LoggerWriter
{
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.m_inner.lock().enabled(metadata)
    }

    fn log(&self, record: &Record) {
        self.m_inner.lock().log(record);
    }

    fn flush(&self) {
        /* the implementation manages by itself the buffering */
    }
}

unsafe impl<L, W> Send for LockedBufferedLogger<L, W>
    where L: ConstCreatBackRawMutex,
          W: LoggerWriter
{
    /* Nothing to implement, just a marker */
}

unsafe impl<L, W> Sync for LockedBufferedLogger<L, W>
    where L: ConstCreatBackRawMutex,
          W: LoggerWriter
{
    /* Nothing to implement, just a marker */
}
