#[doc = "Register `handler_lsb` reader"]
pub type R = crate::R<HANDLER_LSB_SPEC>;
#[doc = "Register `handler_lsb` writer"]
pub type W = crate::W<HANDLER_LSB_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<HANDLER_LSB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The handler task pointer lsb\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`handler_lsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`handler_lsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HANDLER_LSB_SPEC;
impl crate::RegisterSpec for HANDLER_LSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`handler_lsb::R`](R) reader structure"]
impl crate::Readable for HANDLER_LSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`handler_lsb::W`](W) writer structure"]
impl crate::Writable for HANDLER_LSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets handler_lsb to value 0"]
impl crate::Resettable for HANDLER_LSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
