use dbg_utils::dbg_display_size;
use hal::boot_infos::VMLayout;
use logger::info;

pub fn organize_kernel_vm_layout() -> VMLayout {
    let vm_layout = VMLayout::new_zero();

    info!("\tKernel Text: {: >20?}..{:?} ({})",
          vm_layout.kern_text_area().start_addr(),
          vm_layout.kern_text_area().end_addr(),
          dbg_display_size(vm_layout.kern_text_area().size()));
    info!("\tKernel Heap: {: >20?}..{:?} ({})",
          vm_layout.kern_heap_area().start_addr(),
          vm_layout.kern_heap_area().end_addr(),
          dbg_display_size(vm_layout.kern_heap_area().size()));
    info!("\tPhysical Memory Bitmap: {: >20?}..{:?} ({})",
          vm_layout.phys_mem_bitmap_area().start_addr(),
          vm_layout.phys_mem_bitmap_area().end_addr(),
          dbg_display_size(vm_layout.phys_mem_bitmap_area().size()));
    info!("\tPhysical Memory Mapping: {: >20?}..{:?} ({})",
          vm_layout.phys_mem_mapping_area().start_addr(),
          vm_layout.phys_mem_mapping_area().end_addr(),
          dbg_display_size(vm_layout.phys_mem_mapping_area().size()));
    info!("\tDisks Page Cache: {: >20?}..{:?} ({})",
          vm_layout.page_cache_area().start_addr(),
          vm_layout.page_cache_area().end_addr(),
          dbg_display_size(vm_layout.page_cache_area().size()));
    info!("\tTemporary Page Mappings: {: >20?}..{:?} ({})",
          vm_layout.tmp_map_area().start_addr(),
          vm_layout.tmp_map_area().end_addr(),
          dbg_display_size(vm_layout.tmp_map_area().size()));

    vm_layout
}
