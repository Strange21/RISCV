#[doc = "Register `INTR_EN_0_32` reader"]
pub type R = crate::R<IntrEn0_32Spec>;
#[doc = "Register `INTR_EN_0_32` writer"]
pub type W = crate::W<IntrEn0_32Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt enable bits of sources 0-32\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_0_32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_0_32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrEn0_32Spec;
impl crate::RegisterSpec for IntrEn0_32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_en_0_32::R`](R) reader structure"]
impl crate::Readable for IntrEn0_32Spec {}
#[doc = "`write(|w| ..)` method takes [`intr_en_0_32::W`](W) writer structure"]
impl crate::Writable for IntrEn0_32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_EN_0_32 to value 0"]
impl crate::Resettable for IntrEn0_32Spec {
    const RESET_VALUE: u32 = 0;
}
