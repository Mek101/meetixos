/*! Kernel loader */

/* includes the module which links the kernel core binary */
include!(env!("KERNEL_BIN"));

pub const fn loader_kernel_core_size() -> usize {
    KERNEL_SIZE
}

pub fn loader_load_core() {
    use shared::logger::info;

    let b = KERNEL_BYTES[0];
    info!("First kernel byte is {}", b);
}
