#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(option_result_unwrap_unchecked)]

use dbg_utils::dbg_display_size;
use hal::{
    boot_infos::BootInfos,
    paging::PageDir
};
use logger::info;

use crate::{
    log::init_logger,
    phys_mem::init_phys_mem,
    version::HHL_VERSION,
    vm_layout::organize_kernel_vm_layout
};

mod arch;
mod log;
mod panic;
mod phys_mem;
mod version;
mod vm_layout;

/* includes the module which links the kernel core binary */
include!(env!("KERNEL_BIN"));

/** # Higher half loader rust entry point
 *
 * Here is where the 64bit rust code starts his execution
 */
#[no_mangle]
pub unsafe extern "C" fn hhl_rust_entry(raw_info_ptr: *const u8) -> ! {
    /* initialize the higher half loader's instance of the BootInfos, it uses the
     * "loader_stage" <From> implementation, which interprets the raw pointer
     * given as the supported bootloader's info for the architecture compiled in
     */
    let boot_info = BootInfos::from(raw_info_ptr);

    /* initialize the logger, to be able to print formatted */
    init_logger();

    /* print the hh_loader header */
    info!("MeetiX Kernel Loader v{}", HHL_VERSION);
    info!("\tKernel size: {}", dbg_display_size(KERNEL_SIZE));
    info!("\tKernel code: {}{}{}", KERNEL_BYTES[0], KERNEL_BYTES[1], KERNEL_BYTES[2]);

    /* organize the VM layout for the kernel */
    info!("Organizing Kernel Core's VM Layout...");
    let _vm_layout = organize_kernel_vm_layout();

    /*  */
    init_phys_mem();

    info!("MeetiX Kernel Loader v{}", HHL_VERSION);
    info!("Kernel size: {}", dbg_display_size(KERNEL_SIZE));
    info!("Kernel code: {}{}{}", KERNEL_BYTES[0], KERNEL_BYTES[1], KERNEL_BYTES[2]);

    info!("Raw info ptr: {:#x}", raw_info_ptr as usize);
    boot_info.cmdline_args().iter().for_each(|arg| info!("Arg: {}", arg.as_str()));
    boot_info.mem_areas().iter().for_each(|mem_area| {
                                    info!("{:?}, {}",
                                          mem_area,
                                          dbg_display_size(mem_area.size()))
                                });

    let page_dir = PageDir::active_page_dir(0);
    info!("\n{:?}", page_dir);

    loop { /* loop forever here */ }
}
