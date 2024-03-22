#[doc = "Register `intr-enqueue%s` writer"]
pub type W = crate::W<IntrenqueueSpec>;
impl core::fmt::Debug for crate::generic::Reg<IntrenqueueSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Interrupt handler queue enqueue register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intrenqueue::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrenqueueSpec;
impl crate::RegisterSpec for IntrenqueueSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`intrenqueue::W`](W) writer structure"]
impl crate::Writable for IntrenqueueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets intr-enqueue%s to value 0"]
impl crate::Resettable for IntrenqueueSpec {
    const RESET_VALUE: u64 = 0;
}
