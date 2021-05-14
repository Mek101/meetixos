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

static mut KERNEL_PRELOAD_CACHE: Option<KernelPreLoadCache> = None;

pub fn loader_preload_core() {
    assert!(unsafe { KERNEL_PRELOAD_CACHE.is_none() });

    unsafe {
        KERNEL_PRELOAD_CACHE = Some(KernelPreLoadCache::new(KERNEL_BYTES.as_slice()));
    }
}

pub fn loader_load_core() {
    let preload_cache = loader_core_preload_cache();
    for _program_hdr in preload_cache.elf_file().program_iter() {}
}

pub fn loader_core_preload_cache() -> &'static KernelPreLoadCache<'static> {
    if let Some(preload_cache) = unsafe { KERNEL_PRELOAD_CACHE.as_ref() } {
        preload_cache
    } else {
        panic!("Tried to obtain kernel pre-load cache, without pre-loading it");
    }
}

pub struct KernelPreLoadCache<'a> {
    m_elf_file: ElfFile<'a>,
    m_load_size: usize,
    m_load_address: VirtAddr
}

impl<'a> KernelPreLoadCache<'a> {
    /**
     * Constructs a `KernelPreLoadCache` initializing the `ElfFile`
     */
    fn new(core_elf_bytes: &'a [u8]) -> Self {
        /* parse the elf bytes and panic when kernel core image is corrupted */
        let core_elf = match ElfFile::new(core_elf_bytes) {
            Ok(core_elf) => core_elf,
            Err(err) => panic!("Corrupted kernel core image: {}", err)
        };

        /* calculate the necessary memory amount to load the kernel core */
        let load_size =
            core_elf.program_iter()
                    .filter_map(|program_hdr| {
                        let hdr_type = match program_hdr.get_type() {
                            Ok(hdr_type) => hdr_type,
                            Err(err) => panic!("Malformed kernel core header: {}", err)
                        };

                        if hdr_type == Type::Load {
                            Some(program_hdr.mem_size())
                        } else {
                            None
                        }
                    })
                    .sum::<u64>() as usize;

        /* obtain the load virtual address, extracting the first program header */
        let load_address =
            core_elf.program_iter()
                    .filter(|program_hdr| {
                        let hdr_type = match program_hdr.get_type() {
                            Ok(hdr_type) => hdr_type,
                            Err(err) => panic!("Malformed kernel core header: {}", err)
                        };
                        hdr_type == Type::Load
                    })
                    .next()
                    .map(|program_hdr| program_hdr.virtual_addr())
                    .map(|raw_addr| raw_addr as usize)
                    .map(|raw_addr| VirtAddr::new(raw_addr))
                    .unwrap();

        Self { m_elf_file: core_elf,
               m_load_size: load_size,
               m_load_address: load_address }
    }

    /**
     * Returns the reference to the `ElfFile`
     */
    pub fn elf_file(&self) -> &ElfFile<'a> {
        &self.m_elf_file
    }

    /**
     * Returns the memory load size for the kernel
     */
    pub fn load_size(&self) -> usize {
        self.m_load_size
    }

    /**
     * Returns the load `VirtAddr`
     */
    pub fn load_address(&self) -> VirtAddr {
        self.m_load_address
    }
}
