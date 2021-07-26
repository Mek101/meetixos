/*! Kernel boot information */

use core::ops::Range;

use helps::str::{
    copy_str_to_u8_buf,
    u8_slice_to_str_slice
};

use crate::{
    addr::phys_addr::PhysAddr,
    arch::hw_boot_info::HwBootInfo
};
use core::str::FromStr;

/* Global BootInfo instance which will live for all the kernel's life */
static mut SM_BOOT_INFORMATION: Option<BootInfo> = None;

/**
 * Bootloader & architecture independent information useful for kernel
 * booting
 */
pub struct BootInfo {
    m_boot_loader_name_buf: [u8; 64],
    m_boot_loader_name_buf_len: usize,

    m_cmd_line_args_buf: [u8; 1024],
    m_cmd_line_args_buf_len: usize,

    m_boot_mem_areas: BootMemAreas
}

impl BootInfo /* Constructors */ {
    /**
     * Initializes the global instance of the `BootInfo` from the
     * raw-pointer given
     */
    pub fn init_instance(raw_boot_info_ptr: *const u8) {
        /* this function cannot be called more than one time */
        unsafe {
            assert!(SM_BOOT_INFORMATION.is_none(),
                    "Called BootInfo::init_instance() more than one time");
        }

        /* parse the bootloader information with the architecture implementation */
        let hw_boot_info = HwBootInfo::from(raw_boot_info_ptr);

        let mut boot_loader_name_buf = [0; 64];
        copy_str_to_u8_buf(&mut boot_loader_name_buf, hw_boot_info.boot_loader_name());

        let mut cmd_line_buf = [0; 1024];
        copy_str_to_u8_buf(&mut cmd_line_buf, hw_boot_info.cmd_line_args());

        let boot_info =
            Self { m_boot_loader_name_buf: boot_loader_name_buf,
                   m_boot_loader_name_buf_len: hw_boot_info.boot_loader_name().len(),
                   m_cmd_line_args_buf: cmd_line_buf,
                   m_cmd_line_args_buf_len: hw_boot_info.cmd_line_args().len(),
                   m_boot_mem_areas: hw_boot_info.mem_areas() };

        unsafe {
            /* store the global instance of the BootInfo */
            SM_BOOT_INFORMATION = Some(boot_info);
        }
    }
}

impl BootInfo /* Methods */ {
    /**
     * Returns whether the given key exists into the command line
     */
    pub fn cmd_line_arg_exists(&self, key_to_search: &str) -> bool {
        self.cmd_line_args()
            .split_whitespace()
            .find(|arg_str| arg_str.contains(key_to_search))
            .is_some()
    }

    /**
     * Searches for a command line option with the given `key_to_search`.
     *
     * If it founds the key returns the key and, if any, the option after
     * the `=`
     */
    pub fn cmd_line_find_arg(&self, key_to_search: &str) -> Option<(&str, Option<&str>)> {
        self.cmd_line_args()
            .split_whitespace()
            .find(|arg_str| arg_str.contains(key_to_search))
            .map(|arg_str| {
                if let Some(eq_sign_pos) = arg_str.find("=") {
                    (&arg_str[..eq_sign_pos], Some(&arg_str[eq_sign_pos + 1..]))
                } else {
                    (arg_str, None)
                }
            })
    }

    /**
     * Searches for a command line option with the given `key_to_search`.
     *
     * If it founds the key returns the key and, if any, the option after
     * the `=` as `usize` value
     */
    pub fn cmd_line_find_arg_int(&self,
                                 key_to_search: &str)
                                 -> Option<(&str, Option<usize>)> {
        self.cmd_line_find_arg(key_to_search).map(|(key, value_opt)| {
                                                 let int_value =
                                                     value_opt.map(|str_value| {
                                                                  if let Ok(int_value) =
                                                             usize::from_str(str_value)
                                                         {
                                                             int_value
                                                         } else {
                                                             panic!("invalid integer \
                                                                     for `{}`: {}",
                                                                    key_to_search,
                                                                    str_value)
                                                         }
                                                              });

                                                 (key, int_value)
                                             })
    }
}

impl BootInfo /* Static Functions */ {
    /**
     * Returns the reference to the global `BootInfo`
     */
    pub fn instance() -> &'static BootInfo {
        unsafe {
            SM_BOOT_INFORMATION.as_ref().expect("Called BootInfo::instance() before \
                                                 BootInfo::init_instance()")
        }
    }
}

impl BootInfo /* Getters */ {
    /**
     * Returns the name of the bootloader which have started the kernel
     */
    pub fn boot_loader_name(&self) -> &str {
        u8_slice_to_str_slice(&self.m_boot_loader_name_buf
                                  [..self.m_boot_loader_name_buf_len])
    }

    /**
     * Returns the command-line arguments given by the bootloader
     */
    pub fn cmd_line_args(&self) -> &str {
        u8_slice_to_str_slice(&self.m_cmd_line_args_buf[..self.m_cmd_line_args_buf_len])
    }

    /**
     * Returns the `BootMemAreas`
     */
    pub fn boot_mem_areas(&self) -> &BootMemAreas {
        &self.m_boot_mem_areas
    }
}

/**
 * Base interface on which the `BootInfo` relies to obtain the necessary
 * information from the architecture dependent structure of boot-information
 */
pub trait THwBootInfo: From<*const u8> {
    /**
     * Returns the bootloader-name string
     */
    fn boot_loader_name(&self) -> &str;

    /**
     * Returns the raw command-line string
     */
    fn cmd_line_args(&self) -> &str;

    /**
     * Returns a filled `BootMemAreas`
     */
    fn mem_areas(&self) -> BootMemAreas;
}

/**
 * Fixed collection of `Range<PhysAddr>`
 */
#[derive(Clone)]
pub struct BootMemAreas {
    m_phys_regions_ranges: [Range<PhysAddr>; 8],
    m_phys_regions_ranges_count: usize
}

impl BootMemAreas /* Methods */ {
    /**
     * Inserts at the end a new `BootMemArea`
     */
    pub fn push_area(&mut self, phys_region_range: Range<PhysAddr>) {
        self.m_phys_regions_ranges[self.m_phys_regions_ranges_count] = phys_region_range;
        self.m_phys_regions_ranges_count += 1;
    }
}

impl BootMemAreas /* Getters */ {
    /**
     * Returns an `Iterator` to all the `BootMemArea`
     */
    pub fn iter(&self) -> impl Iterator<Item = &Range<PhysAddr>> {
        self.m_phys_regions_ranges[..self.m_phys_regions_ranges_count].iter()
    }
}

impl Default for BootMemAreas {
    fn default() -> Self {
        Self { m_phys_regions_ranges: [Range::default(),
                                       Range::default(),
                                       Range::default(),
                                       Range::default(),
                                       Range::default(),
                                       Range::default(),
                                       Range::default(),
                                       Range::default()], /* TODO range is not Copy */
               m_phys_regions_ranges_count: 0 }
    }
}
