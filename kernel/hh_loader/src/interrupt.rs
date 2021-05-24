/*! Kernel interrupt manager */

use ::x86_64::{
    instructions::{
        segmentation::set_cs,
        tables::load_tss
    },
    structures::{
        gdt::{
            Descriptor,
            GlobalDescriptorTable
        },
        idt::{
            InterruptDescriptorTable,
            InterruptStackFrame,
            PageFaultErrorCode
        },
        tss::TaskStateSegment
    },
    VirtAddr
};

use shared::logger::info;

static mut IDT: Option<InterruptDescriptorTable> = None;
static mut TSS: Option<TaskStateSegment> = None;
static mut GDT: Option<GlobalDescriptorTable> = None;

pub fn init_interrupts() {
    unsafe {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[0] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        TSS = Some(tss);

        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(TSS.as_ref().unwrap()));

        GDT = Some(gdt);

        GDT.as_mut().unwrap().load();
        set_cs(code_selector);
        load_tss(tss_selector);

        let mut idt = InterruptDescriptorTable::new();

        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_intr_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_error_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(0);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);

        IDT = Some(idt);

        IDT.as_mut().unwrap().load();
    }
    info!("Interrupts initialized");
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    panic!("DIVIDE ERROR\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    panic!("OVERFLOW EXCEPTION\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn debug_handler(stack_frame: InterruptStackFrame) {
    panic!("DEBUG EXCEPTION\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn non_maskable_intr_handler(stack_frame: InterruptStackFrame) {
    panic!("NON MASKABLE INTERRUPT\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: InterruptStackFrame) {
    panic!("BOUND RANGE EXCEEDED\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn invalid_opcode_error_handler(stack_frame: InterruptStackFrame) {
    panic!("INVALID OPERATION CODE\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn device_not_available_handler(stack_frame: InterruptStackFrame) {
    panic!("DEVICE NOT AVAILABLE\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame,
                                               error_code: u64)
                                               -> ! {
    panic!("EXCEPTION: DOUBLE FAULT {}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    info!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame,
                                              error_code: u64) {
    panic!("INVALID TSS: {}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn segment_not_present_handler(stack_frame: InterruptStackFrame,
                                                      error_code: u64) {
    panic!("SEGMENT NOT PRESENT: {}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: InterruptStackFrame,
                                                      error_code: u64) {
    panic!("STACK SEGMENT FAULT: {}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: InterruptStackFrame,
                                                           error_code: u64) {
    panic!("GENERAL PROTECTION FAULT: {}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame,
                                             error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;

    info!("EXCEPTION: PAGE FAULT");
    info!("Accessed Address: {:?}", Cr2::read());
    info!("Error Code: {:?}", error_code);
    info!("{:#?}", stack_frame);
    panic!()
}

extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: InterruptStackFrame) {
    panic!("X87 FLOATING POINT\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn alignment_check_handler(stack_frame: InterruptStackFrame,
                                                  error_code: u64) {
    panic!("ALIGNMENT CHECK {}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    panic!("MACHINE CHECK\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: InterruptStackFrame) {
    panic!("SIMD FLOATING POINT\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn virtualization_handler(stack_frame: InterruptStackFrame) {
    panic!("VIRTUALIZATION\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn security_exception_handler(stack_frame: InterruptStackFrame,
                                                     error_code: u64) {
    panic!("SECURITY EXCEPTION {}\n{:#?}", error_code, stack_frame);
}
