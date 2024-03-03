#[repr(C)]
#[doc = "IPC controller of one process"]
#[doc(alias = "ipcc")]
pub struct Ipcc {
    control: Control,
    membuf: Membuf,
    ipcmessage: Ipcmessage,
    ipcbqregs: [Ipcbq; 16],
}
impl Ipcc {
    #[doc = "0x00..0x08 - Control field of the IPC controller"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x08..0x10 - Membuf field of the IPC controller"]
    #[inline(always)]
    pub const fn membuf(&self) -> &Membuf {
        &self.membuf
    }
    #[doc = "0x10..0x18 - IPC message pointer of the IPC controller"]
    #[inline(always)]
    pub const fn ipcmessage(&self) -> &Ipcmessage {
        &self.ipcmessage
    }
    #[doc = "0x18..0x98 - BQ field of the IPC controller"]
    #[inline(always)]
    pub const fn ipcbqregs(&self, n: usize) -> &Ipcbq {
        &self.ipcbqregs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x98 - BQ field of the IPC controller"]
    #[inline(always)]
    pub fn ipcbqregs_iter(&self) -> impl Iterator<Item = &Ipcbq> {
        self.ipcbqregs.iter()
    }
}
#[doc = "control (rw) register accessor: Control field of the IPC controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "control")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control field of the IPC controller"]
pub mod control;
#[doc = "membuf (w) register accessor: Membuf field of the IPC controller\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`membuf::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@membuf`]
module"]
#[doc(alias = "membuf")]
pub type Membuf = crate::Reg<membuf::MembufSpec>;
#[doc = "Membuf field of the IPC controller"]
pub mod membuf;
#[doc = "ipc-message (rw) register accessor: IPC message pointer of the IPC controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipcmessage::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipcmessage::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipcmessage`]
module"]
#[doc(alias = "ipc-message")]
pub type Ipcmessage = crate::Reg<ipcmessage::IpcmessageSpec>;
#[doc = "IPC message pointer of the IPC controller"]
pub mod ipcmessage;
#[doc = "ipc-bq (rw) register accessor: BQ field of the IPC controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipcbq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipcbq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipcbq`]
module"]
#[doc(alias = "ipc-bq")]
pub type Ipcbq = crate::Reg<ipcbq::IpcbqSpec>;
#[doc = "BQ field of the IPC controller"]
pub mod ipcbq;
