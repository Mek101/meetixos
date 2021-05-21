/*! Page directory flags */

use crate::mem::paging::table::PTFlags;

/**
 * Page directory mapping flags using functional construction method
 */
#[derive(Debug, Copy, Clone)]
pub struct PDirFlags {
    m_pt_flags: PTFlags,
    m_remap: bool
}

impl PDirFlags {
    /**
     * Constructs an empty `PDirFlags`
     */
    pub fn new() -> Self {
        Self { m_pt_flags: PTFlags::empty(),
               m_remap: false }
    }

    /**
     * Enables the `PTFlags::PRESENT` flag
     */
    pub fn set_present(&mut self) -> &mut Self {
        self.m_pt_flags |= PTFlags::PRESENT;
        self
    }

    /**
     * Enables the `PTFlags::READABLE` flag
     */
    pub fn set_readable(&mut self) -> &mut Self {
        self.m_pt_flags |= PTFlags::READABLE;
        self
    }

    /**
     * Enables the `PTFlags::WRITEABLE` flag
     */
    pub fn set_writeable(&mut self) -> &mut Self {
        self.m_pt_flags |= PTFlags::WRITEABLE;
        self
    }

    /**
     * Enables the `PTFlags::GLOBAL` flag
     */
    pub fn set_global(&mut self) -> &mut Self {
        self.m_pt_flags |= PTFlags::GLOBAL;
        self
    }

    /**
     * Enables the `PTFlags::HUGE_PAGE` flag
     */
    pub fn set_huge_page(&mut self) -> &mut Self {
        self.m_pt_flags |= PTFlags::HUGE_PAGE;
        self
    }

    /**
     * Enables the `PTFlags::NO_EXECUTE` flag
     */
    pub fn set_no_execute(&mut self) -> &mut Self {
        self.m_pt_flags |= PTFlags::NO_EXECUTE;
        self
    }

    /**
     * Enables the `PTFlags::USER` flag
     */
    pub fn set_user(&mut self) -> &mut Self {
        self.m_pt_flags |= PTFlags::USER;
        self
    }

    /**
     * Forces the remapping of the range updating the flags
     */
    pub fn set_remap(&mut self) -> &mut Self {
        self.m_remap = true;
        self
    }

    /**
     * Constructs the instance cloning the active bits
     */
    pub fn build(&mut self) -> Self {
        self.clone()
    }

    /**
     * Returns the `PTFlags`
     */
    pub fn page_table_flags(&self) -> PTFlags {
        self.m_pt_flags
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlags::PRESENT`
     */
    pub fn is_present(&self) -> bool {
        self.m_pt_flags.contains(PTFlags::PRESENT)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlags::READABLE`
     */
    pub fn is_readable(&self) -> bool {
        self.m_pt_flags.contains(PTFlags::READABLE)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlags::WRITEABLE`
     */
    pub fn is_writeable(&self) -> bool {
        self.m_pt_flags.contains(PTFlags::WRITEABLE)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlags::GLOBAL`
     */
    pub fn is_global(&self) -> bool {
        self.m_pt_flags.contains(PTFlags::GLOBAL)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlags::HUGE_PAGE`
     */
    pub fn is_huge_page(&self) -> bool {
        self.m_pt_flags.contains(PTFlags::HUGE_PAGE)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlags::NO_EXECUTE`
     */
    pub fn is_no_execute(&self) -> bool {
        self.m_pt_flags.contains(PTFlags::NO_EXECUTE)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlags::USER`
     */
    pub fn is_user(&self) -> bool {
        self.m_pt_flags.contains(PTFlags::USER)
    }

    /**
     * Returns whether the remapping was enabled
     */
    pub fn is_remap(&self) -> bool {
        self.m_remap
    }
}
