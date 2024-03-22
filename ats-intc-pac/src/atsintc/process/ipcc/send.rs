#[doc = "Register `send` writer"]
pub type W = crate::W<SendSpec>;
impl core::fmt::Debug for crate::generic::Reg<SendSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "The IPC send register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`send::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SendSpec;
impl crate::RegisterSpec for SendSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`send::W`](W) writer structure"]
impl crate::Writable for SendSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets send to value 0"]
impl crate::Resettable for SendSpec {
    const RESET_VALUE: u64 = 0;
}
