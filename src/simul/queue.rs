use crate::Task;

/// This mod defines some queue in the `Executor`
use super::TaskRef;

#[cfg(feature = "locked-simul")]
// use crossbeam::queue::SegQueue;
use heapless::mpmc::MpMcQueue;

#[cfg(feature = "locked-simul")]
/// This queue stores the `TaskRef` which is ready to run.
#[repr(transparent)]
pub struct Queue(MpMcQueue<usize, 128>);

#[cfg(feature = "locked-simul")]
impl Queue {
    pub const EMPTY: Self = Self(MpMcQueue::new());
    ///
    #[allow(unused)]
    pub const fn new() -> Self {
        Self(MpMcQueue::new())
    }

    ///
    #[inline(always)]
    pub fn dequeue(&self) -> Option<TaskRef> {
        unsafe {
            self.0.dequeue().map(|task_ptr|{
                TaskRef::from_ptr(task_ptr as *const Task)
            })
        }
    }

    ///
    #[inline(always)]
    pub fn enqueue(&self, task_ref: TaskRef) {
        while self.0.enqueue(task_ref.as_ptr() as usize).is_err() {}
    }
}

#[cfg(feature = "lock-free-simul")]
// use crossbeam::queue::SegQueue;
use heapless::Vec;

#[cfg(feature = "lock-free-simul")]
/// This queue stores the `TaskRef` which is ready to run.
#[repr(transparent)]
pub struct Queue(Vec<usize, 128>);

#[cfg(feature = "lock-free-simul")]
impl Queue {
    pub const EMPTY: Self = Self(Vec::new());
    ///
    #[allow(unused)]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    ///
    #[inline(always)]
    pub fn dequeue(&mut self) -> Option<TaskRef> {
        unsafe {
            self.0.pop().map(|task_ptr| {
                TaskRef::from_ptr(task_ptr as *const Task)
            })
        }
    }

    ///
    #[inline(always)]
    pub fn enqueue(&mut self, task_ref: TaskRef) {
        while self.0.push(task_ref.clone().as_ptr() as usize).is_err() {}
    }
}