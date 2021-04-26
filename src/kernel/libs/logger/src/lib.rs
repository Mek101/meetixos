/*! # Kernel Land Logger
 *
 * Implements a thread-safe, generics-customizable [`Logger`] structure
 * which is managed under the hood by the [`log`] crate
 *
 * [`Logger`]: struct.Logger.html
 * [`log`]: https://docs.rs/log/0.4.14/log/index.html
 */

#![no_std]
#![feature(array_methods, const_fn)]

#[cfg(feature = "heap_buffering")]
extern crate alloc;

#[cfg(not(feature = "heap_buffering"))]
use core::marker::PhantomData;
#[cfg(feature = "heap_buffering")]
use core::{alloc::Layout, fmt::Error, slice};
use core::{fmt, fmt::Write, str};

#[cfg(feature = "heap_buffering")]
use alloc::alloc::{alloc_zeroed, dealloc, realloc};

/* re-export logging macros renamed with `log_` prefix */
pub use log::{debug, error, info, warn, LevelFilter};

use log::{set_logger, set_max_level, Log, Metadata, Record, SetLoggerError};
use sync::{Mutex, RawMutex};

/** # Logger Wrapper
 *
 * Implements a generics-customizable [`Log`] implementation which could
 * manage heap-allocated buffer and write to different kind of
 * [`LoggerWriter`]s
 *
 * [`Log`]: https://docs.rs/log/0.4.14/log/trait.Log.html
 * [`LoggerWriter`]: trait.LoggerWriter.html
 */
pub struct Logger<'a, W, L>
    where W: LoggerWriter,
          L: RawMutex + Send + Sync {
    m_inner: Mutex<L, Option<LoggerInner<'a, W>>>
}

impl<'a, W, L> Logger<'a, W, L>
    where W: LoggerWriter,
          L: RawMutex + Send + Sync
{
    /** # Constructs an uninitialized `Logger`
     *
     * The returned instance must be initialized with
     * [`Logger::enable_as_global()`]
     *
     * [`Logger::enable_as_global()`]:
     * struct.Logger.html#method.enable_as_global
     */
    pub const fn new_uninitialized() -> Self {
        Self { m_inner: Mutex::new(None) }
    }

    /** # Enables this as global logger
     *
     * Initializes the inner instance and sets `self` as global logger with
     * [`log::set_logger()`]
     *
     * [`log::set_logger()`]: https://docs.rs/log/0.4.14/log/fn.set_logger.html
     */
    pub fn enable_as_global(&'static mut self) -> Result<(), SetLoggerError> {
        self.m_inner = Mutex::new(Some(LoggerInner::<W>::new()));
        set_logger(self)
    }

    /** # Enables logger's line-buffering
     *
     * Allocates an heap buffer of the given size, or
     * extends/shrinks/re-uses the already existing buffer
     */
    #[cfg(feature = "heap_buffering")]
    pub fn enable_buffering(&self, buffer_size: usize) -> bool {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            /* obtain the pointer to the buffer to use */
            let (buffer, buffer_size) = if let Some(ref mut buffer) = inner.m_buffer {
                /* well, the inner instance holds a valid buffer, so check whether the
                 * given buffer size is 0 or matches the current buffer size; in this
                 * case return the current buffer's pointer, otherwise
                 * re-allocate it
                 */
                if buffer_size == 0 || buffer_size == buffer.len() {
                    (buffer.as_mut_ptr(), buffer.len())
                } else {
                    /* re-allocate the buffer if the size doesn't match and is non-zero */
                    let new_buf_ptr = unsafe {
                        realloc(buffer.as_mut_ptr(),
                                Layout::for_value(&buffer),
                                buffer_size)
                    };
                    (new_buf_ptr, buffer_size)
                }
            } else {
                /* allocate a clean buffer from the heap */
                let new_buf_ptr = unsafe {
                    alloc_zeroed(Layout::from_size_align_unchecked(buffer_size, 1))
                };
                (new_buf_ptr, buffer_size)
            };

            /* enable the inner's buffering */
            if !buffer.is_null() {
                inner.enable_buffering(unsafe {
                         slice::from_raw_parts_mut(buffer, buffer_size)
                     });
                true
            } else {
                false
            }
        } else {
            panic!("Enabling buffering for a NON-initialized Logger");
        }
    }

    /** # Disables logger's line-buffering
     *
     * Disables the buffering if is active and de-allocates the buffer if
     * `keep_buffer` is `false`.
     *
     * If the buffer is kept, following calls to
     * [`Logger::enable_buffering()`] will re-use the existing buffer or
     * simply re-allocates it
     *
     * [`Logger::enable_buffering()`]:
     * struct.Logger.html#method.enable_buffering
     */
    #[cfg(feature = "heap_buffering")]
    pub fn disable_buffering(&self, keep_buffer: bool) {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            if inner.is_buffered() {
                inner.disable_buffering(keep_buffer);
            }
        } else {
            panic!("Disabling buffering for a NON-initialized Logger");
        }
    }

    /** Sets the [`log::LevelFilter`] for the active instance
     *
     * [`log::LevelFilter`]: https://docs.rs/log/0.4.14/log/enum.LevelFilter.html
     */
    pub fn set_max_logging_level(&'static self, log_level: LevelFilter) {
        set_max_level(log_level);
    }
}

impl<'a, W, L> Log for Logger<'a, W, L>
    where W: LoggerWriter,
          L: RawMutex + Send + Sync
{
    /** Determines if a log message with the specified metadata would be
     * logged
     */
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    /** Logs the [`Record`]
     *
     * [`Record`]: https://docs.rs/log/0.4.14/log/struct.Record.html
     */
    fn log(&self, record: &Record) {
        if let Some(ref mut inner) = *self.m_inner.lock() {
            write!(inner,
                   "[{: >5} <> {: <20}] {}\n",
                   record.level(),  /* human readable log-level */
                   record.target(), /* path to the rust module relative to the kernel */
                   record.args()).unwrap();
        }
    }

    /** Flushes any buffered records
     */
    fn flush(&self) {
        #[cfg(feature = "heap_buffering")]
        if let Some(ref mut inner) = *self.m_inner.lock() {
            inner.flush()
        }
    }
}

/** # Logger Writer Base Interface
 *
 * Defines the methods and the markers that each backend writer must
 * implement.
 *
 * This trait is used by the [`Logger`] to communicate with the real logger
 * storage/hardware (a serial output, the video, or a file)
 *
 * [`Logger`]: struct.Logger.html
 */
pub trait LoggerWriter: Write + Send + Sync {
    /** Constructs an initialized `LoggerWriter`
     */
    fn new() -> Self;
}

/** # Inner Logger Implementation
 *
 * Implements the middleware between the public [`Logger`] and the backend
 * [`LoggerWriter`].
 *
 * It Manages the line-buffering
 */
struct LoggerInner<'a, W>
    where W: LoggerWriter {
    m_writer: W,
    #[cfg(feature = "heap_buffering")]
    m_buffer: Option<&'a mut [u8]>,
    #[cfg(feature = "heap_buffering")]
    m_buffered: bool,
    #[cfg(feature = "heap_buffering")]
    m_buf_pos: usize,
    #[cfg(not(feature = "heap_buffering"))]
    _unused: PhantomData<&'a mut [u8]>
}

impl<'a, W> LoggerInner<'a, W> where W: LoggerWriter {
    /** # Constructs a `LoggerInner`
     *
     * The returned instance is not buffered
     */
    fn new() -> Self {
        Self { m_writer: W::new(),
               #[cfg(feature = "heap_buffering")]
               m_buffer: None,
               #[cfg(feature = "heap_buffering")]
               m_buffered: false,
               #[cfg(feature = "heap_buffering")]
               m_buf_pos: 0,
               #[cfg(not(feature = "heap_buffering"))]
               _unused: PhantomData }
    }

    /** # Enables the line-buffering
     *
     * Stores the given buffer reference and enables the buffered flag
     */
    #[cfg(feature = "heap_buffering")]
    fn enable_buffering(&mut self, buffer: &'a mut [u8]) {
        self.m_buffer = Some(buffer);
        self.m_buffered = true
    }

    /** # Disables the line-buffering
     *
     * Disables the buffered flags and, if `keep_buffer` is false
     * de-allocates the heap buffer
     */
    #[cfg(feature = "heap_buffering")]
    fn disable_buffering(&mut self, keep_buffer: bool) {
        self.m_buffered = false;
        if !keep_buffer {
            if let Some(ref mut buffer) = self.m_buffer {
                unsafe { dealloc(buffer.as_mut_ptr(), Layout::for_value(&buffer)) }
                self.m_buffer = None;
            }
        }
    }

    /** # Stores the given string slice into the buffer
     *
     * Flushes the buffer when encounters the newline character `'\n'` or
     * when the buffer is not empty/big enough
     */
    #[cfg(feature = "heap_buffering")]
    fn write_in_buffer(&mut self, s: &str) -> fmt::Result {
        if let Some(ref buffer) = self.m_buffer {
            /* check whether the remaining buffer if not enough to store the given
             * slice but is enough when empty, in this case flush the buffer and
             * continue.
             * Otherwise write unbuffered
             */
            if buffer.len() - self.m_buf_pos < s.len() && s.len() < buffer.len() {
                self.flush();
            } else if buffer.len() < s.len() {
                /* flush anyway, just to be sure */
                self.flush();
                return self.m_writer.write_str(s);
            }
        }

        if let Some(ref mut buffer) = self.m_buffer {
            /* store each byte into the buffer */
            for byte in s.as_bytes().iter() {
                /* store the current byte into the buffer */
                let byte = *byte;
                buffer[self.m_buf_pos] = byte;
                self.m_buf_pos += 1;

                /* flush if found ascii newline */
                if byte == b'\n' {
                    /* TODO here could be called self.flush(), but the compiler throws
                     *      an error due to double mutable borrow
                     */
                    let _ =
                        self.m_writer
                            .write_str(unsafe {
                                str::from_utf8_unchecked(&buffer[..self.m_buf_pos])
                            });
                    self.m_buf_pos = 0;
                }
            }
            Ok(())
        } else {
            Err(Error)
        }
    }

    /** # Flushes the buffer
     *
     * Empty the buffer writing each byte to the underling [`LoggerWriter`]
     *
     * [`LoggerWriter`]: trait.LoggerWriter.html
     */
    #[cfg(feature = "heap_buffering")]
    fn flush(&mut self) {
        if self.is_buffered() && self.m_buf_pos > 0 {
            if let Some(ref buffer) = self.m_buffer {
                let _ = self.m_writer
                            .write_str(unsafe {
                                str::from_utf8_unchecked(&buffer[..self.m_buf_pos])
                            });
                self.m_buf_pos = 0;
            }
        }
    }

    /** Returns whether the `LoggerInner` uses buffering
     */
    #[cfg(feature = "heap_buffering")]
    fn is_buffered(&self) -> bool {
        self.m_buffered
    }
}

#[cfg(feature = "heap_buffering")]
impl<'a, W> Write for LoggerInner<'a, W> where W: LoggerWriter {
    /** Writes a string slice into this writer, returning whether the write
     * succeeded
     */
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.is_buffered() {
            self.write_in_buffer(s)
        } else {
            self.m_writer.write_str(s)
        }
    }
}

#[cfg(not(feature = "heap_buffering"))]
impl<'a, W> Write for LoggerInner<'a, W> where W: LoggerWriter {
    /** Writes a string slice into this writer, returning whether the write
     * succeeded
     */
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.m_writer.write_str(s)
    }
}
