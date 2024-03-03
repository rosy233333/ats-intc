#[doc = "Register `control-byte%s` reader"]
pub type R = crate::R<ControlbyteSpec>;
#[doc = "Register `control-byte%s` writer"]
pub type W = crate::W<ControlbyteSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<ControlbyteSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "Control field of the Async task scheduler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`controlbyte::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`controlbyte::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlbyteSpec;
impl crate::RegisterSpec for ControlbyteSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`controlbyte::R`](R) reader structure"]
impl crate::Readable for ControlbyteSpec {}
#[doc = "`write(|w| ..)` method takes [`controlbyte::W`](W) writer structure"]
impl crate::Writable for ControlbyteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets control-byte%s to value 0"]
impl crate::Resettable for ControlbyteSpec {
    const RESET_VALUE: u64 = 0;
}
