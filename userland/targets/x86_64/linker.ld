OUTPUT_FORMAT("elf64-x86-64")

ENTRY(_start)

SECTIONS {
    . = 0x1000;

    .text ALIGN(4K): {
        __text_begin = .;
        *(.text*)
    }

    .rodata ALIGN(4K): {
        *(.rodata*)
    }

    .data ALIGN(4K): {
        *(.data*)
        *(.sdata*)
    }

    .got ALIGN(4K): {
        *(.got*)
    }

    .bss ALIGN(4K): {
        *(.bss*)
        *(.sbss*)
        __text_end = .;
    }
}