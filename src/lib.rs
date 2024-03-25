//! This is the driver of ats-intc
//!

#![no_std]
#![feature(const_nonnull_new)]
#![feature(const_option)]
#![deny(missing_docs)]
extern crate alloc;

#[cfg(not(feature = "task-as-usize"))]
mod task;
#[cfg(not(feature = "task-as-usize"))]
mod waker;

use ats_intc_pac::*;
#[cfg(not(feature = "task-as-usize"))]
pub use task::*;
#[cfg(not(feature = "task-as-usize"))]
use waker::*;

/// The Async Task Scheduler and Interrupt controller
#[derive(Debug, Clone, Copy)]
pub struct AtsIntc(usize);

impl AtsIntc {
    ///
    pub const fn new(base_addr: usize) -> Self {
        Self(base_addr)
    }

    /// the mmio registers
    fn regs(&self) -> &'static atsintc::Process {
        unsafe { &*(self.0 as *const _) }
    }

    /// Push a task into the prority scheduler
    #[cfg(not(feature = "task-as-usize"))]
    pub fn ps_push(&self, task: TaskRef, priority: usize) {
        self.regs()
            .ats()
            .enqueue(priority)
            .write(|w| unsafe { w.bits(task.as_ptr() as _) })
    }

    /// Push a task into the prority scheduler
    #[cfg(feature = "task-as-usize")]
    pub fn ps_push(&self, task: usize, priority: usize) {
        self.regs()
            .ats()
            .enqueue(priority)
            .write(|w| unsafe { w.bits(task as _) })
    }

    /// Fetch the task who has the highest priority from the hardware.
    #[cfg(not(feature = "task-as-usize"))]
    pub fn ps_fetch(&self) -> Option<TaskRef> {
        let raw_task = self.regs().ats().dequeue().read().bits();
        if raw_task != 0 {
            Some(unsafe { TaskRef::from_ptr(raw_task as *const Task) })
        } else {
            None
        }
    }

    /// Fetch the task who has the highest priority from the hardware.
    #[cfg(feature = "task-as-usize")]
    pub fn ps_fetch(&self) -> Option<usize> {
        let raw_task = self.regs().ats().dequeue().read().bits();
        if raw_task != 0 {
            Some(raw_task as usize)
        } else {
            None
        }
    }

    /// Push a interrupt handler into the interrupt controller
    #[cfg(not(feature = "task-as-usize"))]
    pub fn intr_push(&self, irq: usize, handler: TaskRef) {
        self.regs()
            .intc()
            .intrenqueue(irq)
            .write(|w| unsafe { w.bits(handler.as_ptr() as _) })
    }

    /// Push a interrupt handler into the interrupt controller
    #[cfg(feature = "task-as-usize")]
    pub fn intr_push(&self, irq: usize, handler: usize) {
        self.regs()
            .intc()
            .intrenqueue(irq)
            .write(|w| unsafe { w.bits(handler as _) })
    }
}
