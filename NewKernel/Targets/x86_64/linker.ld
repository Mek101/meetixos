ENTRY(kernel_start)

KERNEL_VIRT_BASE = 0xfffffffffa000000;

SECTIONS {
    /* load at physical 1MiB, after BIOS */
    . = 1M;

    .init_text ALIGN(4K): {
        KEEP(*(.multiboot*))
        *(.init_text*)
    }

    .init_data ALIGN(4K): {
        *(.init_data*)
    }

    . += KERNEL_VIRT_BASE;

    .text ALIGN(4K): AT(ADDR(.text) - KERNEL_VIRT_BASE) {
        __kernel_text_begin = .;
        *(.text*)
    }

    .rodata ALIGN(4K): AT(ADDR(.rodata) - KERNEL_VIRT_BASE) {
        *(.rodata*)
    }

    .data ALIGN(4K): AT(ADDR(.data) - KERNEL_VIRT_BASE) {
        *(.data*)
        *(.sdata*)
    }

    .got ALIGN(4K): AT(ADDR(.got) - KERNEL_VIRT_BASE) {
        *(.got*)
    }

    .bss ALIGN(4K): AT(ADDR(.bss) - KERNEL_VIRT_BASE) {
        *(.bss*)
        *(.sbss*)
        __kernel_text_end = .;
    }
}