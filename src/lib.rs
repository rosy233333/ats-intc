//! This is the driver of ats-intc
//!

#![no_std]
#![deny(missing_docs)]
extern crate alloc;

mod task;
mod waker;

use ats_intc_pac::*;
pub use task::*;
pub use waker::*;

/// The Async Task Scheduler and Interrupt controller
#[derive(Debug, Clone, Copy)]
pub struct AtsIntc(usize);

impl AtsIntc {
    ///
    pub fn new(base_addr: usize) -> Self {
        Self(base_addr)
    }

    /// the mmio registers
    fn regs(&self) -> &'static atsintc::Process {
        unsafe { &*(self.0 as *const _) }
    }

    /// Push a task into the prority scheduler
    pub fn ps_push(&self, task: TaskRef, priority: usize) {
        self.regs()
            .ats()
            .enqueue(priority)
            .write(|w| unsafe { w.bits(task.as_ptr() as _) })
    }

    /// Fetch the task who has the highest priority from the hardware.
    pub fn ps_fetch(&self) -> Option<TaskRef> {
        let raw_task = self.regs().ats().dequeue().read().bits();
        if raw_task != 0 {
            Some(unsafe { TaskRef::from_ptr(raw_task as *const Task) })
        } else {
            None
        }
    }

    /// Push a interrupt handler into the interrupt controller
    pub fn intr_push(&self, irq: usize, handler: TaskRef) {
        self.regs()
            .intc()
            .intrenqueue(irq)
            .write(|w| unsafe { w.bits(handler.as_ptr() as _) })
    }
}
