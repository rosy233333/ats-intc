/// This mod is the `ats` driver.
/// 


#[cfg(feature = "no-simul")]
const ATS_BASE_ADDR: usize = 0xf000000;

const ATS_REGSPACE_LEN: usize = 0x1000;


#[cfg(any(feature = "locked-simul", feature = "lock-free-simul"))]
mod simul {
    use spin::{Lazy, Mutex};
    use crate::{executor::Executor, handler::Handler};
    extern "C" {
        fn cpu_num() -> usize;
    }
    pub static EXECUTOR: Lazy<Mutex<Executor>> = Lazy::new(|| Mutex::new(Executor::new()));
    pub static HANDLER: Lazy<Mutex<Handler>> = Lazy::new(|| Mutex::new(Handler::new(unsafe { cpu_num() })));
}


use ats_intc_pac::*;

use super::{TaskRef, Task};

#[derive(Debug)]
pub struct AtsDriver(usize);

impl AtsDriver {
    #[cfg(any(feature = "locked-simul", feature = "lock-free-simul", feature = "no-simul"))]
    pub fn new(base_addr: usize) -> Self {
        Self(base_addr)
    }

    // #[cfg(feature = "no-simul")]
    // pub fn new() -> Self {
    //     Self(ATS_BASE_ADDR)
    // }

    /// the mmio registers 
    fn regs(&self) -> &'static atsintc::RegisterBlock {
        unsafe { &*(self.0 as *const _) }
    }

    /// When software reset the hardware, it will set the `reset` bit in the `control` register.
    /// It will busy wait the `reset` bit until it's clear.
    pub fn reset(&self) {
        self.regs().control().write(|w| w.reset().set_bit());
        #[cfg(any(feature = "locked-simul", feature = "lock-free-simul"))]
        {
            // init the EXECUTOR & HANDLER
            use spin::Mutex;
            use crate::executor::Executor;
            use crate::handler::Handler;
            extern "C" {
                fn cpu_num() -> usize;
            }
            unsafe { *simul::EXECUTOR.as_mut_ptr() = Mutex::new(Executor::new()) };
            unsafe { *simul::HANDLER.as_mut_ptr() = Mutex::new(Handler::new(cpu_num())) };
            // clear the `reset` bit in the `control` register
            self.regs().control().write(|w| w.reset().clear_bit());
        }
        while !self.regs().control().read().reset().is_complete() {}
    }

    /// When software load a handler task in the hardware, It will fill the `handler` and `irq` registers.
    /// Then it will set the `lhandler` bit in the `control` register.
    pub fn load_handler(&self, irq: usize, handler: TaskRef) {
        while !self.regs().control().read().lhandler().bit_is_clear() {}
        let raw_ptr = handler.as_ptr() as usize;
        self.regs().handler_lsb().write(|w| unsafe { w.bits((raw_ptr & 0xFFFFFFFF) as _) });
        self.regs().handler_msb().write(|w| unsafe { w.bits(((raw_ptr >> 32) & 0xFFFFFFFF) as _) });
        self.regs().irq().write(|w| unsafe { w.bits(irq as _) });
        self.regs().control().write(|w| w.lhandler().set_bit());
        #[cfg(any(feature = "locked-simul", feature = "lock-free-simul"))]
        {
            // The HANDLER load the specified task
            simul::HANDLER.lock().load(irq, handler);
            // clear the `handler` register
            self.regs().handler_lsb().write(|w| unsafe { w.bits(0) });
            self.regs().handler_msb().write(|w| unsafe { w.bits(0) });
            self.regs().irq().write(|w| unsafe { w.bits(0) });
            // clear the `lhandler` bit in the `control` register
            self.regs().control().write(|w| w.lhandler().clear_bit());
        }
        while !self.regs().control().read().lhandler().is_complete() {}
    }

    /// Software store a task into the hardware executor.
    pub fn stask(&self, task: TaskRef, priority: usize) {
        while !self.regs().control().read().stask().bit_is_clear() {}
        let raw_ptr = task.as_ptr() as usize;
        self.regs().stask_lsb().write(|w| unsafe { w.bits((raw_ptr & 0xFFFFFFFF) as _) });
        self.regs().stask_msb().write(|w| unsafe { w.bits(((raw_ptr >> 32) & 0xFFFFFFFF) as _) });
        self.regs().priority().write(|w| unsafe { w.bits(priority as _) });
        self.regs().control().write(|w| w.stask().set_bit());
        #[cfg(any(feature = "locked-simul", feature = "lock-free-simul"))]
        {
            // The EXECUTOR spawn the specified task
            simul::EXECUTOR.lock().spawn(task, priority as _);
            // clear the `handler` register
            self.regs().stask_lsb().write(|w| unsafe { w.bits(0) });
            self.regs().stask_msb().write(|w| unsafe { w.bits(0) });
            self.regs().priority().write(|w| unsafe { w.bits(0) });
            // clear the `lhandler` bit in the `control` register
            self.regs().control().write(|w| w.stask().clear_bit());
        }
        while !self.regs().control().read().stask().is_complete() {}
    }

    /// Software fetch a task from the hardware executor.
    pub fn ftask(&self) -> Option<TaskRef> {
        while !self.regs().control().read().ftask().bit_is_clear() {}
        self.regs().control().write(|w| w.ftask().set_bit());
        #[cfg(any(feature = "locked-simul", feature = "lock-free-simul"))]
        {
            // The EXECUTOR fetch the specified task
            if let Some(handler) = simul::HANDLER.lock().handle(0) {
                simul::EXECUTOR.lock().spawn(handler, 0);
            }
            if let Some(task) = simul::EXECUTOR.lock().fetch() {
                let raw_ptr = task.as_ptr() as usize;
                self.regs().ftask_lsb().write(|w| unsafe { w.bits((raw_ptr & 0xFFFFFFFF) as _) });
                self.regs().ftask_msb().write(|w| unsafe { w.bits(((raw_ptr >> 32) & 0xFFFFFFFF) as _) });
            } else {
                self.regs().ftask_lsb().write(|w| unsafe { w.bits(0) });
                self.regs().ftask_msb().write(|w| unsafe { w.bits(0) });
            }
            // clear the `ftask` bit in the `control` register
            self.regs().control().write(|w| w.ftask().clear_bit());
        }
        while !self.regs().control().read().ftask().is_complete() {}
        let lsb = self.regs().ftask_lsb().read().bits() as usize;
        let msb = self.regs().ftask_msb().read().bits() as usize;
        let raw_ptr = (msb << 32) | lsb;
        if raw_ptr == 0 {
            None
        } else {
            Some(unsafe { TaskRef::from_ptr(raw_ptr as *const Task) })
        }
    }
}
