/*! HH_Loader physical memory management */

use shared::{
    addr::{
        phys::PhysAddr,
        Address
    },
    dbg::{
        dbg_display_size,
        MIB
    },
    infos::info::BootInfos,
    logger::{
        info,
        warn
    },
    mem::{
        bitmap::BitMapAllocator,
        paging::{
            flush::MapFlusher,
            frame::PhysFrame,
            table::PTFlags,
            Page2MiB,
            Page4KiB,
            PageSize
        }
    }
};

use crate::{
    loader::loader_core_preload_cache,
    mem::{
        paging::{
            allocator::HHLPageDirAllocator,
            paging_current_page_dir
        },
        phys::init_allocator::HHLPreInitAllocator,
        vm_layout::vml_core_layout
    }
};

mod init_allocator;

/* bitmap allocator, unused until <CAN_USE_BITMAP> is <false> */
static mut BITMAP_ALLOCATOR: BitMapAllocator = BitMapAllocator::new_uninitialized();

/* simple physical memory allocator, used until <CAN_USE_BITMAP> is <false> */
static mut PRE_INIT_ALLOCATOR: HHLPreInitAllocator = HHLPreInitAllocator::new();

/* <false> until <init_phys_mem()> successfully finish */
static mut CAN_USE_BITMAP: bool = false;

extern "C" {
    static __hhl_text_begin: usize;
    static __hhl_text_end: usize;
}

/**
 * Pre-initializes the physical memory manager module and returns how many
 * 4KiB pages are necessary to map the physical memory bitmap
 */
pub fn phys_pre_init() -> usize {
    let min_memory = loader_core_preload_cache().load_size() + 4 * Page2MiB::SIZE;

    /* calculate the total memory available and warn low memory */
    let total_mem = BootInfos::obtain().mem_areas().iter().map(|area| area.size()).sum();
    if total_mem < min_memory {
        warn!("Detected a VERY SMALL amount of physical memory: less than {}MiB",
              min_memory / MIB);
    }

    /* obtain the range of physical frames occupied by the text of the hh_loader */
    let text_frames_range = {
        let text_begin = unsafe { &__hhl_text_begin as *const _ as usize };
        let text_end = unsafe { &__hhl_text_end as *const _ as usize };

        PhysFrame::range_of(PhysAddr::new(text_begin).containing_frame(),
                            PhysAddr::new(text_end).containing_frame())
    };

    /* instruct the pre-init allocator to not use them */
    unsafe {
        PRE_INIT_ALLOCATOR.skip_range(text_frames_range);
    }

    /* print to the log a bit of informations */
    info!("Total Available Memory: {}", dbg_display_size(total_mem));

    /* return how many pages are necessary to store the bitmap */
    ((total_mem / Page4KiB::SIZE / (u8::BITS as usize)) + Page4KiB::MASK) >> 12
}

/**
 * Initializes the physical memory manager
 */
pub fn phys_init() {
    let bitmap_area = vml_core_layout().phys_mem_bitmap_area();

    /* map into the designated area the bitmap */
    let map_result =
        paging_current_page_dir().map_range(bitmap_area.as_frame_range::<Page4KiB>(),
                                            &HHLPageDirAllocator,
                                            PTFlags::PRESENT
                                            | PTFlags::READABLE
                                            | PTFlags::WRITEABLE
                                            | PTFlags::GLOBAL
                                            | PTFlags::NO_EXECUTE);
    match map_result {
        Ok(flusher) => flusher.flush(),
        Err(err) => panic!("Unable to map physical memory bitmap: cause: {}", err)
    }

    unsafe {
        /* initialize the bitmap allocator */
        BITMAP_ALLOCATOR.init(bitmap_area.start_addr().as_ptr_mut(), bitmap_area.size());

        /* enable now the bits that correspond to the available physical frames */
        if let Some(phys_frames) = PRE_INIT_ALLOCATOR.iter_to_next() {
            /* mark the remaining frames as available */
            for phys_frame in phys_frames {
                BITMAP_ALLOCATOR.add_frame(phys_frame)
            }

            /* now can be used the bitmap allocator */
            CAN_USE_BITMAP = true;
        } else {
            panic!("Pre-init allocator have exhausted the physical memory")
        }
    }
}

/**
 * Allocates a single `PhysFrame<Page4KiB>`
 */
pub fn phys_alloc_frame() -> Option<PhysFrame<Page4KiB>> {
    if can_use_bitmap() {
        unsafe { BITMAP_ALLOCATOR.allocate_one() }
    } else {
        unsafe { PRE_INIT_ALLOCATOR.allocate() }
    }
}

/**
 * Safe wrapper to read `CAN_USE_BITMAP` static
 */
fn can_use_bitmap() -> bool {
    unsafe { CAN_USE_BITMAP }
}
