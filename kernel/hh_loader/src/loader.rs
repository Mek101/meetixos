/*! Kernel loader */

use shared::{
    addr::{
        virt::VirtAddr,
        Address
    },
    elf::{
        program::Type,
        ElfFile
    }
};

/* includes the module which links the kernel core binary */
include!(env!("KERNEL_BIN"));

static mut KERNEL_LOAD_CACHE: Option<KernelLoadCache> = None;
static mut

pub fn loader_preload_core() {
    assert!(unsafe { KERNEL_LOAD_CACHE.is_none() });

    match ElfFile::new(&KERNEL_BYTES) {
        Ok(core_elf) => unsafe { KERNEL_LOAD_CACHE = Some(core_elf) },
        Err(err) => panic!("Failed to preload Kernel's Core, due to {}", err)
    }
}

pub fn loader_load_core() {
}

pub fn loader_kernel_core_load_size() -> usize {
    core_elf_file().program_iter()
                   .filter_map(|program_hdr| {
                       if program_hdr.get_type().unwrap() == Type::Load {
                           Some(program_hdr.mem_size())
                       } else {
                           None
                       }
                   })
                   .sum::<u64>() as usize
}

pub fn loader_core_load_address() -> VirtAddr {
    for program_hdr in core_elf_file().program_iter() {
        if program_hdr.get_type().unwrap() == Type::Load {
            return VirtAddr::new(program_hdr.virtual_addr() as usize);
        }
    }

    panic!("Kernel core have no loadable segments");
    //let raw_addr = core_elf_file().header.pt2.entry_point() as usize;
    //VirtAddr::new(raw_addr)
}

/**
 * Safe wrapper to <KERNEL_ELF_FILE>
 */
fn core_elf_file() -> &'static ElfFile<'static> {
    assert!(unsafe { KERNEL_LOAD_CACHE.is_some() });

    unsafe { KERNEL_LOAD_CACHE.as_ref().unwrap() }
}

struct KernelLoadCache<'a> {
    m_elf_file: ElfFile<'a>,
m_load_size: usize,
    m_load_address: VirtAddr
}