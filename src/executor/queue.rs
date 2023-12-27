/// This mod defines some queue in the `Executor`
use super::TaskRef;

#[cfg(feature = "locked-simul")]
// use crossbeam::queue::SegQueue;
use heapless::mpmc::MpMcQueue;

#[cfg(feature = "locked-simul")]
/// This queue stores the `TaskRef` which is ready to run.
#[repr(transparent)]
pub struct Queue(MpMcQueue<TaskRef, 128>);

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
        self.0.dequeue()
    }

    ///
    #[inline(always)]
    pub fn enqueue(&self, task_ref: TaskRef) {
        while self.0.enqueue(task_ref).is_err() {}
    }
}

#[cfg(feature = "lock-free-simul")]
// use crossbeam::queue::SegQueue;
use heapless::Vec;

#[cfg(feature = "lock-free-simul")]
/// This queue stores the `TaskRef` which is ready to run.
#[repr(transparent)]
pub struct Queue(Vec<TaskRef, 128>);

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
        self.0.pop()
    }

    ///
    #[inline(always)]
    pub fn enqueue(&mut self, task_ref: TaskRef) {
        while self.0.push(task_ref).is_err() {}
    }
}
