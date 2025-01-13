#[doc = "Register `DEADBAND_DELAY` reader"]
pub type R = crate::R<DeadbandDelaySpec>;
#[doc = "Register `DEADBAND_DELAY` writer"]
pub type W = crate::W<DeadbandDelaySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PWM Deadband delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`deadband_delay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deadband_delay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeadbandDelaySpec;
impl crate::RegisterSpec for DeadbandDelaySpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`deadband_delay::R`](R) reader structure"]
impl crate::Readable for DeadbandDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`deadband_delay::W`](W) writer structure"]
impl crate::Writable for DeadbandDelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DEADBAND_DELAY to value 0"]
impl crate::Resettable for DeadbandDelaySpec {
    const RESET_VALUE: u16 = 0;
}
