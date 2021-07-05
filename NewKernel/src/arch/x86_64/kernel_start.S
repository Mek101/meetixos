/*! x86_64 kernel bootstrap implementation */

.extern kernel_rust_start

.set MULTIBOOT_HEADER_MAGIC,    0xe85250d6
.set MULTIBOOT_ARCH_X86,        0


/* ----------------------------------- .multiboot section ---------------------------------- */

.code32
.section .multiboot

/* multiboot 2 header instance to boot with a multiboot compliant bootloader */
.align 8
kernel_multiboot_header_start:
    /* .header_magic */
    .long       MULTIBOOT_HEADER_MAGIC
    /* .architecture: i386 */
    .long       MULTIBOOT_ARCH_X86
    /* .header_length */
    .long       kernel_multiboot_header_end - kernel_multiboot_header_start
    /* .checksum */
    .long       -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_ARCH_X86 + (kernel_multiboot_header_end - kernel_multiboot_header_start))

.align 8
kernel_multiboot_end_tag:
    .word       0
    .word       0
    .long       8
kernel_multiboot_header_end:

/* ------------------------------------- .data section ------------------------------------- */

.section .init_data

/* reserve into the .data section an RW area to use as initial stack */
.align      0x1000
.type       kernel_init_stack_top, @object
kernel_init_stack_top:
.rept       64 * 1024 /* 64KiB of stack */
    .byte   0
.endr
kernel_init_stack_bottom:

/* reserve into the .data section an RW area to use as initial L4 page table */
.align      0x1000
.type       kernel_init_page_tables_begin, @object
kernel_init_page_tables_begin:
kernel_init_l4_page_table:
.rept       0x1000
    .byte   0
.endr

/* reserve into the .data section an RW area to use as initial L3 page table */
.type       kernel_init_l3_page_table, @object
kernel_init_l3_page_table:
.rept       0x1000
    .byte   0
.endr

/* reserve into the .data section an RW area to use as initial L2 page table */
.type       kernel_init_l2_page_table, @object
kernel_init_l2_page_table:
.rept       0x1000
    .byte   0
.endr
kernel_init_page_tables_end:

/**
 * The loader loads this GDT to switch from real mode to protected mode
 */
.align      4
.type       kernel_prot_mode_gdt_table, @object
kernel_prot_mode_gdt_table:
    .word       kernel_prot_mode_gdt_end - kernel_prot_mode_gdt - 1
    .long       kernel_prot_mode_gdt

.type       kernel_prot_mode_gdt, @object
kernel_prot_mode_gdt:
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
kernel_prot_mode_gdt_end:

/**
 * The loader loads this GDT to switch from protected mode to protected 64bit mode
 */
.align      4
.type       kernel_long_mode_gdt_table, @object
kernel_long_mode_gdt_table:
    .word       kernel_long_mode_gdt_end - kernel_long_mode_gdt - 1
    .long       kernel_long_mode_gdt

.type       kernel_long_mode_gdt, @object
kernel_long_mode_gdt:
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
kernel_long_mode_gdt_end:

/* ------------------------------------- .text32 section ------------------------------------- */

.section .init_text

/**
 * Here is where the kernel starts his execution
 */
.global     kernel_start
.type       kernel_start, @function
kernel_start:
    /* ensure that the interrupts are disabled */
    cli

    /* load the protected mode GDT */
    lgdt        (kernel_prot_mode_gdt_table)

    /* reload segmentation registers to switch into protected mode */
    mov         $0x10, %ax
    mov         %ax,   %ds
    mov         %ax,   %es
    mov         %ax,   %fs
    mov         %ax,   %gs
    mov         %ax,   %ss

kernel_construct_page_table:
    /* put into the first L4's slot the L3 page table with PTE_PRESENT, PTE_WRITEABLE and PTE_GLOBAL */
    mov         $kernel_init_l3_page_table, %eax
    or          $(1 | 1 << 1 | 1 << 8), %eax
    mov         %eax, (kernel_init_l4_page_table)

    /* put into the first L3's slot the L2 page table with PTE_PRESENT, PTE_WRITEABLE and PTE_GLOBAL */
    mov         $kernel_init_l2_page_table, %eax
    or          $(1 | 1 << 1 | 1 << 8), %eax
    mov         %eax, (kernel_init_l3_page_table)

    /* since the kernel uses the memory mapped paging strategy, we need all the memory mapped to a
     * particular offset of the virtual memory so that virtual_addr - PHYS_MEM_OFFSET = physical_addr
     *
     * Since the kernel dynamically chooses his virtual layout, and because we do not know how much
     * physcial memory we have, here we map only a little portion of it (about 64MiB).
     *
     * The memory is identity mapped because the kernel is loaded to a specific physical address,
     * and after the enable of the paging we must be able to continue to execute the kernel code
     * in the early stage.
     *
     * We use 2MiB pages to avoid 3rd and 4th level page-tables
     */

    /* identity map the first 64MiB of memory using 2MiB pages PTE_PRESENT, PTE_WRITEABLE, PTE_HUGE and PTE_GLOBAL */
    mov         $kernel_init_l2_page_table, %eax
    movl        $(0x00000000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),    (%eax)
    movl        $(0x00200000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),   8(%eax)
    movl        $(0x00400000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  16(%eax)
    movl        $(0x00600000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  24(%eax)
    movl        $(0x00800000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  32(%eax)
    movl        $(0x00A00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  40(%eax)
    movl        $(0x00C00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  48(%eax)
    movl        $(0x00E00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  56(%eax)
    movl        $(0x01000000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  64(%eax)
    movl        $(0x01200000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  72(%eax)
    movl        $(0x01400000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  80(%eax)
    movl        $(0x01600000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  88(%eax)
    movl        $(0x01800000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  96(%eax)
    movl        $(0x01A00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 104(%eax)
    movl        $(0x01C00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 112(%eax)
    movl        $(0x01E00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 120(%eax)
    movl        $(0x02000000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 128(%eax)
    movl        $(0x02200000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 136(%eax)
    movl        $(0x02400000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 144(%eax)
    movl        $(0x02600000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 152(%eax)
    movl        $(0x02800000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 160(%eax)
    movl        $(0x02A00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 168(%eax)
    movl        $(0x02C00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 176(%eax)
    movl        $(0x02E00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 184(%eax)
    movl        $(0x03000000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 192(%eax)
    movl        $(0x03200000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 200(%eax)
    movl        $(0x03400000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 208(%eax)
    movl        $(0x03600000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 216(%eax)
    movl        $(0x03800000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 224(%eax)
    movl        $(0x03A00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 232(%eax)
    movl        $(0x03C00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 240(%eax)
    movl        $(0x03E00000 | 1 | 1 << 1 | 1 << 7 | 1 << 8), 248(%eax)

kernel_enable_paging:
    /* setup the long mode GDT now */
    lgdt        (kernel_long_mode_gdt_table)

    /* setup as current the static page directory */
    mov         $kernel_init_l4_page_table, %eax
    mov         %eax, %cr3

    /* enable PAGE_SIZE_EXTENSION, PAE and PAGE_GLOBAL */
    mov         %cr4, %eax
    or          $(1 << 4 | 1 << 5 | 1 << 7), %eax
    mov         %eax, %cr4

    /* enable LONG_MODE and NO_EXECUTE in EFER model specific register */
    mov         $0xC0000080, %ecx
    rdmsr
    or          $(1 | 1 << 8 | 1 << 11), %eax
    wrmsr

    /* finally enable the Ring0 WRITE_PROTECT and PAGING */
    mov         %cr0, %eax
    or          $(1 << 16 | 1 << 31), %eax
    mov         %eax, %cr0

    /* jump to the identity mapped text */
    ljmp        $0x08, $kernel_identity_mapped_text

/* ------------------------------------- .text64 section ------------------------------------- */

.code64
.section .text

.type       kernel_identity_mapped_text, @function
kernel_identity_mapped_text:
    /* setup the initial stack and the base pointer */
    mov         $kernel_init_stack_bottom, %rsp
    mov         %rsp, %rbp

    /* put Multiboot2 pointer into <rdi> (x86_64 first argument) and call rust */
    mov         %rbx, %rdi
    call        kernel_rust_start

kernel_halt:
    /* we should never reach this point */
    jmp kernel_halt