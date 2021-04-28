/*! # MeetiX Kernel
 *
 * Here is where the common initialization code
 */

#![no_std]
#![no_main]
#![feature(core_intrinsics,
           alloc_error_handler,
           panic_info_message,
           option_result_unwrap_unchecked,
           const_fn,
           once_cell,
           const_fn_fn_ptr_basics,
           iter_advance_by,
           array_methods,
           stmt_expr_attributes)]

//#[macro_use]
extern crate alloc;

use shared::{infos::BootInfos, logger::info};

use crate::{
    interrupt::init_interrupts,
    log::{enable_logger_buffering, init_logger},
    mem::{
        heap::{heap_allocated_mem, init_heap},
        phys::init_phys_mem
    },
    version::KERN_VERSION
};

mod debug;
mod interrupt;
mod log;
mod mem;
mod panic;
mod version;

pub fn write_video(message: &str) {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, byte) in message.as_bytes().iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = *byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xE as u8;
        }
    }
}

/** # Kernel pre-initialization
 *
 * Here is where the MeetiX kernel starts his execution as common code.
 *
 * In this function are initialized the stuffs relatively to physical and
 * dynamic memory management and other little stuffs, then the control is
 * returned to the HAL that enables all the architecture dependent stuffs
 * that requires physical/dynamic memory allocation
 */
#[no_mangle]
pub unsafe extern "C" fn kern_start(boot_infos: BootInfos) {
    /* initialize the kernel's instance of the BootInfos.
     * The given instance references the higher half loader memory, which will be
     * unmapped in the next steps, and become unreachable
     */
    let _ = BootInfos::from(&boot_infos);

    /* initialize the logging system */
    init_logger();

    info!("MeetiX Kernel v{} is booting...", KERN_VERSION);
    write_video("MeetiX Kernel v0.1.0 is booting...");

    /* initialize the physical memory allocator */
    info!("Initializing physical memory...");
    init_phys_mem();

    /* initialize the heap memory allocator */
    info!("Initializing dynamic memory...");
    init_heap();

    /* enable logger buffering */
    info!("Enabling logger buffering...");
    enable_logger_buffering();

    /* initialize the interrupt manager */
    info!("Initializing interrupts...");
    init_interrupts();

    info!("Pre-init done...");
    kern_debug_and_tests();
}

/** # Kernel initialization
 */
fn kern_debug_and_tests() -> ! {
    fn test_4kib_alloc() {
        use crate::mem::phys::phys_mem_alloc_frame;
        use shared::mem::paging::Page4KiB;

        if let Some(phys_frame) = phys_mem_alloc_frame::<Page4KiB>() {
            info!("allocated PhysFrame<Page4KiB>({:?})", phys_frame)
        } else {
            panic!("Failed to allocate a 4KiB frame");
        }
    }

    fn test_2mib_alloc() {
        use crate::mem::phys::phys_mem_alloc_frame;
        use hal::paging::Page2MiB;

        if let Some(phys_frame) = phys_mem_alloc_frame::<Page2MiB>() {
            info!("allocated PhysFrame<Page2MiB>({:?})", phys_frame)
        } else {
            panic!("Failed to allocate a 2MiB frame");
        }
    }

    fn test_heap_alloc_free() {
        use alloc::boxed::Box;
        use dbg_utils::debug_size_multiplier;

        let boxed_int = Box::new([1u64, 2u64, 3u64, 5u64, 6u64, 7u64, 8u64, 9u64, 10u64]);

        info!("\theap_allocated_mem: {}", debug_size_multiplier(heap_allocated_mem()));

        for (i, value) in boxed_int.iter().enumerate() {
            info!("\tvalue ({}, {})", i, value);
        }
    }

    /* dump some informations in debug mode, this block of code is not compiled
     * when the kernel is compiled in release mode but displays many useful debug
     * informations
     */
    #[cfg(debug_assertions)]
    {
        use core::mem::size_of;

        use dbg_utils::debug_size_multiplier;
        use hal::addr::{PhysAddr, VirtAddr};
        use logger::debug;

        use crate::{
            debug::dump_boot_infos,
            mem::{
                heap::{heap_free_memory, heap_managed_mem},
                paging::paging_active_page_dir,
                phys::{
                    phys_mem_allocated_mem, phys_mem_free_memory, phys_mem_total_mem
                }
            }
        };

        dump_boot_infos();

        debug!("Address Size:");
        debug!("\tVirtAddr size = {} bits, PhysAddr size = {} bits",
               size_of::<VirtAddr>() * 8,
               size_of::<PhysAddr>() * 8);

        debug!("Physical Memory Consumption");
        debug!("\tphys_mem_total_mem:     {}",
               debug_size_multiplier(phys_mem_total_mem()));
        debug!("\tphys_mem_allocated_mem: {}",
               debug_size_multiplier(phys_mem_allocated_mem()));
        debug!("\tphys_mem_free_memory:   {}",
               debug_size_multiplier(phys_mem_free_memory()));

        debug!("Dynamic Memory Consumption");
        debug!("\theap_managed_mem:   {}", debug_size_multiplier(heap_managed_mem()));
        debug!("\theap_allocated_mem: {}", debug_size_multiplier(heap_allocated_mem()));
        debug!("\theap_free_memory:   {}", debug_size_multiplier(heap_free_memory()));

        debug!("Page Directory");
        let active_page_dir = paging_active_page_dir();
        debug!("\tactive_page_dir.root_phys_frame: {:?}",
               active_page_dir.root_phys_frame());
        debug!("\n{:?}", active_page_dir);
    }

    info!("Initializing Core modules...");

    for _ in 0..8 {
        test_4kib_alloc();
    }
    for _ in 0..8 {
        test_2mib_alloc()
    }
    for _ in 0..8 {
        test_heap_alloc_free()
    }

    loop {}
}
