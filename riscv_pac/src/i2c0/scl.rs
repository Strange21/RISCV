#[doc = "Register `SCL` reader"]
pub type R = crate::R<SclSpec>;
#[doc = "Register `SCL` writer"]
pub type W = crate::W<SclSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Clock period register\n\nYou can [`read`](crate::Reg::read) this register and get [`scl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclSpec;
impl crate::RegisterSpec for SclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl::R`](R) reader structure"]
impl crate::Readable for SclSpec {}
#[doc = "`write(|w| ..)` method takes [`scl::W`](W) writer structure"]
impl crate::Writable for SclSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCL to value 0"]
impl crate::Resettable for SclSpec {
    const RESET_VALUE: u32 = 0;
}
