OUTPUT_FORMAT("elf64-riscv")

ENTRY(kern_start)

SECTIONS {
    . = 0xfffffffffa000000;

    .text ALIGN(4K): {
        __text_begin = .;
        *(.text .text.*)
    }

    .rodata ALIGN(4K): {
        *(.rodata .rodata.*)
    }

    .data ALIGN(4K): {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    .got ALIGN(4K): {
        *(.got .got.*)
    }

    .bss ALIGN(4K): {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        __text_end = .;
    }
}