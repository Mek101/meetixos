/*! # Kernel Debugging Utils
 *
 * Implements dump & debug utility functions
 */

use core::fmt;

use hal::boot::infos::BootInfos;

use crate::log::debug;

pub const GIB: usize = 1024 * 1024 * 1024;
pub const MIB: usize = 1024 * 1024;
pub const KIB: usize = 1024;

/** # Dumps `BootInfos`
 *
 * Prints using the `log` module the [`BootInfos`] given by the [`HAL`]
 *
 * [`BootInfos`]: /hal/boot/struct.BootInfos.html
 * [`HAL`]: /hal/
 */
pub fn dump_boot_infos() {
    debug!("BootInfos:");
    debug!("\tPhys Mem Offset: {:?}", BootInfos::obtain().hw_phys_mem_offset());
    dump_boot_cmdline();
    dump_boot_mem_areas();
}

/** # Dumps boot command line
 *
 * Prints using the `log` module the command line inside the [`BootInfos`]
 * given by the [`HAL`]
 *
 * [`BootInfos`]: /hal/boot/struct.BootInfos.html
 * [`HAL`]: /hal/
 */
pub fn dump_boot_cmdline() {
    debug!("\tCommand Line:");
    for cmd_arg in BootInfos::obtain().cmdline_args().iter() {
        debug!("\t\t{}", cmd_arg.as_str());
    }
}

/** # Dumps boot memory areas
 *
 * Prints using the `log` module the memory areas inside the [`BootInfos`]
 * given by the [`HAL`]
 *
 * [`BootInfos`]: /hal/boot/struct.BootInfos.html
 * [`HAL`]: /hal/
 */
pub fn dump_boot_mem_areas() {
    debug!("\tMemory Areas:");
    for mma in BootInfos::obtain().mem_areas().iter() {
        debug!("\t\tBootMemArea {{ {:?}, {} }}",
               mma.start_phys_addr(),
               debug_size_multiplier(mma.size()));
    }
}

/** Returns a [`fmt::Display`] implementation to print in a pretty way the
 * `size_value` given
 *
 * [`fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
 */
pub fn debug_size_multiplier(size_value: usize) -> impl fmt::Display {
    DebugSizeMul::new(size_value)
}

/** # Debug Size Multiplier
 *
 * Internal debug struct used by the [`debug_size_multiplier()`] to
 * implement [`fmt::Display`]
 *
 * [`debug_size_multiplier()`]: /kernel/debug/fn.debug_size_multiplier.html
 * [`fmt::Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
 */
struct DebugSizeMul {
    m_value: usize,
    m_decimals: usize,
    m_multiplier: &'static str
}

impl DebugSizeMul {
    pub fn new(value: usize) -> Self {
        let (value, dec, mul) = if value >= GIB {
            (value / GIB, value % GIB, "GiB")
        } else if value >= MIB {
            (value / MIB, value % MIB, "MiB")
        } else if value >= KIB {
            (value / KIB, value % KIB, "KiB")
        } else {
            (value, 0, "Bytes")
        };

        Self { m_value: value,
               m_decimals: dec,
               m_multiplier: mul }
    }
}

impl fmt::Display for DebugSizeMul {
    /** Formats the value using the given formatter
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.m_decimals > 0 {
            write!(f, "{}.{}{}", self.m_value, self.m_decimals, self.m_multiplier)
        } else {
            write!(f, "{}{}", self.m_value, self.m_multiplier)
        }
    }
}
