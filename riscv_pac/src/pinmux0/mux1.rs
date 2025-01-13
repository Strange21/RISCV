#[doc = "Register `MUX1` reader"]
pub type R = crate::R<Mux1Spec>;
#[doc = "Register `MUX1` writer"]
pub type W = crate::W<Mux1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Select between GPIO1 and PWM1. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mux1Spec;
impl crate::RegisterSpec for Mux1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mux1::R`](R) reader structure"]
impl crate::Readable for Mux1Spec {}
#[doc = "`write(|w| ..)` method takes [`mux1::W`](W) writer structure"]
impl crate::Writable for Mux1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUX1 to value 0"]
impl crate::Resettable for Mux1Spec {
    const RESET_VALUE: u32 = 0;
}
