/*! Loader ELF loader module */

use core::slice;

use shared::{
    addr::{
        align_up,
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
        flags::PDirFlags,
        flush::MapFlusher,
        frame::VirtFrame,
        Page4KiB,
        PageSize
    }
};

use crate::{
    loader::{
        cache::KernelPreLoadCache,
        loader_core_preload_cache
    },
    mem::paging::{
        allocators::HHLPageDirAllocator,
        paging_current_page_dir
    }
};

/**
 * Loads into this page directory the kernel core ELF
 */
pub(super) fn loader_elf_load_core_elf() -> VirtAddr {
    let preload_cache = loader_core_preload_cache();

    /* load all the segments of the ELF executable */
    if let Err(err) = loader_load_elf_segments(preload_cache) {
        panic!("Failed to load kernel's core: cause: {}", err);
    }

    /* return the kernel entry point */
    VirtAddr::new(preload_cache.elf_file().header.pt2.entry_point() as usize)
}

/**
 * Loads into the memory every ELF loadable segment
 */
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

/**
 * Loads the current ELF segment, filling with the ELF file content and
 * remapping it with the right protection bits
 */
fn loader_load_elf_segment(preload_cache: &KernelPreLoadCache,
                           program_hdr: ProgramHeader)
                           -> Result<(), &'static str> {
    /* map into virtual memory the current program segment */
    loader_load_map_segment(program_hdr, |seg_virt_start| {
        let mem_size = program_hdr.mem_size() as usize;
        let file_size = program_hdr.file_size() as usize;

        /* construct the byte slice from the given area */
        let mapped_segment_slice = unsafe {
            slice::from_raw_parts_mut(seg_virt_start.as_ptr_mut::<u8>(), mem_size)
        };

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

        /* first clean the area, then write the content */
        mapped_segment_slice.fill(0);
        mapped_segment_slice[..file_size].copy_from_slice(elf_content);
        Ok(())
    })
}

/**
 * Maps the given segment without protections, executes the given functor
 * and remaps the pages with the right protection
 */
fn loader_load_map_segment<F>(program_hdr: ProgramHeader,
                              f: F)
                              -> Result<(), &'static str>
    where F: FnOnce(VirtAddr) -> Result<(), &'static str> {
    /* map the segment with default protection, to be able to write the content */
    let seg_virt_start =
        loader_load_map_segment_with_flags(program_hdr,
                                           PDirFlags::new().set_present()
                                                           .set_readable()
                                                           .set_writeable()
                                                           .set_global()
                                                           .build())?;

    /* execute the operations to fill the memory */
    f(seg_virt_start)?;

    /* construct the right combination of protection bits */
    let map_flags = {
        let mut pd_flags =
            PDirFlags::new().set_present().set_global().set_remap().build();
        if !program_hdr.flags().is_execute() {
            pd_flags.set_no_execute();
        }
        if program_hdr.flags().is_read() {
            pd_flags.set_readable();
        }
        if program_hdr.flags().is_write() {
            pd_flags.set_writeable();
        }

        pd_flags
    };

    /* remap with protections */
    loader_load_map_segment_with_flags(program_hdr, map_flags).map(|_| ())
}

/**
 * Maps the given segment into the page directory with the given protection
 * flags
 */
fn loader_load_map_segment_with_flags(program_hdr: ProgramHeader,
                                      pd_flags: PDirFlags)
                                      -> Result<VirtAddr, &'static str> {
    /* construct the virtual frame for the current loadable segment */
    let virt_frame_range = {
        let start_frame =
            VirtAddr::new(program_hdr.virtual_addr() as usize).containing_frame();

        let frames_count = {
            /* align up to page boundary the memory size, which is not aligned */
            let aligned_mem_size =
                align_up(program_hdr.mem_size() as usize, Page4KiB::SIZE);

            aligned_mem_size / Page4KiB::SIZE
        };

        VirtFrame::range_of_count(start_frame, frames_count)
    };

    /* map the segment into the memory */
    paging_current_page_dir().map_range(virt_frame_range.clone(),
                                        &HHLPageDirAllocator,
                                        pd_flags)
                             .map(|map_flusher| {
                                 map_flusher.flush();
                                 virt_frame_range.start.start_addr()
                             })
                             .map_err(|err| err.as_str())
}
