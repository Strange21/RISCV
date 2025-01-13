#[doc = "Register `MUX7` reader"]
pub type R = crate::R<Mux7Spec>;
#[doc = "Register `MUX7` writer"]
pub type W = crate::W<Mux7Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Select between GPIO7 and PWM7. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mux7Spec;
impl crate::RegisterSpec for Mux7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mux7::R`](R) reader structure"]
impl crate::Readable for Mux7Spec {}
#[doc = "`write(|w| ..)` method takes [`mux7::W`](W) writer structure"]
impl crate::Writable for Mux7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUX7 to value 0"]
impl crate::Resettable for Mux7Spec {
    const RESET_VALUE: u32 = 0;
}
