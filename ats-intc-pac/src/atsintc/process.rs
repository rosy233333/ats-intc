#[repr(C)]
#[doc = "Related registers of one process"]
#[doc(alias = "process")]
pub struct Process {
    ats: Ats,
    ipcc: Ipcc,
}
impl Process {
    #[doc = "0x00..0x800 - Async task scheduler of one process"]
    #[inline(always)]
    pub const fn ats(&self) -> &Ats {
        &self.ats
    }
    #[doc = "0x800..0x898 - IPC controller of one process"]
    #[inline(always)]
    pub const fn ipcc(&self) -> &Ipcc {
        &self.ipcc
    }
}
#[doc = "Async task scheduler of one process"]
pub use self::ats::Ats;
#[doc = r"Cluster"]
#[doc = "Async task scheduler of one process"]
pub mod ats;
#[doc = "IPC controller of one process"]
pub use self::ipcc::Ipcc;
#[doc = r"Cluster"]
#[doc = "IPC controller of one process"]
pub mod ipcc;
