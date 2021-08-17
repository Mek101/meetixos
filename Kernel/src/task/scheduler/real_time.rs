/*! Kernel real-time scheduler */

use alloc::{
    collections::LinkedList,
    sync::Arc
};

use crate::task::{
    scheduler::TScheduler,
    thread::Thread
};

pub struct RealTimeScheduler {
    m_ready_to_run: LinkedList<Arc<Thread>>
}

impl RealTimeScheduler /* Constructors */ {
    pub const fn new() -> Self {
        Self { m_ready_to_run: LinkedList::new() }
    }
}

impl TScheduler for RealTimeScheduler {
    fn add_thread(&mut self, thread: Arc<Thread>) {
        self.m_ready_to_run.push_front(thread);
    }

    fn pick_next(&mut self) -> Option<Arc<Thread>> {
        self.m_ready_to_run.back().map(|thread| thread.clone())
    }
}
