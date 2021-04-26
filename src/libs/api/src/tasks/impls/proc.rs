/**! # Process Management
 *
 * Implements the running process reference
 */
use os::sysc::{codes::KernProcFnId, fn_path::KernFnPath};

use crate::{
    bits::task::TaskType,
    caller::{KernCaller, Result},
    tasks::{impls::Thread, Task, TaskId}
};

impl_task_id_task! {
    /** # Running `Process`
     *
     * Represents a reference to context that is being executing at least
     * one [`Thread`]
     *
     * [`Thread`]: /api/tasks/impls/struct.Thread.html
     */
    pub struct Proc(TaskType::Proc);
}

impl Proc {
    /** Returns the main [`Thread`] of this process
     *
     * [`Thread`]: /api/tasks/impls/struct.Thread.html
     */
    pub fn main_thread(&self) -> Result<Thread> {
        self.kern_call_0(KernFnPath::Proc(KernProcFnId::MainThread))
            .map(|thr_id| Thread::from(TaskId::from(thr_id)))
    }
}
