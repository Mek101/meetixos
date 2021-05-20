/*! Boot information */

use crate::os::str_utils;

use crate::{
    info::{
        args::CmdLineArgs,
        vm_layout::VMLayout
    },
    mem::paging::{
        frame::VirtFrameRange,
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
    m_loader_mapped_range: VirtFrameRange<Page2MiB>,
    m_bootloader_name: [u8; BOOTLOADER_NAME_LEN_MAX],
    m_bootloader_name_len: usize
}

impl LoaderInfo {
    /**
     * Constructs a `LoaderInfo` from the given arguments
     */
    pub fn new(cmdline_args: CmdLineArgs,
               vm_layout: VMLayout,
               loader_mapped_range: VirtFrameRange<Page2MiB>,
               bootloader_name: &str)
               -> Self {
        let mut name_buffer = [0; BOOTLOADER_NAME_LEN_MAX];
        str_utils::copy_str_to_u8_buf(&mut name_buffer, bootloader_name);

        Self { m_cmdline_args: cmdline_args,
               m_vm_layout: vm_layout,
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
     * Returns the virtual range of pages mapped by the `hh_loader` which
     * the kernel core must unmap.
     *
     * NOTE: Any physical page/page table must be freed because they are
     *       already marked as available into the physical bitmap
     */
    pub fn loader_mapped_range(&self) -> VirtFrameRange<Page2MiB> {
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
