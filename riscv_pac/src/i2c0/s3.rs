#[doc = "Register `S3` reader"]
pub type R = crate::R<S3Spec>;
#[doc = "Register `S3` writer"]
pub type W = crate::W<S3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt vector register\n\nYou can [`read`](crate::Reg::read) this register and get [`s3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S3Spec;
impl crate::RegisterSpec for S3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s3::R`](R) reader structure"]
impl crate::Readable for S3Spec {}
#[doc = "`write(|w| ..)` method takes [`s3::W`](W) writer structure"]
impl crate::Writable for S3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S3 to value 0"]
impl crate::Resettable for S3Spec {
    const RESET_VALUE: u32 = 0;
}
