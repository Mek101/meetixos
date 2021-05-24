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
           once_cell,
           const_fn_fn_ptr_basics,
           iter_advance_by,
           array_methods,
           stmt_expr_attributes,
           const_fn_trait_bound)]

//#[macro_use]
extern crate alloc;

use shared::{
    info::descriptor::LoaderInfo,
    logger::log_info
};

use crate::{
    interrupt::init_interrupts,
    log::{
        init_logger,
        log_enable_buffering
    },
    mem::{
        heap::init_heap,
        phys::init_phys_mem
    },
    version::KERN_VERSION
};
use core::fmt::Write;
use shared::uart::Uart;

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
pub unsafe extern "C" fn kern_start(_boot_info: *const LoaderInfo) {
    /* initialize the kernel's instance of the BootInfo.
     * The given instance references the higher half loader memory, which will be
     * unmapped in the next steps, and become unreachable
     */
    //let _ = BootInfo::from_other(boot_info);

    let mut uart = Uart::new();
    uart.init();
    write!(uart, "MeetiX Kernel v{} is booting...", KERN_VERSION);

    /* initialize the logging system */
    //init_logger();

    //log_info!("MeetiX Kernel v{} is booting...", KERN_VERSION);
    write_video("MeetiX Kernel v0.1.0 is booting...");
    loop {}

    /* initialize the physical memory allocator */
    log_info!("Initializing physical memory...");
    init_phys_mem();

    /* initialize the heap memory allocator */
    log_info!("Initializing dynamic memory...");
    init_heap();

    /* enable logger buffering */
    log_info!("Enabling logger buffering...");
    log_enable_buffering(false);

    /* initialize the interrupt manager */
    log_info!("Initializing interrupts...");
    init_interrupts();

    log_info!("Pre-init done...");
    kern_debug_and_tests();
}

/** # Kernel initialization
 */
fn kern_debug_and_tests() -> ! {
    /*fn test_4kib_alloc() {
        use crate::mem::phys::phys_mem_alloc_frame;
        use shared::mem::paging::Page4KiB;

        if let Some(phys_frame) = phys_mem_alloc_frame::<Page4KiB>() {
            log_info!("allocated PhysFrame<Page4KiB>({:?})", phys_frame)
        } else {
            panic!("Failed to allocate a 4KiB frame");
        }
    }

    fn test_2mib_alloc() {
        use crate::mem::phys::phys_mem_alloc_frame;
        use hal::paging::Page2MiB;

        if let Some(phys_frame) = phys_mem_alloc_frame::<Page2MiB>() {
            log_info!("allocated PhysFrame<Page2MiB>({:?})", phys_frame)
        } else {
            panic!("Failed to allocate a 2MiB frame");
        }
    }

    fn test_heap_alloc_free() {
        use alloc::boxed::Box;
        use dbg_utils::dbg_display_size;

        let boxed_int = Box::new([1u64, 2u64, 3u64, 5u64, 6u64, 7u64, 8u64, 9u64, 10u64]);

        log_info!("\theap_allocated_mem: {}", dbg_display_size(heap_allocated_mem()));

        for (i, value) in boxed_int.iter().enumerate() {
            log_info!("\tvalue ({}, {})", i, value);
        }
    }*/

    /* dump some information in debug mode, this block of code is not compiled
     * when the kernel is compiled in release mode but displays many useful debug
     * information
     */
    /*#[cfg(debug_assertions)]
    {
        use core::mem::size_of;

        use dbg_utils::dbg_display_size;
        use hal::addr::{
            PhysAddr,
            VirtAddr
        };
        use logger::debug;

        use crate::{
            debug::dump_boot_info,
            mem::{
                heap::{
                    heap_free_memory,
                    heap_managed_mem
                },
                paging::paging_active_page_dir,
                phys::{
                    phys_mem_allocated_mem,
                    phys_mem_free_memory,
                    phys_mem_total_mem
                }
            }
        };

        dump_boot_info();

        log_debug!("Address Size:");
        log_debug!("\tVirtAddr size = {} bits, PhysAddr size = {} bits",
               size_of::<VirtAddr>() * 8,
               size_of::<PhysAddr>() * 8);

        log_debug!("Physical Memory Consumption");
        log_debug!("\tphys_mem_total_mem:     {}", dbg_display_size(phys_mem_total_mem()));
        log_debug!("\tphys_mem_allocated_mem: {}",
               dbg_display_size(phys_mem_allocated_mem()));
        log_debug!("\tphys_mem_free_memory:   {}", dbg_display_size(phys_mem_free_memory()));

        log_debug!("Dynamic Memory Consumption");
        log_debug!("\theap_managed_mem:   {}", dbg_display_size(heap_managed_mem()));
        log_debug!("\theap_allocated_mem: {}", dbg_display_size(heap_allocated_mem()));
        log_debug!("\theap_free_memory:   {}", dbg_display_size(heap_free_memory()));

        log_debug!("Page Directory");
        let active_page_dir = paging_active_page_dir();
        log_debug!("\tactive_page_dir.root_phys_frame: {:?}",
               active_page_dir.root_phys_frame());
        log_debug!("\n{:?}", active_page_dir);
    }

    log_info!("Initializing Core modules...");

    for _ in 0..8 {
        test_4kib_alloc();
    }
    for _ in 0..8 {
        test_2mib_alloc()
    }
    for _ in 0..8 {
        test_heap_alloc_free()
    }*/

    loop {}
}
