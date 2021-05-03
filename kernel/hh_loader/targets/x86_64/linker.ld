/* Higher Half Loader entry point */
ENTRY(hhl_start)

SECTIONS {
    /* load at physical 1MiB, after BIOS and GRUB */
    . = 0x100000;

    /* Higher Half Loader text section */
    .text ALIGN(4096): {
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
    }
}