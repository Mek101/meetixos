/*! x86_64 kernel bootstrap implementation */

.extern bsp_rust_start

.set MULTIBOOT_HEADER_MAGIC,    0xe85250d6
.set MULTIBOOT_ARCH_X86,        0

.set KERNEL_VIRT_ADDR,          0xffffffffc0000000

/* ----------------------------------- .multiboot section ---------------------------------- */

.code32
.section .multiboot, "a"

/* multiboot 2 header instance to boot with a multiboot compliant bootloader */
.align      8
.type       kernel_multiboot_header_start, @object
kernel_multiboot_header_start:
    /* .header_magic */
    .long       MULTIBOOT_HEADER_MAGIC
    /* .architecture: i386 */
    .long       MULTIBOOT_ARCH_X86
    /* .header_length */
    .long       kernel_multiboot_header_end - kernel_multiboot_header_start
    /* .checksum */
    .long       -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_ARCH_X86 + (kernel_multiboot_header_end - kernel_multiboot_header_start))
/* TODO
.align      8
.type       kernel_multiboot_acpi_tag, @object
kernel_multiboot_acpi_tag:
    .word       15
    .word       (kernel_multiboot_acpi_tag_end - kernel_multiboot_acpi_tag)
    .long       0
kernel_multiboot_acpi_tag_end:
*/
.align      8
.type       kernel_multiboot_end_tag, @object
kernel_multiboot_end_tag:
    .word       0
    .word       0
    .long       8
kernel_multiboot_header_end:

/* ------------------------------------- .data section ------------------------------------- */

.section .init_data, "a"

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

/* ------------------------------------- .init_text 32 section ------------------------------------- */

.section .init_text, "ax"

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
    /* setup the long mode GDT now */
    lgdt        (kernel_long_mode_gdt_table)

    /* positionate the L4 page-table to the kernel-slot */
    mov         $kernel_init_l4_page_table, %eax
    add         $(((KERNEL_VIRT_ADDR >> 39) & 0x1ff) << 3), %eax

    /* put L3 page-table into L4 selected slot with PTE_PRESENT, PTE_WRITEABLE and PTE_GLOBAL */
    mov         $kernel_init_l3_page_table, %ecx
    or          $(1 | 1 << 1 | 1 << 8), %ecx
    mov         %ecx, (%eax)

    /* positionate the L3 page-table to the kernel-slot */
    mov         $kernel_init_l3_page_table, %eax
    add         $(((KERNEL_VIRT_ADDR >> 30) & 0x1ff) << 3), %eax

    /* put L2 page-table into L3 selected slot with PTE_PRESENT, PTE_WRITEABLE and PTE_GLOBAL */
    mov         $kernel_init_l2_page_table, %ecx
    or          $(1 | 1 << 1 | 1 << 8), %ecx
    mov         %ecx, (%eax)

    /* positionate the L2 page-table to the kernel-slot */
    mov         $kernel_init_l2_page_table, %eax
    add         $(((KERNEL_VIRT_ADDR >> 21) & 0x1ff) << 3), %eax

    /* now map the first 6MiB of memory to the kernel-slot with PTE_PRESENT, PTE_WRITEABLE, PTE_HUGE and PTE_GLOBAL */
    movl        $(0x00000000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),    (%eax)
    movl        $(0x00200000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),   8(%eax)
    movl        $(0x00400000 | 1 | 1 << 1 | 1 << 7 | 1 << 8),  16(%eax)

    /* now map the page-tables at 0x00000000 too with the same protection */
    mov         $kernel_init_l4_page_table, %eax
    mov         $kernel_init_l3_page_table, %ecx
    or          $(1 | 1 << 1 | 1 << 8), %ecx
    mov         %ecx, (%eax)

    /* put into the first L3's slot the L2 page table with PTE_PRESENT, PTE_WRITEABLE and PTE_GLOBAL */
    mov         $kernel_init_l3_page_table, %eax
    mov         $kernel_init_l2_page_table, %ecx
    or          $(1 | 1 << 1 | 1 << 8), %ecx
    mov         %ecx, (%eax)

kernel_enable_paging:
    /* setup as current the static page directory */
    mov         $kernel_init_l4_page_table, %eax
    mov         %eax, %cr3

    /* enable PAGE_SIZE_EXTENSION, PAE and PAGE_GLOBAL */
    mov         %cr4, %eax
    or          $(1 << 4 | 1 << 5 | 1 << 7), %eax
    mov         %eax, %cr4

    /* enable SYSCALL_EXT, LONG_MODE and NO_EXECUTE in EFER model specific register */
    mov         $0xC0000080, %ecx
    rdmsr
    or          $(1 | 1 << 8 | 1 << 11), %eax
    wrmsr

    /* finally enable the Ring0 WRITE_PROTECT and PAGING */
    mov         %cr0, %eax
    or          $(1 << 16 | 1 << 31), %eax
    mov         %eax, %cr0

    /* reload segment registers */
    ljmp        $0x08, $kernel_identity_mapped_text

/* ------------------------------------- .init_text 64 section ------------------------------------- */

.code64
.type       kernel_identity_mapped_text, @function
kernel_identity_mapped_text:
    /* setup the initial stack and the base pointer */
    mov         $kernel_init_stack_bottom, %rsp
    mov         %rsp, %rbp

    /* put Multiboot2 pointer into <rdi> (x86_64 first argument) and call rust */
    mov         %rbx, %rdi
    call        bsp_rust_start

kernel_halt:
    /* we should never reach this point */
    jmp kernel_halt

.code64
.section .data
/* the kernel stack is reserved into the higher-half .data section, since the lower half
 * is unmapped by the <MemManager::init_instance()> initialization
 */
.align      0x1000
.type       kernel_init_stack_top, @object
kernel_init_stack_top:
.rept       512 * 1024 /* 512KiB of stack */
    .byte   0
.endr
kernel_init_stack_bottom: