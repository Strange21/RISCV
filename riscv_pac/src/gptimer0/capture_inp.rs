#[doc = "Register `CAPTURE_INP` reader"]
pub type R = crate::R<CaptureInpSpec>;
#[doc = "Register `CAPTURE_INP` writer"]
pub type W = crate::W<CaptureInpSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Timer capture input register\n\nYou can [`read`](crate::Reg::read) this register and get [`capture_inp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capture_inp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaptureInpSpec;
impl crate::RegisterSpec for CaptureInpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capture_inp::R`](R) reader structure"]
impl crate::Readable for CaptureInpSpec {}
#[doc = "`write(|w| ..)` method takes [`capture_inp::W`](W) writer structure"]
impl crate::Writable for CaptureInpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAPTURE_INP to value 0"]
impl crate::Resettable for CaptureInpSpec {
    const RESET_VALUE: u32 = 0;
}
