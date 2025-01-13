#[doc = "Register `INTR_EN_33_63` reader"]
pub type R = crate::R<IntrEn33_63Spec>;
#[doc = "Register `INTR_EN_33_63` writer"]
pub type W = crate::W<IntrEn33_63Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt enable bits of sources 33-63\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en_33_63::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en_33_63::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrEn33_63Spec;
impl crate::RegisterSpec for IntrEn33_63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_en_33_63::R`](R) reader structure"]
impl crate::Readable for IntrEn33_63Spec {}
#[doc = "`write(|w| ..)` method takes [`intr_en_33_63::W`](W) writer structure"]
impl crate::Writable for IntrEn33_63Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_EN_33_63 to value 0"]
impl crate::Resettable for IntrEn33_63Spec {
    const RESET_VALUE: u32 = 0;
}
