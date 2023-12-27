#[doc = "Register `control` reader"]
pub type R = crate::R<CONTROL_SPEC>;
#[doc = "Register `control` writer"]
pub type W = crate::W<CONTROL_SPEC>;
#[doc = "Field `reset` reader - Software reset for reseting the executor channel"]
pub type RESET_R = crate::BitReader<RESET_A>;
#[doc = "Software reset for reseting the executor channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESET_A {
    #[doc = "0: `0`"]
    COMPLETE = 0,
    #[doc = "1: `1`"]
    RESET = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::COMPLETE,
            true => RESET_A::RESET,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == RESET_A::COMPLETE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESET_A::RESET
    }
}
#[doc = "Field `reset` writer - Software reset for reseting the executor channel"]
pub type RESET_W<'a, REG> = crate::BitWriter<'a, REG, RESET_A>;
impl<'a, REG> RESET_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::COMPLETE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RESET_A::RESET)
    }
}
#[doc = "Field `lhandler` reader - Software load handler task flag"]
pub type LHANDLER_R = crate::BitReader<LHANDLER_A>;
#[doc = "Software load handler task flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LHANDLER_A {
    #[doc = "0: `0`"]
    COMPLETE = 0,
    #[doc = "1: `1`"]
    LOAD = 1,
}
impl From<LHANDLER_A> for bool {
    #[inline(always)]
    fn from(variant: LHANDLER_A) -> Self {
        variant as u8 != 0
    }
}
impl LHANDLER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LHANDLER_A {
        match self.bits {
            false => LHANDLER_A::COMPLETE,
            true => LHANDLER_A::LOAD,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == LHANDLER_A::COMPLETE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_load(&self) -> bool {
        *self == LHANDLER_A::LOAD
    }
}
#[doc = "Field `lhandler` writer - Software load handler task flag"]
pub type LHANDLER_W<'a, REG> = crate::BitWriter<'a, REG, LHANDLER_A>;
impl<'a, REG> LHANDLER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(LHANDLER_A::COMPLETE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn load(self) -> &'a mut crate::W<REG> {
        self.variant(LHANDLER_A::LOAD)
    }
}
#[doc = "Field `stask` reader - Software store task flag"]
pub type STASK_R = crate::BitReader<STASK_A>;
#[doc = "Software store task flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STASK_A {
    #[doc = "0: `0`"]
    COMPLETE = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<STASK_A> for bool {
    #[inline(always)]
    fn from(variant: STASK_A) -> Self {
        variant as u8 != 0
    }
}
impl STASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STASK_A {
        match self.bits {
            false => STASK_A::COMPLETE,
            true => STASK_A::START,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == STASK_A::COMPLETE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STASK_A::START
    }
}
#[doc = "Field `stask` writer - Software store task flag"]
pub type STASK_W<'a, REG> = crate::BitWriter<'a, REG, STASK_A>;
impl<'a, REG> STASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(STASK_A::COMPLETE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(STASK_A::START)
    }
}
#[doc = "Field `ftask` reader - Software fetch task flag"]
pub type FTASK_R = crate::BitReader<FTASK_A>;
#[doc = "Software fetch task flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTASK_A {
    #[doc = "0: `0`"]
    COMPLETE = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<FTASK_A> for bool {
    #[inline(always)]
    fn from(variant: FTASK_A) -> Self {
        variant as u8 != 0
    }
}
impl FTASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FTASK_A {
        match self.bits {
            false => FTASK_A::COMPLETE,
            true => FTASK_A::START,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == FTASK_A::COMPLETE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == FTASK_A::START
    }
}
#[doc = "Field `ftask` writer - Software fetch task flag"]
pub type FTASK_W<'a, REG> = crate::BitWriter<'a, REG, FTASK_A>;
impl<'a, REG> FTASK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(FTASK_A::COMPLETE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(FTASK_A::START)
    }
}
#[doc = "Field `memcache` reader - When initializing, the software will alloc a memory area and set it as the Executor cache."]
pub type MEMCACHE_R = crate::BitReader<MEMCACHE_A>;
#[doc = "When initializing, the software will alloc a memory area and set it as the Executor cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEMCACHE_A {
    #[doc = "0: `0`"]
    COMPLETE = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<MEMCACHE_A> for bool {
    #[inline(always)]
    fn from(variant: MEMCACHE_A) -> Self {
        variant as u8 != 0
    }
}
impl MEMCACHE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MEMCACHE_A {
        match self.bits {
            false => MEMCACHE_A::COMPLETE,
            true => MEMCACHE_A::START,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == MEMCACHE_A::COMPLETE
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == MEMCACHE_A::START
    }
}
#[doc = "Field `memcache` writer - When initializing, the software will alloc a memory area and set it as the Executor cache."]
pub type MEMCACHE_W<'a, REG> = crate::BitWriter<'a, REG, MEMCACHE_A>;
impl<'a, REG> MEMCACHE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(MEMCACHE_A::COMPLETE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(MEMCACHE_A::START)
    }
}
impl R {
    #[doc = "Bit 0 - Software reset for reseting the executor channel"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software load handler task flag"]
    #[inline(always)]
    pub fn lhandler(&self) -> LHANDLER_R {
        LHANDLER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software store task flag"]
    #[inline(always)]
    pub fn stask(&self) -> STASK_R {
        STASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software fetch task flag"]
    #[inline(always)]
    pub fn ftask(&self) -> FTASK_R {
        FTASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When initializing, the software will alloc a memory area and set it as the Executor cache."]
    #[inline(always)]
    pub fn memcache(&self) -> MEMCACHE_R {
        MEMCACHE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset for reseting the executor channel"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<CONTROL_SPEC> {
        RESET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software load handler task flag"]
    #[inline(always)]
    #[must_use]
    pub fn lhandler(&mut self) -> LHANDLER_W<CONTROL_SPEC> {
        LHANDLER_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software store task flag"]
    #[inline(always)]
    #[must_use]
    pub fn stask(&mut self) -> STASK_W<CONTROL_SPEC> {
        STASK_W::new(self, 2)
    }
    #[doc = "Bit 3 - Software fetch task flag"]
    #[inline(always)]
    #[must_use]
    pub fn ftask(&mut self) -> FTASK_W<CONTROL_SPEC> {
        FTASK_W::new(self, 3)
    }
    #[doc = "Bit 4 - When initializing, the software will alloc a memory area and set it as the Executor cache."]
    #[inline(always)]
    #[must_use]
    pub fn memcache(&mut self) -> MEMCACHE_W<CONTROL_SPEC> {
        MEMCACHE_W::new(self, 4)
    }
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
#[doc = "The transaction control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
