/*! HwIOPortRead & HwIOPortWrite implementation for u8 */

use crate::arch::x86_64::dev::io_port::impls::THwIOPort;

impl THwIOPort for u8 {
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
