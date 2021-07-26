/*! Kernel boot information */

use alloc::{
    string::String,
    vec::Vec
};
use core::{
    ops::Range,
    str::FromStr
};

use crate::{
    addr::phys_addr::PhysAddr,
    arch::hw_boot_info::HwBootInfo
};

/* Global BootInfo instance which will live for all the kernel's life */
static mut SM_BOOT_INFORMATION: Option<BootInfo> = None;

/**
 * Bootloader & architecture independent information useful for kernel
 * booting
 */
pub struct BootInfo {
    m_boot_loader_name: String,
    m_cmd_line_args_buf: String,
    m_boot_mem_areas: Vec<Range<PhysAddr>>
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

        /* store the global instance of the BootInfo */
        unsafe {
            SM_BOOT_INFORMATION =
                Some(Self { m_boot_loader_name:
                                String::from(hw_boot_info.boot_loader_name()),
                            m_cmd_line_args_buf:
                                String::from(hw_boot_info.cmd_line_args()),
                            m_boot_mem_areas: hw_boot_info.phys_mem_ranges() });
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
    pub fn boot_loader_name(&self) -> &String {
        &self.m_boot_loader_name
    }

    /**
     * Returns the command-line arguments given by the bootloader
     */
    pub fn cmd_line_args(&self) -> &String {
        &self.m_cmd_line_args_buf
    }

    /**
     * Returns the `Vec` of physical memory regions
     */
    pub fn boot_mem_areas(&self) -> &Vec<Range<PhysAddr>> {
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
    fn phys_mem_ranges(&self) -> Vec<Range<PhysAddr>>;
}
