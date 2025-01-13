#[doc = "Register `MUX5` reader"]
pub type R = crate::R<Mux5Spec>;
#[doc = "Register `MUX5` writer"]
pub type W = crate::W<Mux5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Select between GPIO5 and PWM5. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mux5Spec;
impl crate::RegisterSpec for Mux5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mux5::R`](R) reader structure"]
impl crate::Readable for Mux5Spec {}
#[doc = "`write(|w| ..)` method takes [`mux5::W`](W) writer structure"]
impl crate::Writable for Mux5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUX5 to value 0"]
impl crate::Resettable for Mux5Spec {
    const RESET_VALUE: u32 = 0;
}
