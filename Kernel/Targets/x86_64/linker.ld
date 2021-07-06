ENTRY(kernel_start)

KERNEL_VIRT_BASE = 0xfffffffffa000000;

PHDRS {
    init    PT_LOAD;
    text    PT_LOAD;
    data    PT_LOAD;
    bss     PT_LOAD;
}

SECTIONS {
    /* load at physical 1MiB, after BIOS */
    . = 1M;

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

    .got ALIGN(4K): AT(ADDR(.got) - KERNEL_VIRT_BASE) {
        *(.got*)
    } : data

    .bss ALIGN(4K) (NOLOAD): AT(ADDR(.bss) - KERNEL_VIRT_BASE) {
        *(.bss*)
        *(.sbss*)
        __kernel_text_end = .;
    } : bss

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note*)
    }
}