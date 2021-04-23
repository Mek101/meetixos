.att_syntax

.extern hhl_rust_entry
.global hhl_l4_page_table

/* ----------------------------------- .multiboot section ---------------------------------- */

.code32
.section .multiboot

/* multiboot 2 header instance to boot with a multiboot compliant bootloader */
.align 8
hhl_multiboot_header_start:
    /* .header_magic */
    .long       0xe85250d6
    /* .architecture: i386 */
    .long       0
    /* .header_length */
    .long       hhl_multiboot_header_end - hhl_multiboot_header_start
    /* .checksum */
    .long       -(0xe85250d6 + 0 + (hhl_multiboot_header_end - hhl_multiboot_header_start))

.align 8
hhl_multiboot_end_tag:
    .word       0
    .word       0
    .long       8
hhl_multiboot_header_end:

/* ------------------------------------- .data section ------------------------------------- */

.section .data

/* reserve into the .data section an RW area to use as initial stack */
.align      0x1000
hhl_stack_top:
.rept       0x100 * 0x1000 /* 1MiB of stack */
    .byte   0
.endr
hhl_stack_bottom:

/* reserve into the .data section an RW area to use as initial L4 page table */
.align      0x1000
hhl_page_tables_begin:
hhl_l4_page_table:
.rept       0x1000
    .byte   0
.endr

/* reserve into the .data section an RW area to use as initial L3 page table */
hhl_l3_page_table:
.rept       0x1000
    .byte   0
.endr

/* reserve into the .data section an RW area to use as initial L2 page table */
hhl_l2_page_table:
.rept       0x1000
    .byte   0
.endr
hhl_page_tables_end:

/* ------------------------------------- .text32 section ------------------------------------- */

.section .text

/** # Protected 32bit GDT table
 *
 * The loader loads this GDT to switch from real mode to protected mode
 */
.align 4
hhl_prot_mode_gdt_table:
    .word       hhl_prot_mode_gdt_end - hhl_prot_mode_gdt - 1
    .long       hhl_prot_mode_gdt

hhl_prot_mode_gdt:
    /* null-pointer GDT gate */
    .long       0, 0

    /* protected kernel code selector:
     * value:   0x08
     * base:    0x00000000
     * limit:   0xffffffff
     * type:    0x9a
     * granu:   0xcf
     */
    .byte       0xff, 0xff, 0, 0, 0, 0x9a, 0xcf, 0x00

    /* protected kernel data selector:
     * value:   0x10
     * base:    0x00000000
     * limit:   0xffffffff
     * type:    0x92
     * granu:   0xcf
     */
     .byte      0xff, 0xff, 0, 0, 0, 0x92, 0xcf, 0x00
hhl_prot_mode_gdt_end:

/** # Protected 64bit GDT table
 *
 * The loader loads this GDT to switch from protected mode to protected 64bit mode
 */
.align 4
hhl_long_mode_gdt_table:
    .word       hhl_long_mode_gdt_end - hhl_long_mode_gdt - 1
    .long       hhl_long_mode_gdt

hhl_long_mode_gdt:
    /* null-pointer GDT gate */
    .long       0, 0

    /* 64bit kernel code selector:
     * value:   0x08
     * base:    0x00000000
     * limit:   0xffffffff
     * type:    0x9a
     * granu:   0xa0
     */
    .byte       0xff, 0xff, 0, 0, 0, 0x9a, 0xa0, 0x00

    /* 64bit kernel data selector:
     * value:   0x10
     * base:    0x00000000
     * limit:   0xffffffff
     * type:    0x92
     * granu:   0xa0
     */
    .byte       0xff, 0xff, 0, 0, 0, 0x92, 0xa0, 0x00
hhl_long_mode_gdt_end:

/** # Higher Half Loader entry point
 *
 * Here is where the higher half loader starts his execution
 */
.global hhl_start
hhl_start:
    /* ensure that the interrupts are disabled */
    cli

    /* load the protected mode GDT */
    lgdt        (hhl_prot_mode_gdt_table)

    /* reload segmentation registers to switch into protected mode */
    mov         $0x10, %ax
    mov         %ax,   %ds
    mov         %ax,   %es
    mov         %ax,   %fs
    mov         %ax,   %gs
    mov         %ax,   %ss

    /* put into the first L4's slot the L3 page table with PTE_PRESENT, PTE_WRITEABLE and PTE_GLOBAL */
    mov         $hhl_l3_page_table,     %eax
    or          $(1 | 1 << 1 | 1 << 8), %eax
    mov         %eax,                   (hhl_l4_page_table)

    /* put into the first L3's slot the L2 page table with PTE_PRESENT, PTE_WRITEABLE and PTE_GLOBAL */
    mov         $hhl_l2_page_table,     %eax
    or          $(1 | 1 << 1 | 1 << 8), %eax
    mov         %eax,                   (hhl_l3_page_table)

    /* identity map the first 16MiB of memory with PTE_PRESENT, PTE_WRITEABLE, PTE_HUGE and PTE_GLOBAL */
    mov         $hhl_l2_page_table,     %eax
    movl        $(0x00000000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),   (%eax)
    movl        $(0x00200000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  8(%eax)
    movl        $(0x00400000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 16(%eax)
    movl        $(0x00600000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 24(%eax)
    movl        $(0x00800000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 32(%eax)
    movl        $(0x00A00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 40(%eax)
    movl        $(0x00C00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 48(%eax)
    movl        $(0x00E00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 56(%eax)

hhl_enable_paging:
    /* setup the long mode GDT now */
    lgdt        (hhl_long_mode_gdt_table)

    /* setup as current the static page directory */
    mov         $hhl_l4_page_table, %eax
    mov         %eax,               %cr3

    /* enable PAGE_SIZE_EXTENSION, PAE and PAGE_GLOBAL */
    mov         %cr4,                        %eax
    or          $(1 << 4 | 1 << 5 | 1 << 7), %eax
    mov         %eax,                        %cr4

    /* enable LONG_MODE and NO_EXECUTE in EFER model specific register */
    mov         $0xC0000080, %ecx
    rdmsr
    or          $(1 | 1 << 8 | 1 << 11), %eax
    wrmsr

    /* finally enable the Ring0 WRITE_PROTECT and PAGING */
    mov         %cr0,                 %eax
    or          $(1 << 16 | 1 << 31), %eax
    mov         %eax,                 %cr0

    /* jump to the protected mapped area */
    ljmp        $0x08, $hhl_identity_mapped_text

/* ------------------------------------- .text64 section ------------------------------------- */

.code64

hhl_identity_mapped_text:
    /* setup the initial stack and the base pointer */
    mov         $hhl_stack_bottom, %rsp
    mov         %rsp,              %rbp

    /* put into the first argument register (%rdi for x86_64)
     * the pointer to the MultiBoot2 struct then jump into the
     * rust code
     */
    mov         %rbx, %rdi
    call        hhl_rust_entry

hhl_halt:
    jmp hhl_halt