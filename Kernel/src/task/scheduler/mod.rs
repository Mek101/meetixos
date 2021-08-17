/*! Kernel Scheduler manager */

use alloc::{
    boxed::Box,
    sync::Arc,
    vec::Vec
};

use crate::task::{
    scheduler::{
        real_time::RealTimeScheduler,
        round_robin::RoundRobinScheduler
    },
    thread::Thread
};

pub mod real_time;
pub mod round_robin;

static mut SM_SCHEDULER: Scheduler = Scheduler { m_schedulers: Vec::new() };

pub struct Scheduler {
    m_schedulers: Vec<Box<dyn TScheduler>>
}

impl Scheduler /* Constructors */ {
    pub fn init_instance() {
        unsafe {
            SM_SCHEDULER.m_schedulers.push(Box::new(RoundRobinScheduler::new()));
            SM_SCHEDULER.m_schedulers.push(Box::new(RealTimeScheduler::new()));
        }
    }
}

impl Scheduler /* Getters */ {
    pub fn instance() -> &'static Self {
        unsafe { &SM_SCHEDULER }
    }
}

pub trait TScheduler {
    fn add_thread(&mut self, thread: Arc<Thread>);
    fn pick_next(&mut self) -> Option<Arc<Thread>>;
}
