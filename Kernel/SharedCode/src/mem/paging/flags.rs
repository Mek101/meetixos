/*! Page directory flags */

use bits::flags::BitFlags;

use crate::mem::paging::table::PTFlagsBits;

/**
 * Page directory mapping flags using functional construction method
 */
#[derive(Debug, Copy, Clone)]
pub struct PDirFlags {
    m_pt_flags: BitFlags<usize, PTFlagsBits>,
    m_remap: bool
}

impl PDirFlags {
    /**
     * Constructs an empty `PDirFlags`
     */
    pub fn new() -> Self {
        Self { m_pt_flags: BitFlags::new_zero(),
               m_remap: false }
    }

    /**
     * Enables the `PTFlagsBits::Present` flag
     */
    pub fn set_present(&mut self) -> &mut Self {
        self.m_pt_flags.set_enabled(PTFlagsBits::Present);
        self
    }

    /**
     * Enables the `PTFlagsBits::Readable` flag
     */
    pub fn set_readable(&mut self) -> &mut Self {
        self.m_pt_flags.set_enabled(PTFlagsBits::Readable);
        self
    }

    /**
     * Enables the `PTFlagsBits::Writeable` flag
     */
    pub fn set_writeable(&mut self) -> &mut Self {
        self.m_pt_flags.set_enabled(PTFlagsBits::Writeable);
        self
    }

    /**
     * Enables the `PTFlagsBits::Global` flag
     */
    pub fn set_global(&mut self) -> &mut Self {
        self.m_pt_flags.set_enabled(PTFlagsBits::Global);
        self
    }

    /**
     * Enables the `PTFlagsBits::HugePage` flag
     */
    pub fn set_huge_page(&mut self) -> &mut Self {
        self.m_pt_flags.set_enabled(PTFlagsBits::HugePage);
        self
    }

    /**
     * Enables the `PTFlagsBits::NoExecute` flag
     */
    pub fn set_no_execute(&mut self) -> &mut Self {
        self.m_pt_flags.set_enabled(PTFlagsBits::NoExecute);
        self
    }

    /**
     * Enables the `PTFlagsBits::User` flag
     */
    pub fn set_user(&mut self) -> &mut Self {
        self.m_pt_flags.set_enabled(PTFlagsBits::User);
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
    pub fn page_table_flags(&self) -> BitFlags<usize, PTFlagsBits> {
        self.m_pt_flags
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlagsBits::Present`
     */
    pub fn is_present(&self) -> bool {
        self.m_pt_flags.is_enabled(PTFlagsBits::Present)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlagsBits::Readable`
     */
    pub fn is_readable(&self) -> bool {
        self.m_pt_flags.is_enabled(PTFlagsBits::Readable)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlagsBits::Writeable`
     */
    pub fn is_writeable(&self) -> bool {
        self.m_pt_flags.is_enabled(PTFlagsBits::Writeable)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlagsBits::Global`
     */
    pub fn is_global(&self) -> bool {
        self.m_pt_flags.is_enabled(PTFlagsBits::Global)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlagsBits::HugePage`
     */
    pub fn is_huge_page(&self) -> bool {
        self.m_pt_flags.is_enabled(PTFlagsBits::HugePage)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlagsBits::NoExecute`
     */
    pub fn is_no_execute(&self) -> bool {
        self.m_pt_flags.is_enabled(PTFlagsBits::NoExecute)
    }

    /**
     * Returns whether the `PTFlags` contains `PTFlagsBits::User`
     */
    pub fn is_user(&self) -> bool {
        self.m_pt_flags.is_enabled(PTFlagsBits::User)
    }

    /**
     * Returns whether the remapping was enabled
     */
    pub fn is_remap(&self) -> bool {
        self.m_remap
    }
}
