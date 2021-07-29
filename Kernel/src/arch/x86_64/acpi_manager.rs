/*! x86_64 ACPI support */

use alloc::vec::Vec;
use core::{
    mem,
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
    dbg_println,
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

impl AcpiManager /* Privates */ {
    /**
     * Collects into `m_sdt_tables` all the available tables
     */
    fn collect_tables(&mut self) -> bool {
        dbg_println!(DbgLevel::Debug,
                     "Collecting ACPI tables from root {}",
                     VirtAddr::from(self.m_rsdp as *const _ as *const _));

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
                                  .to_range(C_KIB);
        let bios_area_range =
            MemManager::instance().layout_manager()
                                  .phys_addr_to_virt_addr(0xe0000_usize.into())
                                  .to_range(C_MIB);

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
    m_flags: u32,
    m_apics: *const ApicHeader
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
