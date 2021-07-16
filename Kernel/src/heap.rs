/*! Kernel heap management */

/* TODO */

use core::{
    alloc::Layout,
    ptr::NonNull
};

use heap::lazy_locked_heap::LazyLockedHeap;
use helps::{
    align::align_up,
    dbg::{
        DisplaySizePretty,
        C_MIB
    }
};
use sync::mutex::{
    spin_mutex::RawSpinMutex,
    ConstCreatBackRawMutex
};

use crate::{
    dbg_print::DbgLevel,
    dbg_println,
    vm::paging::{
        Page4KiB,
        PageSize
    }
};

const C_INIT_SIZE: usize = 5 * C_MIB;

static mut SM_INIT_HEAP_STORAGE: [u8; C_INIT_SIZE] = [0; C_INIT_SIZE];

#[global_allocator]
static mut SM_HEAP_ALLOCATOR: LazyLockedHeap<RawSpinMutex> = unsafe {
    LazyLockedHeap::new(kernel_heap_raw_mutex_supplier, kernel_heap_mem_supplier)
};

fn kernel_heap_raw_mutex_supplier() -> Option<RawSpinMutex> {
    Some(RawSpinMutex::CONST_CREAT)
}

fn kernel_heap_mem_supplier(requested_size: usize) -> Option<(NonNull<u8>, usize)> {
    static mut SM_GIVEN_PAGES: usize = 0;

    unsafe {
        let requested_pages = align_up(requested_size, Page4KiB::SIZE) / Page4KiB::SIZE;
        dbg_println!(DbgLevel::Trace, "requested_page: {}", requested_pages);
        dbg_println!(DbgLevel::Trace, "C_INIT_SIZE:    {}", C_INIT_SIZE.display_pretty());

        if C_INIT_SIZE / Page4KiB::SIZE < SM_GIVEN_PAGES + requested_pages {
            None
        } else {
            let given_size = SM_GIVEN_PAGES * Page4KiB::SIZE;
            let requested_size = requested_pages * Page4KiB::SIZE;

            let storage =
                &mut SM_INIT_HEAP_STORAGE[given_size..given_size + requested_size];
            SM_GIVEN_PAGES += requested_pages;

            Some((NonNull::new_unchecked(storage.as_mut_ptr()), requested_size))
        }
    }
}

#[alloc_error_handler]
fn kernel_heap_alloc_error_handler(layout: Layout) -> ! {
    panic!("Heap allocation failed, no more memory available, requested {:?}", layout);
}
