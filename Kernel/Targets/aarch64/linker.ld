/* Higher Half Loader entry point */
ENTRY(hhl_start)

SECTIONS {
    /* load at physical 1MiB, after BIOS and GRUB */
    . = 0x100000;

    /* MultiBoot2 section */
    .multiboot ALIGN(4096): {
        *(.multiboot)
    }

    /* Higher Half Loader text section */
    .text ALIGN(4096): {
        *(.text .text.*)
    }

    /* Higher Half Loader read only data section */
    .rodata ALIGN(4K): {
        *(.rodata .rodata.*)
    }

    /* Higher Half Loader RW data section */
    .data ALIGN(4K): {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    /* Higher Half Loader GOT section */
    .got ALIGN(4K): {
        *(.got .got.*)
    }

    /* Higher Half Loader BSS section */
    .bss ALIGN(4K): {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
}