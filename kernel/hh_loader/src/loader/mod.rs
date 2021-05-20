/*! Kernel core loader */

use core::slice;

use crate::{
    arch::loader::arch_loader_switch_to_kernel,
    info::info_prepare_loader_info,
    loader::{
        cache::KernelPreLoadCache,
        stack::loader_setup_core_stack
    },
    mem::paging::{
        allocator::HHLPageDirAllocator,
        paging_current_page_dir
    }
};

use shared::{
    addr::{
        virt::VirtAddr,
        Address
    },
    elf::{
        program,
        program::{
            ProgramHeader,
            SegmentData,
            Type
        }
    },
    mem::paging::{
        flush::MapFlusher,
        frame::VirtFrame,
        table::PTFlags,
        Page4KiB,
        PageSize
    }
};

pub mod cache;
pub mod stack;

/* includes the module which links the kernel core binary */
include!(env!("KERNEL_BIN"));

/* various information about the kernel core which are accessed frequently */
static mut KERNEL_PRELOAD_CACHE: Option<KernelPreLoadCache> = None;

/**
 * Initializes the global kernel core cache to be rapidly accessed
 * afterwards
 */
pub fn loader_init_core_cache() {
    assert!(unsafe { KERNEL_PRELOAD_CACHE.is_none() });

    unsafe {
        KERNEL_PRELOAD_CACHE = Some(KernelPreLoadCache::new(KERNEL_BYTES.as_slice()));
    }
}

/**
 * Effectively loads the kernel core
 */
pub fn loader_load_core() {
    /* load the kernel core parts needed for switching */
    let stack_area = loader_setup_core_stack();
    let core_entry = loader_load_core_elf();
    let loader_info = info_prepare_loader_info();

    /* switch to the kernel core */
    unsafe {
        arch_loader_switch_to_kernel(stack_area.end_addr(),
                                     loader_info as *const _,
                                     core_entry);
    }
}

/**
 * Returns the global static reference to the `KernelPreLoadCache`
 */
pub fn loader_core_preload_cache() -> &'static KernelPreLoadCache<'static> {
    if let Some(preload_cache) = unsafe { KERNEL_PRELOAD_CACHE.as_ref() } {
        preload_cache
    } else {
        panic!("Tried to obtain kernel pre-load cache, without pre-loading it");
    }
}

fn loader_load_core_elf() -> VirtAddr {
    let preload_cache = loader_core_preload_cache();

    if let Err(err) = loader_load_elf_segments(preload_cache) {
        panic!("Failed to load kernel's core: cause: {}", err);
    }

    VirtAddr::new(preload_cache.elf_file().header.pt2.entry_point() as usize)
}

fn loader_load_elf_segments(preload_cache: &KernelPreLoadCache)
                            -> Result<(), &'static str> {
    /* iterate each program header and load the kernel */
    for program_hdr in preload_cache.elf_file().program_iter() {
        /* ensure that the program header is sane */
        program::sanity_check(program_hdr, preload_cache.elf_file())?;

        /* load the load segment */
        if program_hdr.get_type()? == Type::Load {
            loader_load_elf_segment(preload_cache, program_hdr)?;
        }
    }
    Ok(())
}

fn loader_load_elf_segment(preload_cache: &KernelPreLoadCache,
                           program_hdr: ProgramHeader)
                           -> Result<(), &'static str> {
    let mem_size = program_hdr.mem_size() as usize;
    let file_size = program_hdr.file_size() as usize;

    /* map into virtual memory the current program segment */
    let seg_start_addr = loader_load_map_segment(program_hdr)?;

    /* construct the byte slice from the given area */
    let mapped_segment_slice =
        unsafe { slice::from_raw_parts_mut(seg_start_addr.as_ptr_mut::<u8>(), mem_size) };

    /* obtain the content for the current segment */
    let elf_content = {
        if let SegmentData::Undefined(elf_content) =
            program_hdr.get_data(preload_cache.elf_file())?
        {
            assert_eq!(elf_content.len(), file_size);
            elf_content
        } else {
            return Err("Bad segment data");
        }
    };

    shared::logger::debug!("segment: map_size: {}, elf_seg_size: {}",
                           mapped_segment_slice.len(),
                           elf_content.len());

    /* first clean the area, then write the content */
    mapped_segment_slice[0] = 0;
    mapped_segment_slice[mapped_segment_slice.len() - 1] = 0;
    //mapped_segment_slice.fill(0);
    //mapped_segment_slice[..file_size].copy_from_slice(elf_content);
    Ok(())
}

fn loader_load_map_segment(program_hdr: ProgramHeader) -> Result<VirtAddr, &'static str> {
    let mut page_dir = paging_current_page_dir();

    /* construct the virtual frame for the current loadable segment */
    let virt_frame_range = {
        let start_frame =
            VirtAddr::new(program_hdr.virtual_addr() as usize).containing_frame();
        let frames_count = (program_hdr.mem_size() / Page4KiB::SIZE as u64) as usize;

        VirtFrame::range_of_count(start_frame, frames_count)
    };

    /* constructs the flags to provide to the mapping range */
    let flags = {
        let mut segment_flags = PTFlags::PRESENT | PTFlags::READABLE;
        if !program_hdr.flags().is_execute() {
            segment_flags |= PTFlags::NO_EXECUTE;
        }
        if program_hdr.flags().is_write() {
            segment_flags |= PTFlags::WRITEABLE;
        }

        segment_flags
    };

    /* map the virtual range allocating new pages */
    match page_dir.map_range(virt_frame_range.clone(), &HHLPageDirAllocator, flags) {
        Ok(map_flusher) => {
            map_flusher.flush();
            Ok(virt_frame_range.start.start_addr())
        },
        Err(err) => Err(err.as_str())
    }
}
