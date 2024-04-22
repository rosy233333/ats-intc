//! This is the driver of ats-intc
//!

#![no_std]
#![feature(const_nonnull_new)]
#![feature(const_option)]
#![feature(waker_getters)]
#![deny(missing_docs)]
extern crate alloc;

mod task;
mod waker;
mod simul;

#[cfg(feature = "svd2rust-impl")]
use ats_intc_pac::*;
pub use task::*;
use waker::*;

/// The Async Task Scheduler and Interrupt controller
#[cfg(feature = "svd2rust-impl")]
#[derive(Debug, Clone, Copy)]
pub struct AtsIntc(usize);

#[cfg(feature = "svd2rust-impl")]
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

/// The Async Task Scheduler and Interrupt controller
#[cfg(feature = "simple-impl")]
#[derive(Debug, Clone, Copy)]
pub struct AtsIntc(usize);

#[cfg(feature = "simple-impl")]
impl AtsIntc {
    ///
    pub const fn new(base_addr: usize) -> Self {
        Self(base_addr)
    }

    /// Push a task into the prority scheduler
    pub fn ps_push(&self, task: TaskRef, priority: usize) {
        assert!(priority < 255);
        let waddr: *mut usize = (self.0 + 8 + priority * 8) as *mut usize;
        unsafe {
            waddr.write(task.as_ptr() as usize);
        }
    }

    /// Fetch the task who has the highest priority from the hardware.
    pub fn ps_fetch(&self) -> Option<TaskRef> {
        let raddr: *const usize = self.0 as *const usize;
        // log::error!("raddr: {:x}", raddr as usize);
        let raw_task = unsafe {
            raddr.read()
        };
        // log::error!("raw_task: {:x}", raw_task);
        if raw_task != 0 {
            Some(unsafe { TaskRef::from_ptr(raw_task as *const Task) })
        } else {
            None
        }
    }

    /// Push a interrupt handler into the interrupt controller
    pub fn intr_push(&self, irq: usize, handler: TaskRef) {
        assert!(irq < 224);
        let waddr: *mut usize = (self.0 + 0x900 + irq * 8) as *mut usize;
        unsafe {
            waddr.write(handler.as_ptr() as usize);
        }
    }
}

#[cfg(feature = "software-impl")]
use simul::SimulInner;
#[cfg(feature = "software-impl")]
use core::cell::UnsafeCell;

#[cfg(feature = "software-impl")]
#[derive(Debug)]
/// The Async Task Scheduler and Interrupt controller, simulated bt software
pub struct AtsIntc(UnsafeCell<SimulInner>);

#[cfg(feature = "software-impl")]
impl AtsIntc {
    ///
    pub const fn new(base_addr: usize) -> Self{
        Self(UnsafeCell::new(SimulInner::new()))
    }

    /// Push a task into the prority scheduler
    pub fn ps_push(&self, task: TaskRef, priority: usize) {
        unsafe {
            (*self.0.get()).ps_push(task, priority)
        }
    }

    /// Fetch the task who has the highest priority from the scheduler
    pub fn ps_fetch(&self) -> Option<TaskRef> {
        unsafe {
            (*self.0.get()).ps_fetch()
        }
    }

    /// Push a interrupt handler into the interrupt controller
    pub fn intr_push(&self, irq: usize, handler: TaskRef) {
        unsafe {
            (*self.0.get()).intr_push(irq, handler)
        }
    }

    /// Handle an interrupt
    pub fn intr_handle(&self, irq: usize) {
        unsafe {
            (*self.0.get()).intr_handle(irq)
        }
    }
}