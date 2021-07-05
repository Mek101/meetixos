/*! Boot information */

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
use helps::str::{
    copy_str_to_u8_buf,
    u8_ptr_to_str_slice,
    u8_slice_to_str_slice
};

/**
 * Size in bytes of the bootloader name store into `BootInfo`
 */
pub const BOOTLOADER_NAME_LEN_MAX: usize = 64;

/**
 * Stores a bunch of information which the `HHLoader` shares with the
 * Kernel core when starts
 */
#[derive(Debug, Clone)]
pub struct LoaderInfo {
    m_cmdline_args: CmdLineArgs,

    /* core VMLayout related fields */
    m_vm_layout: VMLayout,
    m_bitmap_allocated_bits: usize,

    /* ranges to unmap when in Kernel context */
    m_loader_reserved_range: VirtFrameRangeIncl<Page2MiB>,
    m_loader_mapped_range: VirtFrameRangeIncl<Page2MiB>,

    /* Kernel symbols, part of the loader text */
    m_kern_symbols: *const u8,
    m_kern_symbols_len: usize,

    /* the name of the bootloader which have booted the entire Kernel */
    m_bootloader_name: [u8; BOOTLOADER_NAME_LEN_MAX],
    m_bootloader_name_len: usize
}

impl LoaderInfo {
    /**
     * Constructs a `LoaderInfo` from the given arguments
     */
    pub fn new(cmdline_args: CmdLineArgs,
               vm_layout: VMLayout,
               bitmap_allocated_bits: usize,
               loader_reserved_range: VirtFrameRangeIncl<Page2MiB>,
               loader_mapped_range: VirtFrameRangeIncl<Page2MiB>,
               kern_symbols: &str,
               bootloader_name: &str)
               -> Self {
        let mut name_buffer = [0; BOOTLOADER_NAME_LEN_MAX];
        copy_str_to_u8_buf(&mut name_buffer, bootloader_name);

        Self { m_cmdline_args: cmdline_args,
               m_vm_layout: vm_layout,
               m_bitmap_allocated_bits: bitmap_allocated_bits,
               m_loader_reserved_range: loader_reserved_range,
               m_loader_mapped_range: loader_mapped_range,
               m_kern_symbols: kern_symbols.as_ptr(),
               m_kern_symbols_len: kern_symbols.len(),
               m_bootloader_name: name_buffer,
               m_bootloader_name_len: bootloader_name.len() }
    }

    /**
     * Returns the slice to the Kernel's command line
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
     * Returns the amount of bits allocated into the bitmap area
     */
    pub fn bitmap_allocated_bits(&self) -> usize {
        self.m_bitmap_allocated_bits
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
     * Returns the Kernel symbols as slice
     */
    pub fn kernel_symbols_slice(&self) -> &str {
        u8_ptr_to_str_slice(self.m_kern_symbols, self.m_kern_symbols_len)
    }

    /**
     * Returns the bootloader's name
     */
    pub fn bootloader_name(&self) -> &str {
        let name_slice = &self.m_bootloader_name[..self.m_bootloader_name_len];
        u8_slice_to_str_slice(name_slice)
    }
}
