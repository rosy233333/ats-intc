#[doc = "Register `ipc-bq%s` reader"]
pub type R = crate::R<IpcbqSpec>;
#[doc = "Register `ipc-bq%s` writer"]
pub type W = crate::W<IpcbqSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<IpcbqSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "BQ field of the IPC controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipcbq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipcbq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcbqSpec;
impl crate::RegisterSpec for IpcbqSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`ipcbq::R`](R) reader structure"]
impl crate::Readable for IpcbqSpec {}
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
