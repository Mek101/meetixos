/*! # Object Use Instant
 *
 * Implements the descriptor of a usage instant
 */

use crate::{
    bits::obj::ObjUse,
    tasks::impls::Thread,
    time::Instant
};

/** # Object Use Instant
 *
 * Contains the data that describes an usage instant related to an object
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct ObjUseInstant {
    m_obj_use: ObjUse,
    m_user: Thread,
    m_use_start: Instant
}

#[cfg(feature = "enable_kernel_methods")]
impl ObjUseInstant {
    /** # Constructs an `ObjUseInstant`
     *
     * The returned instance is filled with the given data
     */
    pub fn new(obj_use: ObjUse, user: Thread, use_start: Instant) -> Self {
        Self { m_obj_use: obj_use,
               m_user: user,
               m_use_start: use_start }
    }
}

impl ObjUseInstant {
    /** Returns the [`ObjUse`] performed by the referred user
     *
     * [`ObjUse`]: crate::bits::obj::uses::ObjUse
     */
    pub fn obj_use(&self) -> ObjUse {
        self.m_obj_use
    }

    /** Returns the [`Thread`] that have performed the usage
     *
     * [`Thread`]: crate::tasks::impls::Thread
     */
    pub fn user(&self) -> Thread {
        self.m_user
    }

    /** Returns the [`Instant`] instant of the operation
     *
     * [`Instant`]: crate::time::Instant
     */
    pub fn use_start(&self) -> Instant {
        self.m_use_start
    }
}
