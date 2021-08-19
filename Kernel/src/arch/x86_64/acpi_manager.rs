/*! x86_64 ACPI support */

use alloc::vec::Vec;
use core::{
    marker::PhantomData,
    mem,
    mem::size_of,
    ops::Range,
    ptr
};

use helps::{
    dbg::{
        C_KIB,
        C_MIB
    },
    str::u8_slice_to_str_slice
};

use crate::{
    addr::{
        phys_addr::PhysAddr,
        virt_addr::VirtAddr,
        TAddress
    },
    arch::interrupts::apic_manager::ApicManager,
    dbg_println,
    processor::{
        CpuCoreId,
        Processor
    },
    vm::mem_manager::MemManager,
    DbgLevel
};

/* <None> until <AcpiManager::init_instance()> is called */
static mut SM_ACPI_MANAGER: Option<AcpiManager> = None;

/**
 * ACPI manager
 */
pub struct AcpiManager {
    m_rsdp: &'static RootSysDescPtr,
    m_sdt_tables: Vec<&'static SystemDescTable>,
    m_enabled: bool
}

impl AcpiManager /* Constructors */ {
    /**
     * Initializes the global `AcpiManager` instance
     */
    pub fn init_instance() {
        let rsdp_ptr = Self::find_root_table_ptr();
        if let Some(rsdp_ptr_ref) = unsafe { rsdp_ptr.as_ref() } {
            /* constructs the ACPI */
            let mut acpi_manager = Self { m_rsdp: rsdp_ptr_ref,
                                          m_sdt_tables: Vec::new(),
                                          m_enabled: false };

            /* parse the ACPI tables and enable the manager if done */
            if !acpi_manager.collect_tables() {
                dbg_println!(DbgLevel::Err,
                             "Failed ACPI tables parsing, disabling ACPI support");
            } else {
                acpi_manager.m_enabled = true;
            }

            /* store the global ACPI manager */
            unsafe {
                SM_ACPI_MANAGER = Some(acpi_manager);
            }
        } else {
            dbg_println!(DbgLevel::Debug, "No ACPI support found!");
        }
    }
}

impl AcpiManager /* Methods */ {
    /**
     * Registers the secondary CPUs into the `Processor` module
     */
    pub fn register_ap_cpus(&self) {
        let bsp_cpu_id = Processor::instance().this_core().id();
        dbg_println!(DbgLevel::Trace, "bsp_cpu_id: {}", bsp_cpu_id);

        for &sdt in self.m_sdt_tables.iter() {
            /* find APIC entries */
            if &sdt.m_signature == b"APIC" {
                /* obtain the APIC table entry */
                let apic_table_entry =
                    unsafe { &*(sdt as *const _ as *const ApicTableEntry) };

                /* iterate now all the entries */
                for apic_entry in ApicEntryIter::new(apic_table_entry) {
                    match apic_entry {
                        ApicEntry::LocalApic(local_apic_entry)
                            if local_apic_entry.m_flags & 0x01 != 0 =>
                        {
                            let cpu_core_id = local_apic_entry.m_id as CpuCoreId;
                            if bsp_cpu_id == cpu_core_id {
                                /* skip the BSP CPU since is already registered by the
                                 * <Processor::init_instance()> method
                                 */
                                continue;
                            }

                            dbg_println!(DbgLevel::Debug,
                                         "Registering AP CPU Core: {}",
                                         cpu_core_id);

                            /* register the core for the SMP module */
                            Processor::instance_mut().register_cpu_core(cpu_core_id,
                                                                        true);
                        }
                        ApicEntry::LocalApic(disabled_lapic) => {
                            dbg_println!(DbgLevel::Warn,
                                         "Skipping Hardware Disabled CPU with ID: {}",
                                         disabled_lapic.m_id)
                        },
                        ApicEntry::IoApic(io_apic_entry) => {
                            dbg_println!(DbgLevel::Debug,
                                         "Registering I/O APIC with ID: {}",
                                         io_apic_entry.m_id);
                            ApicManager::instance_mut().add_io_apic(io_apic_entry.m_id,
                                                                    io_apic_entry.m_address as usize,
                                                                    io_apic_entry.m_base_gsi);
                        },
                        ApicEntry::Interrupt(interrupt_entry) => {
                            let polarity_high_active =
                                if (interrupt_entry.m_flags & 0x3) == 1
                                   || (interrupt_entry.m_flags & 0x3) == 0
                                {
                                    true
                                } else {
                                    false
                                };

                            dbg_println!(DbgLevel::Debug,
                                         "Registering Interrupt SO {}, \
                                          delivery_mode_fixed: true, \
                                          polarity_high_active: {}, trigger_edge: true",
                                         interrupt_entry.m_source,
                                         polarity_high_active);

                            ApicManager::instance_mut().configure_irq(interrupt_entry.m_source,
                                                                      interrupt_entry.m_gsi,
                                                                      true,
                                                                      polarity_high_active,
                                                                      true);
                        }
                    }
                }
            }
        }
    }
}

impl AcpiManager /* Getters */ {
    /**
     * Returns the global `AcpiManager` instance
     */
    pub fn instance() -> &'static Self {
        unsafe {
            SM_ACPI_MANAGER.as_ref().expect("Called AcpiManager::instance() before \
                                             AcpiManager::init_instance()")
        }
    }
}

impl AcpiManager /* Privates */ {
    /**
     * Collects into `m_sdt_tables` all the available tables
     */
    fn collect_tables(&mut self) -> bool {
        dbg_println!(DbgLevel::Debug,
                     "Collecting ACPI tables from root {}",
                     VirtAddr::from(self.m_rsdp as *const _));

        /* perform the table parsing according to the version */
        if self.m_rsdp.m_revision == 0 {
            self.collect_tables_revision((self.m_rsdp.m_rsdt_addr as usize).into(),
                                         self.m_rsdp.m_revision)
        } else {
            self.collect_tables_revision((self.m_rsdp.m_xsdt_addr as usize).into(),
                                         self.m_rsdp.m_revision)
        }
    }

    /**
     * Collects the tables according to the `revision` starting from the
     * given `PhysAddr`
     */
    fn collect_tables_revision(&mut self,
                               rsdt_phys_addr: PhysAddr,
                               revision: u8)
                               -> bool {
        const SIZEOF_RSDT: usize = mem::size_of::<SystemDescTable>();

        /* obtain the accessible reference to the RSDT header */
        let rsdt_virt_addr =
            MemManager::instance().layout_manager()
                                  .phys_addr_to_virt_addr(rsdt_phys_addr);
        let rsdt_ref = unsafe { rsdt_virt_addr.as_ref::<SystemDescTable>() };

        if revision == 0 {
            /* ACPI version 1.0, It must be a RSDT */
            if &rsdt_ref.m_signature != b"RSDT" {
                return false;
            }

            let child_tables_start = rsdt_virt_addr.offset(SIZEOF_RSDT).as_ptr::<u32>();
            let tables_count =
                (rsdt_ref.m_len as usize - SIZEOF_RSDT) / mem::size_of::<u32>();

            for i in 0..tables_count {
                /* obtain the next table's physical address */
                let sdt_phys_addr: PhysAddr =
                    unsafe { (*child_tables_start.offset(i as isize)) as usize }.into();

                /* collect the inner tables */
                dbg_println!(DbgLevel::Trace,
                             "RSDT ACPI table num: {} at {}",
                             i,
                             sdt_phys_addr);
                self.collect_table(sdt_phys_addr);
            }
        } else {
            /* ACPI version 2.0+, It must be a XSDT */
            if &rsdt_ref.m_signature != b"XSDT" {
                return false;
            }

            let child_tables_start = rsdt_virt_addr.offset(SIZEOF_RSDT).as_ptr::<u64>();
            let tables_count =
                (rsdt_ref.m_len as usize - SIZEOF_RSDT) / mem::size_of::<u64>();

            for i in 0..tables_count {
                /* obtain the next table's physical address */
                let sdt_phys_addr: PhysAddr =
                    unsafe { (*child_tables_start.offset(i as isize)) as usize }.into();

                /* collect the inner tables */
                dbg_println!(DbgLevel::Trace,
                             "XSDT ACPI table num: {} at {}",
                             i,
                             sdt_phys_addr);
                self.collect_table(sdt_phys_addr);
            }
        }
        true
    }

    /**
     * Stores into `m_sdt_tables` the table stored at the given
     * `sdt_phys_addr`
     */
    fn collect_table(&mut self, sdt_phys_addr: PhysAddr) {
        let sdt_virt_addr =
            MemManager::instance().layout_manager().phys_addr_to_virt_addr(sdt_phys_addr);
        let sdt_ref = unsafe { sdt_virt_addr.as_ref::<SystemDescTable>() };

        match u8_slice_to_str_slice(sdt_ref.m_signature.as_slice()) {
            "FACP" => {
                let fixed_acpi_desc_table =
                    unsafe { sdt_virt_addr.as_ref::<FixedAcpiDescTable>() };
                let rsdt_phys_addr: PhysAddr =
                    (fixed_acpi_desc_table.m_dsdt as usize).into();

                /* collect the sub-inner tables */
                self.collect_tables_revision(rsdt_phys_addr,
                                             fixed_acpi_desc_table.m_header.m_revision);
            },
            _ => {
                /* validate the table before adding */
                if Self::is_valid_checksum(sdt_virt_addr.as_ptr(), sdt_ref.m_len) {
                    self.m_sdt_tables.push(sdt_ref);
                }
            }
        }
    }

    /**
     * Finds the `RootSysDescPtr` pointer into the BIOS data area
     */
    fn find_root_table_ptr() -> *const RootSysDescPtr {
        let ext_bios_data_area_range =
            MemManager::instance().layout_manager()
                                  .phys_addr_to_virt_addr(0x40e_usize.into())
                                  .as_range(C_KIB);
        let bios_area_range =
            MemManager::instance().layout_manager()
                                  .phys_addr_to_virt_addr(0xe0000_usize.into())
                                  .as_range(C_MIB);

        /* first try to find the root-table in the first KiB of the EBDA, then, if
         * fail, try into the bios data area
         */
        if let Some(rsdp_ptr) = Self::find_root_table_ptr_in(ext_bios_data_area_range) {
            rsdp_ptr
        } else if let Some(rsdp_ptr) = Self::find_root_table_ptr_in(bios_area_range) {
            rsdp_ptr
        } else {
            ptr::null()
        }
    }

    /**
     * Finds the `RootSysDescPtr` pointer into the given virtual range
     */
    fn find_root_table_ptr_in(virt_addr_range: Range<VirtAddr>)
                              -> Option<*const RootSysDescPtr> {
        /* step each 16 bytes to find the RSDP */
        for virt_addr in virt_addr_range.step_by(16) {
            let rsdp = unsafe { virt_addr.as_ref::<RootSysDescPtr>() };

            /* validate signature and checksum validity */
            if &rsdp.m_signature == b"RSD PTR "
               && Self::is_valid_checksum(virt_addr.as_ptr(), rsdp.m_len)
            {
                return Some(virt_addr.as_ptr());
            }
        }
        None
    }

    /**
     * Validates the checksum
     */
    fn is_valid_checksum(ptr: *const u8, len: u32) -> bool {
        let mut sum = 0u8;
        for i in 0..len {
            sum = sum.wrapping_add(unsafe { *(ptr.offset(i as isize)) } as u8);
        }
        sum == 0
    }
}

#[repr(C)]
#[repr(packed)]
struct RootSysDescPtr {
    m_signature: [u8; 8],
    m_checksum: u8,
    m_oem_id: [u8; 6],
    m_revision: u8,
    m_rsdt_addr: u32,
    m_len: u32,
    m_xsdt_addr: u64,
    m_xchecksum: u8
}

#[repr(C)]
#[repr(packed)]
struct SystemDescTable {
    m_signature: [u8; 4],
    m_len: u32,
    m_revision: u8,
    m_checksum: u8,
    m_oem_id: [u8; 6],
    m_oem_table_id: [u8; 8],
    m_oem_revision: u32,
    m_creator_id: [u8; 4],
    m_creator_revision: u32
}

#[repr(C)]
#[repr(packed)]
struct FixedAcpiDescTable {
    m_header: SystemDescTable,
    _reserved: u32,
    m_dsdt: u32
}

#[repr(C)]
#[repr(packed)]
struct ApicTableEntry {
    m_header: SystemDescTable,
    m_apic_addr: u32,
    m_flags: u32
}

#[repr(C)]
#[repr(packed)]
struct ApicHeader {
    m_type: ApicType,
    m_len: u8
}

#[repr(u8)]
enum ApicType {
    LocalApic = 0,
    IoApic    = 1,
    Interrupt = 2
}

#[repr(C)]
#[repr(packed)]
struct LocalApicEntry {
    m_header: ApicHeader,
    m_cpu: u8,
    m_id: u8,
    m_flags: u32
}

#[repr(C)]
#[repr(packed)]
struct IoApicEntry {
    m_header: ApicHeader,
    m_id: u8,
    _reserved: u8,
    m_address: u32,
    m_base_gsi: u32
}

#[repr(C)]
#[repr(packed)]
struct ApicInterruptSourceOverrideEntry {
    m_header: ApicHeader,
    m_bus: u8,
    m_source: u8,
    m_gsi: u32,
    m_flags: u16
}

enum ApicEntry<'a> {
    LocalApic(&'a LocalApicEntry),
    IoApic(&'a IoApicEntry),
    Interrupt(&'a ApicInterruptSourceOverrideEntry)
}

struct ApicEntryIter<'a> {
    m_entry_ptr: *const u8,
    m_rem_len: u32, /* since ApicTableEntry.m_len is u32 */
    _unused: PhantomData<ApicEntry<'a>>
}

impl<'a> ApicEntryIter<'a> /* Constructors */ {
    fn new(apic_table_entry: &ApicTableEntry) -> Self {
        let entry_ptr = unsafe {
            (apic_table_entry as *const _ as *const u8).offset(size_of::<ApicTableEntry>()
                                                               as isize)
        };
        let entries_len =
            apic_table_entry.m_header.m_len - (size_of::<ApicTableEntry>() as u32);
        dbg_println!(DbgLevel::Trace,
                     "apic_table_entry.m_apics: {:#018x}",
                     entry_ptr as usize);
        Self { m_entry_ptr: entry_ptr,
               m_rem_len: entries_len,
               _unused: PhantomData }
    }
}

impl<'a> Iterator for ApicEntryIter<'a> {
    type Item = ApicEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.m_rem_len > 0 {
            /* obtain the DST-APIC header */
            let apic_header = unsafe { &*(self.m_entry_ptr as *const ApicHeader) };
            /* update the values */
            self.m_entry_ptr =
                unsafe { self.m_entry_ptr.offset(apic_header.m_len as isize) };
            self.m_rem_len -= apic_header.m_len as u32;

            match apic_header.m_type {
                ApicType::LocalApic => Some(ApicEntry::LocalApic(unsafe {
                                                &*(apic_header as *const _
                                                   as *const LocalApicEntry)
                                            })),
                ApicType::IoApic => Some(ApicEntry::IoApic(unsafe {
                                             &*(apic_header as *const _
                                                as *const IoApicEntry)
                                         })),
                ApicType::Interrupt => Some(ApicEntry::Interrupt(unsafe {
                                                &*(apic_header as *const _ as *const ApicInterruptSourceOverrideEntry)
                                            }))
            }
        } else {
            None
        }
    }
}
