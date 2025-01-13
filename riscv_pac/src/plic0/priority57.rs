#[doc = "Register `PRIORITY57` reader"]
pub type R = crate::R<Priority57Spec>;
#[doc = "Register `PRIORITY57` writer"]
pub type W = crate::W<Priority57Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 57\n\nYou can [`read`](crate::Reg::read) this register and get [`priority57::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority57::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority57Spec;
impl crate::RegisterSpec for Priority57Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority57::R`](R) reader structure"]
impl crate::Readable for Priority57Spec {}
#[doc = "`write(|w| ..)` method takes [`priority57::W`](W) writer structure"]
impl crate::Writable for Priority57Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY57 to value 0"]
impl crate::Resettable for Priority57Spec {
    const RESET_VALUE: u32 = 0;
}
