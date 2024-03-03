#[doc = "Register `ipc-message` reader"]
pub type R = crate::R<IpcmessageSpec>;
#[doc = "Register `ipc-message` writer"]
pub type W = crate::W<IpcmessageSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<IpcmessageSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {}
#[doc = "IPC message pointer of the IPC controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipcmessage::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipcmessage::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpcmessageSpec;
impl crate::RegisterSpec for IpcmessageSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`ipcmessage::R`](R) reader structure"]
impl crate::Readable for IpcmessageSpec {}
#[doc = "`write(|w| ..)` method takes [`ipcmessage::W`](W) writer structure"]
impl crate::Writable for IpcmessageSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets ipc-message to value 0"]
impl crate::Resettable for IpcmessageSpec {
    const RESET_VALUE: u64 = 0;
}
