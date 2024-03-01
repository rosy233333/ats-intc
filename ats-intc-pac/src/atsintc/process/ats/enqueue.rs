#[doc = "Register `enqueue%s` writer"]
pub type W = crate::W<EnqueueSpec>;
impl core::fmt::Debug for crate::generic::Reg<EnqueueSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Enqueue position of the Async task scheduler\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enqueue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnqueueSpec;
impl crate::RegisterSpec for EnqueueSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`enqueue::W`](W) writer structure"]
impl crate::Writable for EnqueueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets enqueue%s to value 0"]
impl crate::Resettable for EnqueueSpec {
    const RESET_VALUE: u64 = 0;
}
