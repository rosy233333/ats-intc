#[doc = "Register `membuf` writer"]
pub type W = crate::W<MembufSpec>;
impl core::fmt::Debug for crate::generic::Reg<MembufSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Membuf field of the IPC controller\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`membuf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MembufSpec;
impl crate::RegisterSpec for MembufSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`membuf::W`](W) writer structure"]
impl crate::Writable for MembufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets membuf to value 0"]
impl crate::Resettable for MembufSpec {
    const RESET_VALUE: u64 = 0;
}
