/* Higher Half Loader entry point */
ENTRY(hhl_start)

SECTIONS {
    /* load at physical 1MiB, after BIOS */
    . = 1M;

    /* Higher Half Loader text section */
    .text ALIGN(4K): {
        __hhl_text_begin = .;

        /* Keep into the text section the multiboot header */
        KEEP(*(.multiboot*))
        *(.text*)
    }

    /* Higher Half Loader read only data section */
    .rodata ALIGN(4K): {
        *(.rodata*)
    }

    /* Higher Half Loader RW data section */
    .data ALIGN(4K): {
        *(.data*)
        *(.sdata*)
    }

    /* Higher Half Loader GOT section */
    .got ALIGN(4K): {
        *(.got*)
    }

    /* Higher Half Loader BSS section */
    .bss ALIGN(4K): {
        *(.bss*)
        *(.sbss*)

        . = ALIGN(4K);
        __hhl_text_end = .;
    }
}