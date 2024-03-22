#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    processes: [Process; 4096],
}
impl RegisterBlock {
    #[doc = "0x00..0x1000000 - Related registers of one process"]
    #[inline(always)]
    pub const fn processes(&self, n: usize) -> &Process {
        &self.processes[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1000000 - Related registers of one process"]
    #[inline(always)]
    pub fn processes_iter(&self) -> impl Iterator<Item = &Process> {
        self.processes.iter()
    }
}
#[doc = "Related registers of one process"]
pub use self::process::Process;
#[doc = r"Cluster"]
#[doc = "Related registers of one process"]
pub mod process;
