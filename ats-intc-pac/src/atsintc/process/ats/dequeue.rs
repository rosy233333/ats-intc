#[doc = "Register `dequeue` reader"]
pub type R = crate::R<DequeueSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<DequeueSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "The priority queue dequeue register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dequeue::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DequeueSpec;
impl crate::RegisterSpec for DequeueSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`dequeue::R`](R) reader structure"]
impl crate::Readable for DequeueSpec {}
#[doc = "`reset()` method sets dequeue to value 0"]
impl crate::Resettable for DequeueSpec {
    const RESET_VALUE: u64 = 0;
}
