/*! # HAL Boot Informations
 *
 * Implements the bootloader independent informations structure
 */

use crate::{
    addr::VirtAddr,
    arch::boot::HwBootInfos,
    boot::infos::{BootMemAreas, CmdLineArgs}
};

/** It is initialized by the [`bsp_entry`] entry point macro, to allow the
 * [`BootInfos`] to be copiable and freely accessible without any
 * re-initialization
 *
 * [`bsp_entry`]: /hal/boot/macro.bsp_entry.html
 * [`BootInfos`]: /hal/boot/struct.BootInfos.html
 */
static mut BOOT_INFOS_INNER: Option<BootInfosInner> = None;

/** # Common Bootloader Informations
 *
 * Stores the common bootloader's informations
 */
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BootInfos {
    m_inner: &'static BootInfosInner
}

impl BootInfos {
    /** # Constructs a `BootInfos`
     *
     * The returned instance is already filled and valid
     */
    pub fn obtain() -> Self {
        unsafe {
            assert!(BOOT_INFOS_INNER.is_some(),
                    "HAL haven't initialized boot informations");
            Self { m_inner: BOOT_INFOS_INNER.as_mut().unwrap() }
        }
    }

    /** Returns the physical address of the initial page directory
     * constructed
     */
    pub fn hw_phys_mem_offset(&self) -> VirtAddr {
        self.m_inner.m_hw_phys_mem_offset
    }

    /** Returns the slice to the kernel's command line
     */
    pub fn cmdline_args(&self) -> &'static CmdLineArgs {
        &self.m_inner.m_cmdline_args
    }

    /** Returns the [`BootMemAreas`] collection
     *
     * [`BootMemAreas`]: /hal/boot/infos/struct.BootMemAreas.html
     */
    pub fn mem_areas(&self) -> &'static BootMemAreas {
        &self.m_inner.m_mem_areas
    }
}

impl From<*const u8> for BootInfos {
    /** Initializes the global inner informations then constructs the
     * `BootInfos` instance.
     *
     * Used by the higher half loader to initialize his instance of the
     * `BootInfosInner`
     */
    fn from(raw_ptr: *const u8) -> Self {
        unsafe {
            assert!(BOOT_INFOS_INNER.is_none(), "Tried to re-initialize inner BootInfos");
        }

        /* obtain the informations inner and store to the global struct */
        let inner_infos = HwBootInfos::obtain_inner_from_arch_infos(raw_ptr);
        unsafe {
            BOOT_INFOS_INNER = Some(inner_infos);
        }

        /* return an instance of the wrapper */
        Self::obtain()
    }
}

impl From<&Self> for BootInfos {
    /** Initializes the global inner informations then constructs the
     * `BootInfos` instance.
     *
     * Used by the kernel core to clone the higher half loader's instance of
     * the `BootInfosInner` into the higher half instance
     */
    fn from(rhs: &BootInfos) -> Self {
        unsafe {
            assert!(BOOT_INFOS_INNER.is_none(), "Tried to re-initialize inner BootInfos");
        }

        /* clone the boot informations inner and store to our global copy */
        unsafe {
            BOOT_INFOS_INNER = Some(rhs.m_inner.clone());
        }

        /* return an instance of the wrapper */
        Self::obtain()
    }
}

/** # Boot Informations Inner
 *
 * Defines the container of the common boot informations that is initialized
 * and instantiated once across all the HAL/kernel instance
 */
#[derive(Debug, Clone)]
pub(crate) struct BootInfosInner {
    m_hw_phys_mem_offset: VirtAddr,
    m_cmdline_args: CmdLineArgs,
    m_mem_areas: BootMemAreas
}

impl BootInfosInner {
    /** # Constructs a `BootInfosInner`
     *
     * The returned instance copies the given buffers into his
     */
    pub fn new(hw_phys_mem_offset: VirtAddr,
               raw_cmdline: &str,
               mem_areas: BootMemAreas)
               -> Self {
        Self { m_hw_phys_mem_offset: hw_phys_mem_offset,
               m_cmdline_args: CmdLineArgs::new(raw_cmdline),
               m_mem_areas: mem_areas }
    }
}

/** # Hardware Boot Informations Base Interface
 *
 * Defines the method that is required by the [`BootInfosInner`]
 *
 * [`BootInfosInner`]: /hal/boot/struct.BootInfosInner.html
 */
pub(crate) trait HwBootInfosBase {
    /** # Constructs a `BootInfosInner`
     *
     * The instance returned is expected to be filled by the architecture
     * dependent code using the bootloaders informations given via raw
     * pointer
     */
    fn obtain_inner_from_arch_infos(raw_boot_infos_ptr: *const u8) -> BootInfosInner;
}
