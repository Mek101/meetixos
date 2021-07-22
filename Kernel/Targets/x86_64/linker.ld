ENTRY(kernel_start)

KERNEL_VIRT_BASE = 0xffffffffc0000000;

PHDRS {
    init    PT_LOAD;
    text    PT_LOAD;
    data    PT_LOAD;
    bss     PT_LOAD;
    symbols PT_LOAD;
}

SECTIONS {
    /* load at physical 2MiB, after BIOS */
    . = 2M;

    .init ALIGN(4K): {
        __kernel_init_begin = .;
        KEEP(*(.multiboot*))
        *(.init_text*)
        *(.init_data*)
        __kernel_init_end = .;
    } : init

    . += KERNEL_VIRT_BASE;

    .text ALIGN(4K): AT(ADDR(.text) - KERNEL_VIRT_BASE) {
        __kernel_text_begin = .;
        *(.text*)
    } : text

    .rodata ALIGN(4K): AT(ADDR(.rodata) - KERNEL_VIRT_BASE) {
        *(.rodata*)
    } : text

    .data ALIGN(4K): AT(ADDR(.data) - KERNEL_VIRT_BASE) {
        *(.data*)
        *(.sdata*)
    } : data

    .bss ALIGN(4K) (NOLOAD): AT(ADDR(.bss) - KERNEL_VIRT_BASE) {
        *(.bss*)
        *(.sbss*)
    } : bss

    .symbols ALIGN(4K): AT(ADDR(.symbols) - KERNEL_VIRT_BASE) {
        KEEP(*(.code_symbols*))
        __kernel_text_end = .;
    } : symbols

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note*)
    }
}