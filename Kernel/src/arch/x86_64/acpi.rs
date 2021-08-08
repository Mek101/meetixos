/*! x86_64 basic ACPI implementation */

use alloc::vec::Vec;
use core::{
    mem,
    ops::Range,
    ptr
};

use helps::dbg::{
    C_KIB,
    C_MIB
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

static mut SM_ACPI_MANAGER: Option<AcpiManager> = None;

pub struct AcpiManager {
    m_rsdp: &'static RootSysDescPtr,
    m_rsdt_tables: Vec<&'static RootSysDescTable>,
    m_enabled: bool
}

impl AcpiManager /* Constructors */ {
    pub fn init_instance() {
        let rsdp_ptr = Self::find_root_table();
        if let Some(rsdp_ptr_ref) = unsafe { rsdp_ptr.as_ref() } {
            /* constructs the ACPI manager then parse the RSDT tables */
            let mut acpi_manager = Self { m_rsdp: rsdp_ptr_ref,
                                          m_rsdt_tables: Vec::new(),
                                          m_enabled: true };
            acpi_manager.parse_tables();

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
    fn parse_tables(&mut self) {
        dbg_println!(DbgLevel::Debug,
                     "Parsing ACPI tables from root {}",
                     VirtAddr::from(self.m_rsdp as *const _ as *const _));

        /* perform the table parsing according to the version */
        if self.m_rsdp.m_revision == 0 {
            self.do_parse_tables((self.m_rsdp.m_rsdt_addr as usize).into());
        } else {
            self.do_parse_tables((self.m_rsdp.m_xsdt_addr as usize).into());
        }
        //
        // /* obtain the header size and the RSDT, check for extended one */
        // let (rsdt_ptr, entry_ptr_size) = if self.m_rsdp.m_revision > 1 {
        //     (self.m_rsdp.m_xsdt_addr as usize, mem::size_of::<u64>())
        // } else {
        //     (self.m_rsdp.m_rsdt_addr as usize, mem::size_of::<u32>())
        // };
        //
        // /* obtain an accessible RSDT pointer and reference */
        // let (rsdt_ptr, rsdt) = {
        //     let rsdt_ptr = MemManager::instance().layout_manager()
        //
        // .phys_addr_to_virt_addr(rsdt_ptr.into());     (rsdt_ptr,
        // unsafe { rsdt_ptr.as_ref::<RootSysDescTable>() }) };
        //
        // /* obtain how many RSDT entries are present */
        // let rsdt_entry_count =
        //     (rsdt.m_len as usize - mem::size_of::<RootSysDescTable>()) /
        // entry_ptr_size; for i in 1..rsdt_entry_count {
        //     /* obtain the offset to the next entry */
        //     let entry_raw_ptr = *rsdt_ptr + i * entry_ptr_size;
        //
        //     /* truncate the pointer to the size expected */
        //     let next_rsdt_table_ptr = if entry_ptr_size ==
        // mem::size_of::<u64>() {         entry_raw_ptr as *const u64
        // as usize     } else {
        //         entry_raw_ptr as *const u32 as usize
        //     };
        //
        //     /* obtain the pointer to the next table descriptor */
        //     let next_rsdt_table_ptr = {
        //         let virt_entry_raw_ptr =
        //             MemManager::instance().layout_manager()
        //
        // .phys_addr_to_virt_addr(next_rsdt_table_ptr.into())
        //                                   .as_ptr::<usize>();
        //
        //         unsafe { *virt_entry_raw_ptr }
        //     };
        //
        //     /* obtain the accessible reference to the next RSDT table */
        //     let next_rsdt_table = unsafe {
        //         MemManager::instance().layout_manager()
        //
        // .phys_addr_to_virt_addr(next_rsdt_table_ptr.into())
        //                               .as_ref::<RootSysDescTable>()
        //     };
        //
        //     /* save the table reference */
        //     dbg_println!(DbgLevel::Trace,
        //                  "Found RSDT: {:?} -> {}",
        //                  VirtAddr::from(next_rsdt_table as *const _),
        //                  next_rsdt_table.m_len);
        //     self.m_rsdt_tables.push(next_rsdt_table);
        //}
    }

    fn do_parse_tables(&mut self, _rsdt_phys_addr: PhysAddr) {
    }

    fn find_root_table() -> *const RootSysDescPtr {
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
        if let Some(rsdp_ptr) = Self::find_root_table_in(ext_bios_data_area_range) {
            rsdp_ptr
        } else if let Some(rsdp_ptr) = Self::find_root_table_in(bios_area_range) {
            rsdp_ptr
        } else {
            ptr::null()
        }
    }

    fn find_root_table_in(virt_addr_range: Range<VirtAddr>)
                          -> Option<*const RootSysDescPtr> {
        /* step each 16 bytes to find the RSDP */
        for virt_addr in virt_addr_range.step_by(16) {
            let rsdp = unsafe { virt_addr.as_ref::<RootSysDescPtr>() };

            /* validate signature and checksum validity */
            if &rsdp.m_signature == b"RSD PTR "
               && Self::is_valid_checksum(rsdp as *const _ as *const _, rsdp.m_len)
            {
                return Some(virt_addr.as_ptr());
            }
        }
        None
    }

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
struct RootSysDescTable {
    m_signature: [u8; 4],
    m_len: u32,
    m_revision: u32,
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
    m_header: RootSysDescTable,
    _reserved: u32,
    m_dsdt: u32
}

#[repr(C)]
#[repr(packed)]
struct ApicTableEntry {
    m_header: RootSysDescTable,
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
