/*! Kernel heap management */

/* TODO */

use core::alloc::Layout;

//#[global_allocator]
static S_HEAP_ALLOCATOR: FakeAllocator = FakeAllocator;

struct FakeAllocator;

unsafe impl core::alloc::GlobalAlloc for FakeAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        todo!()
    }
}
