#[doc = "Register `ipc-bq%s` writer"]
pub type W = crate::W<IpcbqSpec>;
impl core::fmt::Debug for crate::generic::Reg<IpcbqSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Block queue enqueue registers\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipcbq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcbqSpec;
impl crate::RegisterSpec for IpcbqSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`ipcbq::W`](W) writer structure"]
impl crate::Writable for IpcbqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets ipc-bq%s to value 0"]
impl crate::Resettable for IpcbqSpec {
    const RESET_VALUE: u64 = 0;
}
