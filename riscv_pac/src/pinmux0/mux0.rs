#[doc = "Register `MUX0` reader"]
pub type R = crate::R<Mux0Spec>;
#[doc = "Register `MUX0` writer"]
pub type W = crate::W<Mux0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Select between GPIO0 and PWM0. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mux0Spec;
impl crate::RegisterSpec for Mux0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mux0::R`](R) reader structure"]
impl crate::Readable for Mux0Spec {}
#[doc = "`write(|w| ..)` method takes [`mux0::W`](W) writer structure"]
impl crate::Writable for Mux0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUX0 to value 0"]
impl crate::Resettable for Mux0Spec {
    const RESET_VALUE: u32 = 0;
}
