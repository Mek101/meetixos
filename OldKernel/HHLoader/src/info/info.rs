/*! Boot information descriptor */

use helps::str::{
    copy_str_to_u8_buf,
    u8_ptr_to_str_slice
};

use shared::{
    info::{
        args::CmdLineArgs,
        descriptor::BOOTLOADER_NAME_LEN_MAX
    },
    mem::paging::{
        frame::VirtFrameRangeIncl,
        Page2MiB
    }
};

use crate::info::mem_area::BootMemAreas;

/**
 * Collects various information used by the `HHLoader` to initialize itself
 * and then give to the OldKernel's core
 */
#[derive(Debug, Clone)]
pub struct BootInfo {
    m_mem_areas: BootMemAreas,
    m_cmdline_args: CmdLineArgs,
    m_loader_reserved_range: VirtFrameRangeIncl<Page2MiB>,
    m_loader_mapped_range: VirtFrameRangeIncl<Page2MiB>,
    m_bootloader_name: [u8; BOOTLOADER_NAME_LEN_MAX],
    m_bootloader_name_len: usize
}

impl BootInfo {
    /**
     * Constructs a `BootInfo` from the given parameters
     */
    pub fn new(mem_areas: BootMemAreas,
               raw_cmdline: &str,
               loader_reserved_range: VirtFrameRangeIncl<Page2MiB>,
               loader_mapped_range: VirtFrameRangeIncl<Page2MiB>,
               bootloader_name: &str)
               -> Self {
        let mut name_buffer = [0; BOOTLOADER_NAME_LEN_MAX];
        copy_str_to_u8_buf(&mut name_buffer, bootloader_name);

        Self { m_mem_areas: mem_areas,
               m_cmdline_args: CmdLineArgs::new(raw_cmdline),
               m_loader_reserved_range: loader_reserved_range,
               m_loader_mapped_range: loader_mapped_range,
               m_bootloader_name: name_buffer,
               m_bootloader_name_len: bootloader_name.len() }
    }

    /**
     * Returns the slice to the OldKernel's command line
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
     * Returns the virtual range on which the `HHLoader` physically
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
        u8_ptr_to_str_slice(self.m_bootloader_name.as_ptr(), self.m_bootloader_name_len)
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
