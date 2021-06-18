/*! `Object` usage instant */

use crate::{
    bits::obj::uses::ObjUseBits,
    tasks::impls::thread::Thread,
    time::Instant
};

/**
 * Data container with usage instant related to an `Object`
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct ObjUseInstant {
    m_obj_use: ObjUseBits,
    m_user: Thread,
    m_use_start: Instant
}

#[cfg(feature = "enable_kernel_methods")]
impl ObjUseInstant {
    /**
     * Constructs an `ObjUseInstant` with the given parameters
     */
    pub fn new(obj_use: ObjUseBits, user: Thread, use_start: Instant) -> Self {
        Self { m_obj_use: obj_use,
               m_user: user,
               m_use_start: use_start }
    }
}

impl ObjUseInstant {
    /**
     * Returns the `ObjUse` performed by the referred user
     */
    pub fn obj_use(&self) -> ObjUseBits {
        self.m_obj_use
    }

    /**
     * Returns the `Thread` that have performed the usage
     */
    pub fn user(&self) -> Thread {
        self.m_user
    }

    /**
     * Returns the `Instant` instant of the operation
     */
    pub fn use_start(&self) -> Instant {
        self.m_use_start
    }
}
