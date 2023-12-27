//! This mod specific the waker related with coroutine
//!

use super::task::{Task, TaskRef};
use core::task::{RawWaker, RawWakerVTable, Waker};

const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop);

unsafe fn clone(p: *const ()) -> RawWaker {
    RawWaker::new(p, &VTABLE)
}

/// nop
unsafe fn wake(p: *const ()) { }

unsafe fn drop(_p: *const ()) {
    // nop
}

/// 
pub unsafe fn from_task(task_ref: TaskRef) -> Waker {
    Waker::from_raw(RawWaker::new(task_ref.as_ptr() as _, &VTABLE))
}
