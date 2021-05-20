/*! Boot information descriptor */

use shared::{
    info::{
        args::CmdLineArgs,
        descriptor::BOOTLOADER_NAME_LEN_MAX
    },
    mem::paging::{
        frame::VirtFrameRange,
        Page2MiB
    },
    os::str_utils
};

use crate::info::mem_area::BootMemAreas;

/**
 * Collects various information used by the `hh_loader` to initialize itself
 * and then give to the kernel's core
 */
#[derive(Debug, Clone)]
pub struct BootInfo {
    m_mem_areas: BootMemAreas,
    m_cmdline_args: CmdLineArgs,
    m_loader_mapped_range: VirtFrameRange<Page2MiB>,
    m_bootloader_name: [u8; BOOTLOADER_NAME_LEN_MAX],
    m_bootloader_name_len: usize
}

impl BootInfo {
    /**
     * Constructs a `BootInfo` from the given parameters
     */
    pub fn new(mem_areas: BootMemAreas,
               raw_cmdline: &str,
               loader_mapped_range: VirtFrameRange<Page2MiB>,
               bootloader_name: &str)
               -> Self {
        let mut name_buffer = [0; BOOTLOADER_NAME_LEN_MAX];
        str_utils::copy_str_to_u8_buf(&mut name_buffer, bootloader_name);

        Self { m_mem_areas: mem_areas,
               m_cmdline_args: CmdLineArgs::new(raw_cmdline),
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
     * Returns the `BootMemAreas` collection
     */
    pub fn mem_areas(&self) -> &BootMemAreas {
        &self.m_mem_areas
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

/**
 * Interface of methods that is required by the `BootInfo`
 */
pub trait HwBootInfoBase {
    /**
     * The instance returned is expected to be filled by the architecture
     * dependent code using the bootloaders information given via raw
     * pointer
     */
    fn obtain_from_arch_info(raw_boot_info_ptr: *const u8) -> BootInfo;
}
