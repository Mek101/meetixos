/*! Traits for accessing I/O ports */

pub mod u16;
pub mod u32;
pub mod u8;

/**
 * Interface on which `IOPort` relies to read/write from the port of the
 * generic type selected
 */
pub trait HwIOPort {
    /**
     * Reads a `Self` value from the given port.
     */
    unsafe fn io_port_read(port: u16) -> Self;

    /**
     * Writes a `Self` value to the given port
     */
    unsafe fn io_port_write(port: u16, value: Self);
}
