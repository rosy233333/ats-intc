#[repr(C)]
#[doc = "Interrupt controller"]
#[doc(alias = "intc")]
pub struct Intc {
    intrenqueue: [Intrenqueue; 224],
}
impl Intc {
    #[doc = "0x00..0x700 - Interrupt handler queue enqueue register"]
    #[inline(always)]
    pub const fn intrenqueue(&self, n: usize) -> &Intrenqueue {
        &self.intrenqueue[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x700 - Interrupt handler queue enqueue register"]
    #[inline(always)]
    pub fn intrenqueue_iter(&self) -> impl Iterator<Item = &Intrenqueue> {
        self.intrenqueue.iter()
    }
}
#[doc = "intr-enqueue (w) register accessor: Interrupt handler queue enqueue register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intrenqueue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intrenqueue`]
module"]
#[doc(alias = "intr-enqueue")]
pub type Intrenqueue = crate::Reg<intrenqueue::IntrenqueueSpec>;
#[doc = "Interrupt handler queue enqueue register"]
pub mod intrenqueue;
