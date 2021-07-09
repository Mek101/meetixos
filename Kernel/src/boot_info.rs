/*! Kernel boot information */

use helps::str::{
    copy_str_to_u8_buf,
    u8_slice_to_str_slice
};

use crate::{
    addr::{
        phys_addr::PhysAddr,
        Address
    },
    arch::hw_boot_info::HwBootInfo
};

static mut SM_BOOT_INFORMATION: Option<BootInfo> = None;

pub struct BootInfo {
    m_boot_loader_name_buf: [u8; 64],
    m_boot_loader_name_buf_len: usize,

    m_cmd_line_args_buf: [u8; 1024],
    m_cmd_line_args_buf_len: usize,

    m_boot_mem_areas: BootMemAreas
}

impl BootInfo {
    pub fn init_instance(raw_boot_info_ptr: *const u8) {
        /* this function cannot be called more than one time */
        unsafe {
            assert!(SM_BOOT_INFORMATION.is_none(),
                    "Called BootInfo::init_instance() more than one time");
        }

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

    pub fn instance() -> &'static BootInfo {
        unsafe {
            SM_BOOT_INFORMATION.as_ref().expect("Called BootInfo::instance() before \
                                                 BootInfo::init_instance()")
        }
    }

    pub fn boot_loader_name(&self) -> &str {
        u8_slice_to_str_slice(&self.m_boot_loader_name_buf
                                  [..self.m_boot_loader_name_buf_len])
    }

    pub fn cmd_line_args(&self) -> &str {
        u8_slice_to_str_slice(&self.m_cmd_line_args_buf[..self.m_cmd_line_args_buf_len])
    }

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

    pub fn boot_mem_areas(&self) -> &BootMemAreas {
        &self.m_boot_mem_areas
    }
}

impl From<*const u8> for BootInfo {
    fn from(raw_boot_info_ptr: *const u8) -> Self {
        /* this function cannot be called more than one time */
        unsafe {
            assert!(SM_BOOT_INFORMATION.is_none());
        }

        let hw_boot_info = HwBootInfo::from(raw_boot_info_ptr);

        let mut boot_loader_name_buf = [0; 64];
        copy_str_to_u8_buf(&mut boot_loader_name_buf, hw_boot_info.boot_loader_name());

        let mut cmd_line_buf = [0; 1024];
        copy_str_to_u8_buf(&mut cmd_line_buf, hw_boot_info.cmd_line_args());

        Self { m_boot_loader_name_buf: boot_loader_name_buf,
               m_boot_loader_name_buf_len: hw_boot_info.boot_loader_name().len(),
               m_cmd_line_args_buf: cmd_line_buf,
               m_cmd_line_args_buf_len: hw_boot_info.cmd_line_args().len(),
               m_boot_mem_areas: hw_boot_info.mem_areas() }
    }
}

pub trait HwBootInfoBase: From<*const u8> {
    fn boot_loader_name(&self) -> &str;
    fn cmd_line_args(&self) -> &str;
    fn mem_areas(&self) -> BootMemAreas;
}

#[derive(Default)]
#[derive(Copy, Clone)]
pub struct BootMemArea {
    m_start_addr: PhysAddr,
    m_size: usize
}

impl BootMemArea {
    pub fn new(start_addr: PhysAddr, size: usize) -> Self {
        Self { m_start_addr: start_addr,
               m_size: size }
    }

    pub fn start_addr(&self) -> PhysAddr {
        self.m_start_addr
    }

    pub fn size(&self) -> usize {
        self.m_size
    }

    pub fn end_addr(&self) -> PhysAddr {
        self.start_addr().offset(self.size() as isize)
    }
}

#[derive(Copy, Clone)]
pub struct BootMemAreas {
    m_mem_areas: [BootMemArea; 64],
    m_mem_areas_count: usize
}

impl BootMemAreas {
    pub fn push_area(&mut self, boot_mem_area: BootMemArea) {
        self.m_mem_areas[self.m_mem_areas_count] = boot_mem_area;
        self.m_mem_areas_count += 1;
    }

    pub fn iter(&self) -> impl Iterator<Item = &BootMemArea> {
        self.m_mem_areas[..self.m_mem_areas_count].iter()
    }
}

impl Default for BootMemAreas {
    fn default() -> Self {
        Self { m_mem_areas: [BootMemArea::default(); 64],
               m_mem_areas_count: 0 }
    }
}
