#[repr(C)]
#[doc = "Async task scheduler of one process"]
#[doc(alias = "ats")]
pub struct Ats {
    dequeue: Dequeue,
    enqueue: [Enqueue; 255],
}
impl Ats {
    #[doc = "0x00..0x08 - The priority queue dequeue register"]
    #[inline(always)]
    pub const fn dequeue(&self) -> &Dequeue {
        &self.dequeue
    }
    #[doc = "0x08..0x800 - The priority queue enqueue registers"]
    #[inline(always)]
    pub const fn enqueue(&self, n: usize) -> &Enqueue {
        &self.enqueue[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x800 - The priority queue enqueue registers"]
    #[inline(always)]
    pub fn enqueue_iter(&self) -> impl Iterator<Item = &Enqueue> {
        self.enqueue.iter()
    }
}
#[doc = "dequeue (r) register accessor: The priority queue dequeue register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dequeue::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dequeue`]
module"]
#[doc(alias = "dequeue")]
pub type Dequeue = crate::Reg<dequeue::DequeueSpec>;
#[doc = "The priority queue dequeue register"]
pub mod dequeue;
#[doc = "enqueue (w) register accessor: The priority queue enqueue registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enqueue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enqueue`]
module"]
#[doc(alias = "enqueue")]
pub type Enqueue = crate::Reg<enqueue::EnqueueSpec>;
#[doc = "The priority queue enqueue registers"]
pub mod enqueue;
