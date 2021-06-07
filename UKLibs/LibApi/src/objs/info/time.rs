/*! `Object` timing information */

use crate::time::Instant;

/**
 * Stores the various `Object` timestamps
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct ObjTimeInfo {
    m_creat_inst: Instant,
    m_last_data_access_inst: Instant,
    m_last_info_access_inst: Instant,
    m_last_data_modify_inst: Instant,
    m_last_info_modify_inst: Instant
}

#[cfg(feature = "enable_kernel_methods")]
impl ObjTimeInfo {
    /**
     * Constructs a `TimeInfo` with the given parameters
     */
    pub const fn new(creat_inst: Instant,
                     last_data_access_inst: Instant,
                     last_info_access_inst: Instant,
                     last_data_modify_inst: Instant,
                     last_info_modify_inst: Instant)
                     -> Self {
        Self { m_creat_inst: creat_inst,
               m_last_data_access_inst: last_data_access_inst,
               m_last_info_access_inst: last_info_access_inst,
               m_last_data_modify_inst: last_data_modify_inst,
               m_last_info_modify_inst: last_info_modify_inst }
    }
}

impl ObjTimeInfo {
    /**
     * Returns the `Object` creation `Instant`
     */
    pub fn creat_inst(&self) -> Instant {
        self.m_creat_inst
    }

    /**
     * Sets the `Object` creation `Instant`
     */
    pub fn set_creat_inst(&mut self, new_inst: Instant) {
        self.m_creat_inst = new_inst
    }

    /**
     * Returns the `Object` last data access `Instant`
     */
    pub fn last_data_access_inst(&self) -> Instant {
        self.m_last_data_access_inst
    }

    /**
     * Sets the `Object` last data access `Instant`
     */
    pub fn set_last_data_access_inst(&mut self, new_inst: Instant) {
        self.m_last_data_access_inst = new_inst;
    }

    /**
     * Returns the `Object` last info access `Instant`
     */
    pub fn last_info_access_inst(&self) -> Instant {
        self.m_last_info_access_inst
    }

    /**
     * Sets the `Object` last info access `Instant`
     */
    pub fn set_last_info_access_inst(&mut self, new_inst: Instant) {
        self.m_last_info_access_inst = new_inst;
    }

    /**
     * Returns the `Object` last data modification `Instant`
     */
    pub fn last_data_modify_inst(&self) -> Instant {
        self.m_last_data_modify_inst
    }

    /**
     * Sets the `Object` last data modification `Instant`
     */
    pub fn set_last_data_modify_inst(&mut self, new_inst: Instant) {
        self.m_last_data_modify_inst = new_inst;
    }

    /**
     * Returns the `Object` last info modification `Instant`
     */
    pub fn last_info_modify_inst(&self) -> Instant {
        self.m_last_info_modify_inst
    }

    /**
     * Sets the `Object` last info modification `Instant`
     */
    pub fn set_last_info_modify_inst(&mut self, new_inst: Instant) {
        self.m_last_info_modify_inst = new_inst;
    }
}
