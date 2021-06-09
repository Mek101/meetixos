/*! Lock guards helpers */

/**
 * Marker type which indicates that the Guard type for a lock is `Send`
 */
pub struct LockGuardSendable(());

impl LockGuardShareability for LockGuardSendable {
    /* No methods, just a marker trait */
}

/**
 * Marker type which indicates that the Guard type for a lock is not `Send`
 */
pub struct LockGuardNonSendable(*mut ());

impl LockGuardShareability for LockGuardNonSendable {
    /* No methods, just a marker trait */
}

unsafe impl Sync for LockGuardNonSendable {
    /* No methods, just a marker trait */
}

pub trait LockGuardShareability {
    /* No methods, just a marker trait */
}
