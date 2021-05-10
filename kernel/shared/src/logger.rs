/*! Kernel land logger */

#[cfg(not(feature = "loader_stage"))]
extern crate alloc;

#[cfg(not(feature = "loader_stage"))]
use core::fmt::Error;
use core::{
    fmt,
    fmt::Write,
    str
};

#[cfg(not(feature = "loader_stage"))]
use alloc::vec::Vec;

/* re-export logging macros */
pub use log::{
    debug,
    error,
    info,
    warn,
    LevelFilter
};

use log::{
    set_logger,
    set_max_level,
    Log,
    Metadata,
    Record,
    SetLoggerError
};
use sync::{
    Mutex,
    RawMutex
};

/**
 * Generics-customizable `Log` implementation which could manage
 * heap-allocated buffer and write to different kind of `LoggerWriter`s
 */
pub struct Logger<W, L>
    where W: LoggerWriter,
          L: RawMutex + Send + Sync {
    m_inner: Mutex<L, Option<LoggerInner<W>>>
}

impl<W, L> Logger<W, L>
    where W: LoggerWriter,
          L: RawMutex + Send + Sync
{
    /**
     * Constructs an uninitialized `Logger` which must be initialized with
     * `Logger::enable_as_global()`
     */
    pub const fn new_uninitialized() -> Self {
        Self { m_inner: Mutex::new(None) }
    }

    /**
     * Initializes the inner instance and sets `self` as global logger with
     * `log::set_logger()`
     */
    pub fn enable_as_global(&'static mut self) -> Result<(), SetLoggerError> {
        self.m_inner = Mutex::new(Some(LoggerInner::<W>::new()));
        set_logger(self)
    }

    /**
     * Enables the line-buffering for this logger re-using the previously
     * kept buffer or allocating a new one
     */
    #[cfg(not(feature = "loader_stage"))]
    pub fn enable_buffering(&self) {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            inner.enable_buffering();
        } else {
            panic!("Enabling buffering for a NON-initialized Logger");
        }
    }

    /**
     * Disables the buffering if is active and de-allocates the buffer if
     * `keep_buffer` is `false`.
     *
     * If the buffer is kept, following calls to
     * `Logger::enable_buffering()` will re-use the existing buffer or
     * simply re-allocates it
     */
    #[cfg(not(feature = "loader_stage"))]
    pub fn disable_buffering(&self, keep_buffer: bool) {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            if inner.is_buffered() {
                inner.disable_buffering(keep_buffer);
            }
        } else {
            panic!("Disabling buffering for a NON-initialized Logger");
        }
    }

    /**
     * Sets the `log::LevelFilter` for the active instance
     */
    pub fn set_max_logging_level(&'static self, log_level: LevelFilter) {
        set_max_level(log_level);
    }
}

impl<W, L> Log for Logger<W, L>
    where W: LoggerWriter,
          L: RawMutex + Send + Sync
{
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            write!(inner,
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

/**
 * Defines the methods and the markers that each backend writer must
 * implement.
 *
 * This trait is used by the `Logger` to communicate with the real logger
 * storage/hardware (a serial output, the video, or a file)
 */
pub trait LoggerWriter: Write + Send + Sync {
    /**
     * Constructs an initialized `LoggerWriter`
     */
    fn new() -> Self;
}

/**
 * Implements the middleware between the public `Logger` and the backend
 * `LoggerWriter`.
 *
 * It Manages the line-buffering when enabled and available
 */
struct LoggerInner<W>
    where W: LoggerWriter {
    #[cfg(not(feature = "loader_stage"))]
    m_buffer: Option<LoggerBuffer>,
    #[cfg(not(feature = "loader_stage"))]
    m_buffered: bool,
    m_writer: W
}

impl<W> LoggerInner<W> where W: LoggerWriter {
    /**  
     * Constructs an unbuffered `LoggerInner`
     */
    fn new() -> Self {
        Self { #[cfg(not(feature = "loader_stage"))]
               m_buffer: None,
               #[cfg(not(feature = "loader_stage"))]
               m_buffered: false,
               m_writer: W::new() }
    }

    /**
     * Allocates a buffer of `LoggerBuffer::SIZE`.
     *
     * Further write to the logger inner will buffer the pieces until the
     * `\n` character
     */
    #[cfg(not(feature = "loader_stage"))]
    fn enable_buffering(&mut self) {
        if self.m_buffer.is_none() {
            self.m_buffer = Some(LoggerBuffer::new());
        }
        self.m_buffered = true;
    }

    /**
     * Disables the buffered flags and, if `keep_buffer` is false
     * de-allocates the heap buffer
     */
    #[cfg(not(feature = "loader_stage"))]
    fn disable_buffering(&mut self, keep_buffer: bool) {
        self.m_buffered = false;
        if !keep_buffer {
            /* overwriting with <None> the field throws the <drop> call for it */
            self.m_buffer = None;
        }
    }

    /**
     * Flushes the buffer when encounters the newline character `'\n'` or
     * when the buffer is not empty/big enough
     */
    #[cfg(not(feature = "loader_stage"))]
    fn write_in_buffer(&mut self, s: &str) -> fmt::Result {
        let writer = &mut self.m_writer;

        /* write the given chunk to the buffer and flush to the writer if needed */
        if let Some(ref mut buffer) = self.m_buffer {
            buffer.write_str_chunk(s, |buffer| {
                      if let Ok(utf8_str) = str::from_utf8(buffer) {
                          writer.write_str(utf8_str)
                      } else {
                          Err(Error)
                      }
                  })
        } else {
            Err(Error)
        }
    }

    /**
     * Returns whether the `LoggerInner` uses buffering
     */
    #[cfg(not(feature = "loader_stage"))]
    fn is_buffered(&self) -> bool {
        self.m_buffered
    }
}

#[cfg(not(feature = "loader_stage"))]
impl<W> Write for LoggerInner<W> where W: LoggerWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.is_buffered() {
            self.write_in_buffer(s)
        } else {
            self.m_writer.write_str(s)
        }
    }
}

#[cfg(feature = "loader_stage")]
impl<W> Write for LoggerInner<W> where W: LoggerWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.m_writer.write_str(s)
    }
}

/**
 * Manages a byte buffer using a `Vec<u8>` with initial capacity of
 * `LoggerBuffer::SIZE`.
 *
 * The object stores buffer until catches the ASCII new-line `\n`
 */
#[cfg(not(feature = "loader_stage"))]
struct LoggerBuffer {
    m_buffer: Vec<u8>
}

#[cfg(not(feature = "loader_stage"))]
impl LoggerBuffer {
    /**
     * Initial size of the buffer in bytes
     */
    const INITIAL_SIZE: usize = 512;

    /**
     * Constructs a new `LoggerBuffer` of `LoggerBuffer::SIZE` bytes
     */
    fn new() -> Self {
        Self { m_buffer: Vec::with_capacity(Self::INITIAL_SIZE) }
    }

    /**
     * Fills the buffer with the given `str_chunk` and calls
     * `flush_callback` when encounters ASCII `\n`
     */
    fn write_str_chunk<F>(&mut self,
                          str_chunk: &str,
                          mut flush_callback: F)
                          -> fmt::Result
        where F: FnMut(&[u8]) -> fmt::Result {
        /* extend the buffer if the remaining capacity doesn't suffice */
        if !self.can_store(str_chunk) {
            self.m_buffer.reserve(str_chunk.len());
        }

        /* iterate each byte into the given string and put it into the buffer */
        for byte in str_chunk.as_bytes() {
            let byte = *byte;
            self.m_buffer.push(byte);

            /* catch ASCII new-line to flush the buffer */
            if byte == b'\n' {
                /* call the callback given to flush the buffer */
                if let Err(err) = flush_callback(self.m_buffer.as_slice()) {
                    return Err(err);
                } else {
                    /* reset the buffer */
                    self.m_buffer.truncate(0);
                }
            }
        }
        Ok(())
    }

    /**
     * Returns whether the buffer can hold `s` without re-allocations
     */
    fn can_store(&self, s: &str) -> bool {
        self.capacity() - self.m_buffer.len() < s.len()
    }

    /**
     * Returns the capacity of the buffer
     */
    fn capacity(&self) -> usize {
        self.m_buffer.capacity()
    }
}
