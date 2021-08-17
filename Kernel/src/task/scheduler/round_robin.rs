/*! Kernel prioritized Round Robin Scheduler */

use alloc::{
    collections::LinkedList,
    sync::Arc
};

use crate::task::{
    scheduler::TScheduler,
    thread::Thread
};

pub struct RoundRobinScheduler {
    m_ready_to_run: LinkedList<Arc<Thread>>
}

impl RoundRobinScheduler /* Constructors */ {
    pub const fn new() -> Self {
        Self { m_ready_to_run: LinkedList::new() }
    }
}

impl TScheduler for RoundRobinScheduler {
    fn add_thread(&mut self, thread: Arc<Thread>) {
        self.m_ready_to_run.push_back(thread);
    }

    fn pick_next(&mut self) -> Option<Arc<Thread>> {
        self.m_ready_to_run.front().map(|thread| thread.clone())
    }
}
