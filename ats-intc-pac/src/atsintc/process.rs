#[repr(C)]
#[doc = "Related registers of one process"]
#[doc(alias = "process")]
pub struct Process {
    ats: Ats,
    ipcc: Ipcc,
    intc: Intc,
}
impl Process {
    #[doc = "0x00..0x800 - Async task scheduler of one process"]
    #[inline(always)]
    pub const fn ats(&self) -> &Ats {
        &self.ats
    }
    #[doc = "0x800..0x900 - IPC controller of one process"]
    #[inline(always)]
    pub const fn ipcc(&self) -> &Ipcc {
        &self.ipcc
    }
    #[doc = "0x900..0x1000 - Interrupt controller"]
    #[inline(always)]
    pub const fn intc(&self) -> &Intc {
        &self.intc
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
#[doc = "Interrupt controller"]
pub use self::intc::Intc;
#[doc = r"Cluster"]
#[doc = "Interrupt controller"]
pub mod intc;
