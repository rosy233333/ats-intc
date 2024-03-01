#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    processes: (),
    _reserved1: [u8; 0x00ff_d000],
    intc: Intc,
}
impl RegisterBlock {
    #[doc = "0x00..0x896638 - Related registers of one process"]
    #[inline(always)]
    pub const fn processes(&self, n: usize) -> &Process {
        #[allow(clippy::no_effect)]
        [(); 4093][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(4096 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x896638 - Related registers of one process"]
    #[inline(always)]
    pub fn processes_iter(&self) -> impl Iterator<Item = &Process> {
        (0..4093).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(4096 * n)
                .cast()
        })
    }
    #[doc = "0xffd000..0xfff080 - Interrupt controller"]
    #[inline(always)]
    pub const fn intc(&self) -> &Intc {
        &self.intc
    }
}
#[doc = "Related registers of one process"]
pub use self::process::Process;
#[doc = r"Cluster"]
#[doc = "Related registers of one process"]
pub mod process;
#[doc = "Interrupt controller"]
pub use self::intc::Intc;
#[doc = r"Cluster"]
#[doc = "Interrupt controller"]
pub mod intc;
