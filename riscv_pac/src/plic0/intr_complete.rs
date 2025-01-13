#[doc = "Register `INTR_COMPLETE` reader"]
pub type R = crate::R<IntrCompleteSpec>;
#[doc = "Register `INTR_COMPLETE` writer"]
pub type W = crate::W<IntrCompleteSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Interrupt claim/complete register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_complete::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_complete::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrCompleteSpec;
impl crate::RegisterSpec for IntrCompleteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_complete::R`](R) reader structure"]
impl crate::Readable for IntrCompleteSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_complete::W`](W) writer structure"]
impl crate::Writable for IntrCompleteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_COMPLETE to value 0"]
impl crate::Resettable for IntrCompleteSpec {
    const RESET_VALUE: u32 = 0;
}
