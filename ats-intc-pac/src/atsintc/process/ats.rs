#[repr(C)]
#[doc = "Async task scheduler of one process"]
#[doc(alias = "ats")]
pub struct Ats {
    control: [Controlbyte; 4],
    membuf: Membuf,
    dequeue: Dequeue,
    enqueueregs: [Enqueue; 250],
}
impl Ats {
    #[doc = "0x00..0x20 - Control field of the Async task scheduler"]
    #[inline(always)]
    pub const fn control(&self, n: usize) -> &Controlbyte {
        &self.control[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Control field of the Async task scheduler"]
    #[inline(always)]
    pub fn control_iter(&self) -> impl Iterator<Item = &Controlbyte> {
        self.control.iter()
    }
    #[doc = "0x20..0x28 - Memory buffer of the Async task scheduler"]
    #[inline(always)]
    pub const fn membuf(&self) -> &Membuf {
        &self.membuf
    }
    #[doc = "0x28..0x30 - Deque position of the Async task scheduler"]
    #[inline(always)]
    pub const fn dequeue(&self) -> &Dequeue {
        &self.dequeue
    }
    #[doc = "0x30..0x800 - Enqueue position of the Async task scheduler"]
    #[inline(always)]
    pub const fn enqueueregs(&self, n: usize) -> &Enqueue {
        &self.enqueueregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x800 - Enqueue position of the Async task scheduler"]
    #[inline(always)]
    pub fn enqueueregs_iter(&self) -> impl Iterator<Item = &Enqueue> {
        self.enqueueregs.iter()
    }
}
#[doc = "control-byte (rw) register accessor: Control field of the Async task scheduler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`controlbyte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`controlbyte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@controlbyte`]
module"]
#[doc(alias = "control-byte")]
pub type Controlbyte = crate::Reg<controlbyte::ControlbyteSpec>;
#[doc = "Control field of the Async task scheduler"]
pub mod controlbyte;
#[doc = "membuf (w) register accessor: Memory buffer of the Async task scheduler\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`membuf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@membuf`]
module"]
#[doc(alias = "membuf")]
pub type Membuf = crate::Reg<membuf::MembufSpec>;
#[doc = "Memory buffer of the Async task scheduler"]
pub mod membuf;
#[doc = "dequeue (r) register accessor: Deque position of the Async task scheduler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dequeue::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dequeue`]
module"]
#[doc(alias = "dequeue")]
pub type Dequeue = crate::Reg<dequeue::DequeueSpec>;
#[doc = "Deque position of the Async task scheduler"]
pub mod dequeue;
#[doc = "enqueue (w) register accessor: Enqueue position of the Async task scheduler\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enqueue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enqueue`]
module"]
#[doc(alias = "enqueue")]
pub type Enqueue = crate::Reg<enqueue::EnqueueSpec>;
#[doc = "Enqueue position of the Async task scheduler"]
pub mod enqueue;
