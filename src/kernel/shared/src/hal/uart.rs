/*! # Universal Asynchronous Receiver-Transmitter
 *
 * Implements an architecture independent interface to use the UART of the
 * underling hardware without worrying about technical specification
 */

use core::{
    fmt,
    sync::atomic::{AtomicBool, Ordering}
};

use crate::arch::uart::HwUart;

/** # UART Writer
 *
 * Implements a simple arch independent interface for UART communication
 * (only in write direction).
 *
 * The object implements [`fmt::Write`], so it is possible to use all the
 * formatting methods/macros
 *
 * [`fmt::Write`]: https://doc.rust-lang.org/std/fmt/trait.Write.html
 */
pub struct Uart {
    m_inner: UartWriterInner<HwUart>
}

impl Uart {
    /** # Constructs an uninitialized `Uart`
     *
     * The returned instance hasn't initialized the hardware yet, use
     * [`Uart::init()`] to do that
     *
     * [`Uart::init()`]: /hal/uart/struct.Uart.html#method.init
     */
    pub fn new() -> Self {
        Self { m_inner: UartWriterInner::new() }
    }

    /** # Initializes the hardware
     *
     * Initializes the underling hardware to make it active and ready to
     * receive writing bytes
     */
    pub fn init(&mut self) -> bool {
        self.m_inner.init()
    }
}

impl fmt::Write for Uart {
    /** Writes a string slice into this writer, returning whether the write
     * succeeded
     */
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.m_inner.write_str(s)
    }
}

/** # UART Base Interface
 *
 * Defines a little amount of methods that are required by the
 * [`UartWriterInner`]
 *
 * [`UartWriterInner`]: /hal/uart/struct.UartWriterInner.html
 */
pub(crate) trait HwUartBase: fmt::Write {
    /** # Constructs an `UartBase` based object
     *
     * The returned instance should not be initialized (i.e the underling
     * hardware should not be initialized)
     */
    fn new() -> Self;

    /** # Initializes the hardware
     *
     * Initializes the underling hardware to make it active and ready to
     * receive writing bytes.
     *
     * The method is ensured by the upper encapsulating object that is
     * called only once per instance
     */
    fn init_hw(&mut self) -> bool;
}

/** # UART Writer Inner
 *
 * Encapsulates a possibly uninitialized [`UartBase`] based object and
 * manages atomically his initialization
 *
 * [`UartBase`]: /hal/uart/trait.UartBase.html
 */
struct UartWriterInner<T>
    where T: HwUartBase {
    m_inited: AtomicBool,
    m_inner: T
}

impl<T> UartWriterInner<T> where T: HwUartBase {
    /** # Constructs an uninitialized `UartWriterInner`
     *
     * The returned instance is not initialized and not ready for use, use
     * the [`UartWriterInner::init()`] before any usage
     *
     * [`UartWriterInner::init()`]:
     * /hal/uart/struct.UartWriterInner.html#method.init
     */
    pub fn new() -> Self {
        Self { m_inited: AtomicBool::new(false),
               m_inner: T::new() }
    }

    /** # Initializes the hardware
     *
     * Initializes the underling hardware to make it active and ready to
     * receive writing bytes.
     *
     * If the hardware successfully initializes itself the object sets
     * itself as ready to write data
     */
    pub fn init(&mut self) -> bool {
        if !self.m_inited.load(Ordering::SeqCst) {
            let res = self.m_inner.init_hw();
            if res {
                self.m_inited.store(true, Ordering::SeqCst);
            }
            res
        } else {
            /* if the hardware is already initialized return true without
             * any computation
             */
            true
        }
    }

    /** # Writes the given string
     *
     * Writes to the underling UART hardware the given string.
     *
     * An [`fmt::Error`] is returned if the hardware is not initialized
     *
     * [`fmt::Error`]: https://doc.rust-lang.org/std/fmt/struct.Error.html
     */
    pub fn write_str(&mut self, str: &str) -> fmt::Result {
        if self.m_inited.load(Ordering::SeqCst) {
            self.m_inner.write_str(str)
        } else {
            Err(fmt::Error)
        }
    }
}
