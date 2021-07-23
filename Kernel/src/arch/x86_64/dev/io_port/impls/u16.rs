/*! HwIOPortRead & HwIOPortWrite implementation for u16 */

use crate::arch::x86_64::dev::io_port::impls::THwIOPort;

impl THwIOPort for u16 {
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
