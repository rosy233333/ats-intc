#[repr(C)]
#[doc = "IPC controller of one process"]
#[doc(alias = "ipcc")]
pub struct Ipcc {
    send: Send,
    ipcbqregs: [Ipcbq; 31],
}
impl Ipcc {
    #[doc = "0x00..0x08 - The IPC send register"]
    #[inline(always)]
    pub const fn send(&self) -> &Send {
        &self.send
    }
    #[doc = "0x08..0x100 - Block queue enqueue registers"]
    #[inline(always)]
    pub const fn ipcbqregs(&self, n: usize) -> &Ipcbq {
        &self.ipcbqregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x100 - Block queue enqueue registers"]
    #[inline(always)]
    pub fn ipcbqregs_iter(&self) -> impl Iterator<Item = &Ipcbq> {
        self.ipcbqregs.iter()
    }
}
#[doc = "send (w) register accessor: The IPC send register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`send::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@send`]
module"]
#[doc(alias = "send")]
pub type Send = crate::Reg<send::SendSpec>;
#[doc = "The IPC send register"]
pub mod send;
#[doc = "ipc-bq (w) register accessor: Block queue enqueue registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipcbq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipcbq`]
module"]
#[doc(alias = "ipc-bq")]
pub type Ipcbq = crate::Reg<ipcbq::IpcbqSpec>;
#[doc = "Block queue enqueue registers"]
pub mod ipcbq;
