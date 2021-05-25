/*! Higher half loader information management */

use shared::{
    info::{
        args::CmdLineArgs,
        descriptor::{
            LoaderInfo,
            BOOTLOADER_NAME_LEN_MAX
        }
    },
    os::str_utils
};

/* initialized by <cmdline_info_init()> */
static mut CMDLINE_INFO: Option<CmdLineInfo> = None;

/**
 * Initializes the global `CmdLineInfo` instance from the given `LoaderInfo`
 */
pub fn cmdline_info_init(loader_info: &LoaderInfo) {
    let cmdline_info = CmdLineInfo::from_loader_info(loader_info);

    /* move the instance to the global field */
    unsafe {
        CMDLINE_INFO = Some(cmdline_info);
    }
}

/**
 * Returns the reference to the global `CmdLineInfo` instance
 */
pub fn cmdline_info() -> &'static CmdLineInfo {
    unsafe {
        assert!(CMDLINE_INFO.is_some());

        CMDLINE_INFO.as_ref().unwrap()
    }
}

/**
 * String information from `LoaderInfo`
 */
pub struct CmdLineInfo {
    m_cmdline_args: CmdLineArgs,
    m_bootloader_name: [u8; BOOTLOADER_NAME_LEN_MAX],
    m_bootloader_name_len: usize
}

impl CmdLineInfo {
    /**
     * Constructs a `CmdLineInfo` from the `LoaderInfo` given
     */
    fn from_loader_info(loader_info: &LoaderInfo) -> Self {
        let mut name_buffer = [0; BOOTLOADER_NAME_LEN_MAX];
        str_utils::copy_str_to_u8_buf(&mut name_buffer, loader_info.bootloader_name());

        Self { m_cmdline_args: loader_info.cmdline_args().clone(),
               m_bootloader_name: name_buffer,
               m_bootloader_name_len: loader_info.bootloader_name().len() }
    }

    /**
     * Returns the `CmdLineArgs` collection
     */
    pub fn cmdline_args(&self) -> &CmdLineArgs {
        &self.m_cmdline_args
    }

    /**
     * Returns the bootloader name string
     */
    pub fn bootloader_name(&self) -> &str {
        let name_slice = &self.m_bootloader_name[..self.m_bootloader_name_len];
        str_utils::u8_slice_to_str_slice(name_slice)
    }
}
