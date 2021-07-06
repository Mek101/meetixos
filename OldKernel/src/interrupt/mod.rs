/*! OldKernel Interrupt Management */

pub fn interrupt_init() {
    interrupt_inner_init();
}

fn interrupt_inner_init() {
    interrupt_inner_do_stuffs();
}

fn interrupt_inner_do_stuffs() {
    interrupt_inner_do_stuffs_2();
}

fn interrupt_inner_do_stuffs_2() {
    interrupt_inner_do_stuffs_3();
}

fn interrupt_inner_do_stuffs_3() {
    panic!("Failed to initialize interrupts...");
}
