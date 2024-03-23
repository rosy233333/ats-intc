//! Coroutine Control Block structures for more control.
//!

use crate::{new_waker, AtsIntc};
use alloc::{boxed::Box, sync::Arc};
use core::{
    future::Future,
    pin::Pin,
    ptr::NonNull,
    sync::atomic::{AtomicU32, Ordering},
    task::{Context, Poll},
};
use crossbeam::atomic::AtomicCell;

///
#[repr(u32)]
pub enum TaskState {
    ///
    Ready = 1 << 0,
    ///
    Running = 1 << 1,
    ///
    Pending = 1 << 2,
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
    /// Create a virtual task
    pub const unsafe fn const_task(ptr: usize) -> Self {
        Self {
            ptr: NonNull::new(ptr as *mut Task).unwrap()
        }
    }

    /// Safety: The pointer must have been obtained with `Task::as_ptr`
    pub unsafe fn from_ptr(ptr: *const Task) -> Self {
        Self {
            ptr: NonNull::new(ptr as *mut Task).unwrap(),
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
    pub atsintc: &'static AtsIntc,
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
        atsintc: &'static AtsIntc,
    ) -> TaskRef {
        let task = Arc::new(Self {
            state: AtomicU32::new(TaskState::Ready as _),
            priority: AtomicU32::new(priority),
            task_type,
            atsintc,
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
    pub fn poll_inner(self: &Arc<Self>) -> Poll<i32> {
        let waker = unsafe { new_waker(self.clone().as_ref(), self.atsintc) };
        let mut context = Context::from_waker(&waker);
        unsafe { (&mut *self.fut.as_ptr()).as_mut().poll(&mut context) }
    }
}
