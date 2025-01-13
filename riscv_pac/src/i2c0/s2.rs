#[doc = "Register `S2` reader"]
pub type R = crate::R<S2Spec>;
#[doc = "Register `S2` writer"]
pub type W = crate::W<S2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Prescalar register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2Spec;
impl crate::RegisterSpec for S2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s2::R`](R) reader structure"]
impl crate::Readable for S2Spec {}
#[doc = "`write(|w| ..)` method takes [`s2::W`](W) writer structure"]
impl crate::Writable for S2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets S2 to value 0"]
impl crate::Resettable for S2Spec {
    const RESET_VALUE: u8 = 0;
}
