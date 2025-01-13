#[doc = "Register `S01` reader"]
pub type R = crate::R<S01Spec>;
#[doc = "Register `S01` writer"]
pub type W = crate::W<S01Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "I2C Own Address Slave Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S01Spec;
impl crate::RegisterSpec for S01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s01::R`](R) reader structure"]
impl crate::Readable for S01Spec {}
#[doc = "`write(|w| ..)` method takes [`s01::W`](W) writer structure"]
impl crate::Writable for S01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S01 to value 0"]
impl crate::Resettable for S01Spec {
    const RESET_VALUE: u32 = 0;
}
