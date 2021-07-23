/*! x86_64 I/O port */

use core::marker::PhantomData;

use crate::arch::x86_64::dev::io_port::impls::THwIOPort;

pub mod impls;

/**
 * Wrapper for an x86 I/O port which allows reading and writing
 */
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IOPort<T>
    where T: THwIOPort {
    m_port: u16,
    _unused: PhantomData<T>
}

impl<T> IOPort<T> where T: THwIOPort {
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
