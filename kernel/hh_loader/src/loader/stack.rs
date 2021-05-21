/*! Loader stack management */

use shared::{
    info::vm_layout::VMLayoutArea,
    logger::debug,
    mem::paging::flush::MapFlusher
};

use crate::mem::{
    paging::{
        allocators::HHLPageDirAllocator,
        paging_current_page_dir
    },
    vm_layout::vml_core_layout
};
use shared::mem::paging::flags::PDirFlags;

/**
 * Allocates an maps the stack for the kernel core
 */
pub fn loader_stack_setup_core_stack() -> VMLayoutArea {
    let stack_area = vml_core_layout().kern_stack_area();
    debug!("Mapping kernel stack at: {}", stack_area);

    /* map the stack area */
    let mapping_res =
        paging_current_page_dir().map_range(stack_area.as_frame_range(),
                                            &HHLPageDirAllocator,
                                            PDirFlags::new().set_present()
                                                            .set_readable()
                                                            .set_writeable()
                                                            .set_no_execute()
                                                            .build());
    match mapping_res {
        Ok(map_flusher) => map_flusher.flush(),
        Err(err) => {
            debug!("\n{:?}", paging_current_page_dir());
            panic!("Failed to map kernel stack: cause: {}", err)
        }
    }

    stack_area.clone()
}
