//! Coroutine Control Block structures for more control.
//!


use alloc::{boxed::Box, sync::Arc};
use core::{
    borrow::BorrowMut, future::Future, pin::Pin, ptr::NonNull, sync::atomic::{AtomicU32, Ordering}, task::{Context, Poll}
};
use crossbeam::atomic::AtomicCell;

use crate::from_task;

/// 
#[repr(u32)]
pub enum TaskState {
    ///
    Ready = 1 << 0,
    /// 
    Running = 1 << 1,
    ///
    Pending = 1 << 2
}


/// The pointer of `Task`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskRef {
    ptr: NonNull<Task>,
}

unsafe impl Send for TaskRef {}
unsafe impl Sync for TaskRef {}

impl TaskRef {

    /// Safety: The pointer must have been obtained with `Task::as_ptr`
    pub unsafe fn from_ptr(ptr: *const Task) -> Self {
        Self {
            ptr: NonNull::new_unchecked(ptr as *mut Task),
        }
    }

    /// The returned pointer
    pub fn as_ptr(self) -> *const Task {
        self.ptr.as_ptr()
    }
}

///
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskType {
    ///
    KernelSche,
    ///
    KernelProcess,
    ///
    Process,
    ///
    Syscall,
    /// 
    AsyncSyscall,
    ///
    Other,
}

/// The `Task` is stored in heap by using `Arc`.
#[repr(C)]
pub struct Task {
    /// 
    pub state: AtomicU32,
    ///
    pub priority: AtomicU32,
    ///
    pub task_type: TaskType,
    /// 
    pub fut: AtomicCell<Pin<Box<dyn Future<Output = i32> + 'static + Send + Sync>>>,
}

impl Task {
    /// Create a new Task, in not-spawned state.
    pub fn new(
        fut: Pin<Box<dyn Future<Output = i32> + 'static + Send + Sync>>,
        priority: u32,
        task_type: TaskType,
    ) -> TaskRef {
        let task = Arc::new(Self {
            state: AtomicU32::new(TaskState::Ready as _),
            priority: AtomicU32::new(priority),
            task_type,
            fut: AtomicCell::new(fut),
        });
        task.as_ref()
    }

    /// Update priority
    pub fn update_priority(&self, new_priority: u32) {
        self.priority.store(new_priority, Ordering::Relaxed);
    }

    ///
    pub fn as_ref(self: Arc<Self>) -> TaskRef {
        unsafe { TaskRef::from_ptr(Arc::into_raw(self)) }
    }

    /// 
    pub fn from_ref(task_ref: TaskRef) -> Arc<Self> {
        let raw_ptr = task_ref.as_ptr();
        unsafe { Arc::from_raw(raw_ptr) }
    }

    /// poll the inner future
    pub fn poll_inner(self: Arc<Self>) -> Poll<i32> {
        let waker = unsafe { from_task(self.as_ref()) };
        let mut context = Context::from_waker(&waker);
        (self.fut.as_ptr() as &mut Pin<Box<dyn Future<Output = i32> + 'static + Send + Sync>>).as_mut().poll(&mut context)
    }
}

