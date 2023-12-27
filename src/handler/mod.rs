/// This mod defines the structures related to `handler`.

use super::TaskRef;
use alloc::{vec::Vec, collections::BTreeMap};
use rv_plic::{Priority, PLIC};

const POOL_NUM: usize = 8; 
pub const PLIC_BASE: usize = 0xffffffc00c00_0000;
pub const PLIC_PRIORITY_BIT: usize = 2;

pub type Plic = PLIC<{ PLIC_BASE }, { PLIC_PRIORITY_BIT }>;

fn get_context(hart_id: usize, mode: char) -> usize {
    const MODE_PER_HART: usize = 2;
    hart_id * MODE_PER_HART
        + match mode {
            'M' => 0,
            'S' => 1,
            _ => panic!("Wrong Mode"),
        }
}

pub struct Handler {
    pool: HandlerPool
}

impl Handler {
    /// create a new handler component
    pub fn new(cpu_num: usize) -> Self {
        // init plic
        for irq in 1..9 {
            Plic::set_priority(irq, Priority::lowest());
            for i in 0..cpu_num {
                let context = get_context(i, 'S');
                Plic::clear_enable(context, 0);
                Plic::enable(context, irq);
                Plic::set_threshold(context, Priority::any());
            }
        }
        Self{
            pool: HandlerPool::EMPTY
        }
    }

    /// This function will check wheather there is interrupt.
    /// If there is a interrupt, it will fetch the task from its inner pool.
    /// If there is no interrypt, it will do no thing
    pub fn handle(&mut self, hart_id: usize) -> Option<TaskRef> {
        // check plic
        let context = get_context(hart_id, 'S');
        if let Some(irq) = Plic::claim(context) {
            log::trace!("plic claim {}", irq);
            let handler = self.pool.fetch(irq as _);
            Plic::complete(context, irq);
            handler
        } else {
            None
        }
    }

    /// 
    pub fn load(&mut self, irq: usize, task_ref: TaskRef) {
        self.pool.load(irq, task_ref);
    }
}



pub struct HandlerPool {
    tasks: BTreeMap<usize, Vec<TaskRef>>,
}

impl HandlerPool {
    pub const EMPTY: Self = Self {
        tasks: BTreeMap::new()
    };


    pub fn load(&mut self, irq: usize, task_ref: TaskRef) {
        if let Some(tasks) = self.tasks.get_mut(&irq) {
            tasks.push(task_ref);
        } else {
            let mut irq_tasks = Vec::new();
            irq_tasks.push(task_ref);
            self.tasks.insert(irq, irq_tasks);
        }
    }

    pub fn fetch(&mut self, irq: usize) -> Option<TaskRef> {
        if let Some(tasks) = self.tasks.get_mut(&irq) {
            tasks.pop()
        } else {
            log::warn!("There is no handler related with {}", irq);
            None
        }
    }
}

