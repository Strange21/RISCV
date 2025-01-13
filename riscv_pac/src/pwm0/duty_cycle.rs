#[doc = "Register `DUTY_CYCLE` reader"]
pub type R = crate::R<DutyCycleSpec>;
#[doc = "Register `DUTY_CYCLE` writer"]
pub type W = crate::W<DutyCycleSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PWM Duty_Cycle register\n\nYou can [`read`](crate::Reg::read) this register and get [`duty_cycle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duty_cycle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DutyCycleSpec;
impl crate::RegisterSpec for DutyCycleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`duty_cycle::R`](R) reader structure"]
impl crate::Readable for DutyCycleSpec {}
#[doc = "`write(|w| ..)` method takes [`duty_cycle::W`](W) writer structure"]
impl crate::Writable for DutyCycleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DUTY_CYCLE to value 0"]
impl crate::Resettable for DutyCycleSpec {
    const RESET_VALUE: u32 = 0;
}
