#[doc = "Register `MUX3` reader"]
pub type R = crate::R<Mux3Spec>;
#[doc = "Register `MUX3` writer"]
pub type W = crate::W<Mux3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Select between GPIO3 and PWM3. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mux3Spec;
impl crate::RegisterSpec for Mux3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mux3::R`](R) reader structure"]
impl crate::Readable for Mux3Spec {}
#[doc = "`write(|w| ..)` method takes [`mux3::W`](W) writer structure"]
impl crate::Writable for Mux3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUX3 to value 0"]
impl crate::Resettable for Mux3Spec {
    const RESET_VALUE: u32 = 0;
}
