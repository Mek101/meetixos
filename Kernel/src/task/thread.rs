/*! Thread management */

use alloc::sync::Arc;

use api_data::task::TaskId;

use crate::task::process::Process;

pub struct Thread {
    m_id: TaskId,
    m_proc: Arc<Process>
}
