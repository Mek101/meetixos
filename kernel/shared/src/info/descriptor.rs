/*! Boot information */

use crate::os::str_utils;

use crate::{
    info::{
        args::CmdLineArgs,
        vm_layout::VMLayout
    },
    mem::paging::{
        frame::VirtFrameRangeIncl,
        Page2MiB
    }
};

/**
 * Size in bytes of the bootloader name store into `BootInfo`
 */
pub const BOOTLOADER_NAME_LEN_MAX: usize = 64;

/**
 * Stores a bunch of information which the `hh_loader` shares with the
 * kernel core when starts
 */
#[derive(Debug, Clone)]
pub struct LoaderInfo {
    m_cmdline_args: CmdLineArgs,
    m_vm_layout: VMLayout,
    m_loader_reserved_range: VirtFrameRangeIncl<Page2MiB>,
    m_loader_mapped_range: VirtFrameRangeIncl<Page2MiB>,
    m_bootloader_name: [u8; BOOTLOADER_NAME_LEN_MAX],
    m_bootloader_name_len: usize
}

impl LoaderInfo {
    /**
     * Constructs a `LoaderInfo` from the given arguments
     */
    pub fn new(cmdline_args: CmdLineArgs,
               vm_layout: VMLayout,
               loader_reserved_range: VirtFrameRangeIncl<Page2MiB>,
               loader_mapped_range: VirtFrameRangeIncl<Page2MiB>,
               bootloader_name: &str)
               -> Self {
        let mut name_buffer = [0; BOOTLOADER_NAME_LEN_MAX];
        str_utils::copy_str_to_u8_buf(&mut name_buffer, bootloader_name);

        Self { m_cmdline_args: cmdline_args,
               m_vm_layout: vm_layout,
               m_loader_reserved_range: loader_reserved_range,
               m_loader_mapped_range: loader_mapped_range,
               m_bootloader_name: name_buffer,
               m_bootloader_name_len: bootloader_name.len() }
    }

    /**
     * Returns the slice to the kernel's command line
     */
    pub fn cmdline_args(&self) -> &CmdLineArgs {
        &self.m_cmdline_args
    }

    /**
     * Returns the `VMLayout` collection
     */
    pub fn vm_layout(&self) -> &VMLayout {
        &self.m_vm_layout
    }

    /**
     * Returns the virtual range on which the `hh_loader` physically
     * resides.
     *
     * NOTE: Physical pages in this range can be marked as available again
     */
    pub fn loader_reserved_range(&self) -> VirtFrameRangeIncl<Page2MiB> {
        self.m_loader_reserved_range.clone()
    }

    /**
     * Returns the initial mapped range of virtual memory which must be
     * unmapped
     *
     * NOTE: Physical pages cannot be marked as available
     */
    pub fn loader_mapped_range(&self) -> VirtFrameRangeIncl<Page2MiB> {
        self.m_loader_mapped_range.clone()
    }

    /**
     * Returns the bootloader's name
     */
    pub fn bootloader_name(&self) -> &str {
        str_utils::u8_ptr_to_str_slice(self.m_bootloader_name.as_ptr(),
                                       self.m_bootloader_name_len)
    }
}
