/*! HwIOPortRead & HwIOPortWrite implementation for u32 */

use crate::arch::x86_64::io_port::impls::HwIOPort;

impl HwIOPort for u32 {
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
