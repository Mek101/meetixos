/*! x86_64 I/O port */

use core::marker::PhantomData;

/**
 * Wrapper for an x86 I/O port which allows reading and writing
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IoPort<T>
    where T: THwX64Port {
    m_port: u16,
    _unused: PhantomData<T>
}

impl<T> IoPort<T> where T: THwX64Port {
    /**
     * Constructs an `IOPort` which read and writes on the given `port`
     */
    pub const fn new(port: u16) -> Self {
        Self { m_port: port,
               _unused: PhantomData }
    }

    /**
     * Reads a value from the selected I/O port
     */
    #[inline]
    pub unsafe fn read(&self) -> T {
        T::io_port_read(self.m_port)
    }

    /**
     * Writes the given value to the selected I/O port
     */
    #[inline]
    pub unsafe fn write(&self, value: T) {
        T::io_port_write(self.m_port, value);
    }
}

impl THwX64Port for u8 {
    #[inline]
    unsafe fn io_port_read(port: u16) -> Self {
        let value: u8;
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
        value
    }

    #[inline]
    unsafe fn io_port_write(port: u16, value: Self) {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

impl THwX64Port for u16 {
    #[inline]
    unsafe fn io_port_read(port: u16) -> Self {
        let value: u16;
        asm!("in ax, dx", out("ax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        value
    }

    #[inline]
    unsafe fn io_port_write(port: u16, value: Self) {
        asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
    }
}

impl THwX64Port for u32 {
    #[inline]
    unsafe fn io_port_read(port: u16) -> Self {
        let value: u32;
        asm!("in eax, dx", out("eax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        value
    }

    #[inline]
    unsafe fn io_port_write(port: u16, value: Self) {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
    }
}

/**
 * Interface on which `IOPort` relies to read/write from the port of the
 * generic type selected
 */
pub trait THwX64Port {
    /**
     * Reads a `Self` value from the given port.
     */
    unsafe fn io_port_read(port: u16) -> Self;

    /**
     * Writes a `Self` value to the given port
     */
    unsafe fn io_port_write(port: u16, value: Self);
}
