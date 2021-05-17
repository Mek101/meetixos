/*! Boot information */

#[cfg(feature = "loader_stage")]
use os::str_utils;

#[cfg(feature = "loader_stage")]
use crate::{
    arch::info::HwBootInfo,
    info::mem_area::BootMemAreas
};

use crate::info::{
    args::CmdLineArgs,
    vm_layout::VMLayout
};

/**
 * Size in bytes of the bootloader name store into `BootInfoInner`
 */
pub(crate) const BOOTLOADER_NAME_LEN_MAX: usize = 64;

/**
 * It is initialized by the `BootInfo::from()` implementation
 */
static mut BOOT_INFO_INNER: Option<BootInfoInner> = None;

/**
 * Stores an immutable reference to the common bootloader's information
 */
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct BootInfo {
    m_inner: &'static BootInfoInner
}

impl BootInfo {
    /**
     * Constructs a `BootInfo` filled with the global `BootInfoInner`
     */
    pub fn obtain() -> Self {
        unsafe {
            assert!(BOOT_INFO_INNER.is_some(),
                    "HAL haven't initialized boot information");
            Self { m_inner: BOOT_INFO_INNER.as_ref().unwrap() }
        }
    }

    /**
     * Initializes the global inner information from the given raw
     * information pointer then constructs the `BootInfo` instance.
     *
     * Used by the higher half loader to initialize his instance of the
     * `BootInfoInner`
     */
    #[cfg(feature = "loader_stage")]
    pub fn from_raw(raw_info_ptr: *const u8) -> Self {
        unsafe {
            assert!(BOOT_INFO_INNER.is_none(), "Tried to re-initialize inner BootInfo");
        }

        /* obtain the information inner and store to the global struct */
        let inner_info = HwBootInfo::obtain_inner_from_arch_info(raw_info_ptr);
        unsafe {
            BOOT_INFO_INNER = Some(inner_info);
        }

        /* return an instance of the wrapper */
        Self::obtain()
    }

    /**
     * Initializes the global inner information cloning the given instance
     * then constructs the `BootInfo` instance.
     *
     * Used by the kernel core to clone the higher half loader's instance of
     * the `BootInfoInner` into the higher half instance
     */
    #[cfg(not(feature = "loader_stage"))]
    pub fn from_other(rhs: BootInfo) -> Self {
        unsafe {
            assert!(BOOT_INFO_INNER.is_none(), "Tried to re-initialize inner BootInfo");
        }

        /* clone the info information inner and store to our global copy */
        unsafe {
            BOOT_INFO_INNER = Some(rhs.m_inner.clone());
        }

        /* return an instance of the wrapper */
        Self::obtain()
    }

    /**
     * Returns the slice to the kernel's command line
     */
    pub fn cmdline_args(&self) -> &'static CmdLineArgs {
        &self.m_inner.m_cmdline_args
    }

    /**
     * Returns the `BootMemAreas` collection
     */
    #[cfg(feature = "loader_stage")]
    pub fn mem_areas(&self) -> &'static BootMemAreas {
        &self.m_inner.m_mem_areas
    }

    /**
     * Returns the `VMLayout` collection
     */
    pub fn vm_layout(&self) -> &'static VMLayout {
        &self.m_inner.m_vm_layout
    }
}

/**
 * Container of the common info information that is initialized and
 * instantiated once across all the HAL/kernel instance
 */
#[derive(Debug)]
pub(crate) struct BootInfoInner {
    #[cfg(feature = "loader_stage")]
    m_mem_areas: BootMemAreas,
    m_cmdline_args: CmdLineArgs,
    m_vm_layout: VMLayout,
    m_bootloader_name: [u8; BOOTLOADER_NAME_LEN_MAX]
}

#[cfg(feature = "loader_stage")]
impl BootInfoInner {
    /**
     * Constructs a `BootInfoInner` with the given arguments
     */
    pub(crate) fn new(raw_cmdline: &str,
                      mem_areas: BootMemAreas,
                      bootloader_name: &str)
                      -> Self {
        let mut name_buffer = [0; BOOTLOADER_NAME_LEN_MAX];
        str_utils::copy_str_to_u8_buf(&mut name_buffer, bootloader_name);

        Self { m_cmdline_args: CmdLineArgs::new(raw_cmdline),
               m_mem_areas: mem_areas,
               m_vm_layout: VMLayout::new_zero(),
               m_bootloader_name: name_buffer }
    }
}

#[cfg(not(feature = "loader_stage"))]
impl Clone for BootInfoInner {
    fn clone(&self) -> Self {
        Self { m_cmdline_args: self.m_cmdline_args.clone(),
               m_vm_layout: self.m_vm_layout.clone(),
               m_bootloader_name: self.m_bootloader_name.clone() }
    }
}

/**
 * Interface of methods that is required by the `BootInfoInner`
 */
#[cfg(feature = "loader_stage")]
pub(crate) trait HwBootInfoBase {
    /**
     * The instance returned is expected to be filled by the architecture
     * dependent code using the bootloaders information given via raw
     * pointer
     */
    fn obtain_inner_from_arch_info(raw_boot_info_ptr: *const u8) -> BootInfoInner;
}
