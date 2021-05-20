/*! Loader stack management */

use shared::{
    info::vm_layout::VMLayoutArea,
    mem::paging::{
        flush::MapFlusher,
        table::PTFlags
    }
};

use crate::mem::{
    paging::{
        allocator::HHLPageDirAllocator,
        paging_current_page_dir
    },
    vm_layout::vml_core_layout
};

/**
 * Allocates an maps the stack for the kernel core
 */
pub fn loader_setup_core_stack() -> VMLayoutArea {
    let stack_area = vml_core_layout().kern_stack_area();

    /* map the stack area */
    let mapping_res = paging_current_page_dir().map_range(stack_area.as_frame_range(),
                                                          &HHLPageDirAllocator,
                                                          PTFlags::PRESENT
                                                          | PTFlags::READABLE
                                                          | PTFlags::WRITEABLE
                                                          | PTFlags::NO_EXECUTE);
    match mapping_res {
        Ok(map_flusher) => map_flusher.flush(),
        Err(err) => panic!("Failed to map kernel stack: cause: {}", err)
    }

    stack_area.clone()
}
