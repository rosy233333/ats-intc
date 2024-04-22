/// This mod defines the structures related to `executor`.
///

mod queue;

use super::task::*;

use alloc::collections::BTreeMap;
/// pub use priority::PRIO_LEVEL;
#[cfg(all(any(feature = "prio-level-4", feature = "prio-level-8", feature = "prio-level-16"), any(feature = "locked-simul", feature = "lock-free-simul")))]
pub use priority::PRIO_LEVEL;

///
mod priority {
    #[cfg(feature = "prio-level-4")]
    ///
    pub const PRIO_LEVEL: usize = 4;
    #[cfg(feature = "prio-level-8")]
    ///
    pub const PRIO_LEVEL: usize = 8;
    #[cfg(feature = "prio-level-16")]
    ///
    pub const PRIO_LEVEL: usize = 16;
}

use queue::*;

#[cfg(all(any(feature = "prio-level-4", feature = "prio-level-8", feature = "prio-level-16"), any(feature = "locked-simul", feature = "lock-free-simul")))]
pub struct SimulInner {
    run_queue: [Queue; PRIO_LEVEL],
    handler_queue: BTreeMap<usize, Queue>,
}

#[cfg(all(any(feature = "prio-level-4", feature = "prio-level-8", feature = "prio-level-16"), any(feature = "locked-simul", feature = "lock-free-simul")))]
impl SimulInner {
    pub const fn new() -> Self {
        Self {
            run_queue: [Queue::EMPTY; PRIO_LEVEL],
            handler_queue: BTreeMap::new(),
        }
    }

    pub fn ps_push(&mut self, task: TaskRef, priority: usize) {
        assert!(priority < PRIO_LEVEL);
        self.run_queue[priority].enqueue(task);
    }

    pub fn ps_fetch(&mut self) -> Option<TaskRef> {
        let mut return_task: Option<TaskRef> = None;
        for i in 0 .. PRIO_LEVEL {
            if let Some(task) = self.run_queue[i].dequeue() {
                return_task = Some(task);
                break;
            }
        }
        return_task
    }

    pub fn intr_push(&mut self, irq: usize, handler: TaskRef) {
        // self.handler_queue.entry(irq)
        //     .and_modify(|queue| {
        //         queue.enqueue(handler);
        //     })
        //     .or_insert_with(|| {
        //         let mut queue: Queue = Queue::EMPTY;
        //         queue.enqueue(handler);
        //         queue
        //     });
        if let Some(queue) = self.handler_queue.get_mut(&irq) {
            queue.enqueue(handler);
        }
        else {
            let mut queue: Queue = Queue::EMPTY;
            queue.enqueue(handler);
            self.handler_queue.insert(irq, queue);
        }
    }

    pub fn intr_handle(&mut self, irq: usize) {
        if let Some(queue) = self.handler_queue.get_mut(&irq) {
            if let Some(task) = queue.dequeue() {
                self.run_queue[0].enqueue(task);
            }
        }
    }
}