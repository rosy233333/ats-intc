#[repr(C)]
#[doc = "Interrupt controller"]
#[doc(alias = "intc")]
pub struct Intc {
    control: [Controlbyte; 16],
    enqueueregs: [Enqueue; 1024],
}
impl Intc {
    #[doc = "0x00..0x80 - Control field of the Interrupt controller"]
    #[inline(always)]
    pub const fn control(&self, n: usize) -> &Controlbyte {
        &self.control[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - Control field of the Interrupt controller"]
    #[inline(always)]
    pub fn control_iter(&self) -> impl Iterator<Item = &Controlbyte> {
        self.control.iter()
    }
    #[doc = "0x80..0x2080 - Enqueue position of the Interrupt controller"]
    #[inline(always)]
    pub const fn enqueueregs(&self, n: usize) -> &Enqueue {
        &self.enqueueregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x2080 - Enqueue position of the Interrupt controller"]
    #[inline(always)]
    pub fn enqueueregs_iter(&self) -> impl Iterator<Item = &Enqueue> {
        self.enqueueregs.iter()
    }
}
#[doc = "control-byte (rw) register accessor: Control field of the Interrupt controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`controlbyte::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`controlbyte::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@controlbyte`]
module"]
#[doc(alias = "control-byte")]
pub type Controlbyte = crate::Reg<controlbyte::ControlbyteSpec>;
#[doc = "Control field of the Interrupt controller"]
pub mod controlbyte;
#[doc = "enqueue (w) register accessor: Enqueue position of the Interrupt controller\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enqueue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enqueue`]
module"]
#[doc(alias = "enqueue")]
pub type Enqueue = crate::Reg<enqueue::EnqueueSpec>;
#[doc = "Enqueue position of the Interrupt controller"]
pub mod enqueue;
