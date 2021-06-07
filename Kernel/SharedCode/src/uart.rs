/*! Universal asynchronous receiver-transmitter */

use core::{
    fmt,
    sync::atomic::{
        AtomicBool,
        Ordering
    }
};

use crate::arch::uart::HwUart;

/**
 * Simple arch independent interface for UART writing.
 *
 * The object implements `fmt::Write`, so it is possible to use all the
 * formatting methods/macros
 */
pub struct Uart {
    m_inner: UartWriterInner<HwUart>
}

impl Uart {
    /**
     * Constructs an uninitialized `Uart` which must be initialized with
     * `Uart::init()`
     */
    pub fn new() -> Self {
        Self { m_inner: UartWriterInner::new() }
    }

    /**
     * Initializes the underling hardware to make it active and ready to
     * receive bytes to write
     */
    pub fn init(&mut self) -> bool {
        self.m_inner.init()
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.m_inner.write_str(s)
    }
}

/**
 * Common interface used by the `UartWriterInner` to use the hardware
 * implementations
 */
pub(crate) trait HwUartBase: fmt::Write {
    /**
     * Constructs an uninitialized `HwUartBase` based object
     */
    fn new() -> Self;

    /**
     * Initializes the underling hardware to make it active and ready to
     * receive bytes to write.
     *
     * The method is ensured by the upper encapsulating object that is
     * called only once per instance
     */
    fn init_hw(&mut self) -> bool;
}

/**
 * Encapsulates a possibly uninitialized `HwUartBase` based object and
 * manages atomically his initialization
 */
struct UartWriterInner<T>
    where T: HwUartBase {
    m_inited: AtomicBool,
    m_inner: T
}

impl<T> UartWriterInner<T> where T: HwUartBase {
    /**
     * Constructs an uninitialized `UartWriterInner`
     */
    pub fn new() -> Self {
        Self { m_inited: AtomicBool::new(false),
               m_inner: T::new() }
    }

    /**
     * Initializes the hardware
     */
    pub fn init(&mut self) -> bool {
        if !self.m_inited.load(Ordering::SeqCst) {
            if self.m_inner.init_hw() {
                self.m_inited.store(true, Ordering::SeqCst);
                true
            } else {
                false
            }
        } else {
            /* immediately return for already inited hardware */
            true
        }
    }

    /**
     * Dispatches to the underling UART hardware the given string.
     */
    pub fn write_str(&mut self, str: &str) -> fmt::Result {
        if self.m_inited.load(Ordering::SeqCst) {
            self.m_inner.write_str(str)
        } else {
            Err(fmt::Error)
        }
    }
}
