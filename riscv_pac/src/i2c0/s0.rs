#[doc = "Register `S0` reader"]
pub type R = crate::R<S0Spec>;
#[doc = "Register `S0` writer"]
pub type W = crate::W<S0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data Transmission register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S0Spec;
impl crate::RegisterSpec for S0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s0::R`](R) reader structure"]
impl crate::Readable for S0Spec {}
#[doc = "`write(|w| ..)` method takes [`s0::W`](W) writer structure"]
impl crate::Writable for S0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets S0 to value 0"]
impl crate::Resettable for S0Spec {
    const RESET_VALUE: u8 = 0;
}
