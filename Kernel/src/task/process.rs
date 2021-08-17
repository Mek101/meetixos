/*! Process management */

use alloc::{
    sync::Arc,
    vec::Vec
};

use api_data::task::TaskId;

use crate::task::thread::Thread;

pub struct Process {
    m_id: TaskId,
    m_parent_proc: Arc<Process>,
    m_threads: Vec<Arc<Thread>>
}
