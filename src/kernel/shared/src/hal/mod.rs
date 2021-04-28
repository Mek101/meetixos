pub mod addr;
pub mod boot_infos;
#[cfg(not(feature = "loader_stage"))]
pub mod interrupt;
pub mod paging;
pub mod uart;

mod arch;
