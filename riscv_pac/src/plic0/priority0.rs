#[doc = "Register `PRIORITY0` reader"]
pub type R = crate::R<Priority0Spec>;
#[doc = "Register `PRIORITY0` writer"]
pub type W = crate::W<Priority0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`priority0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority0Spec;
impl crate::RegisterSpec for Priority0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority0::R`](R) reader structure"]
impl crate::Readable for Priority0Spec {}
#[doc = "`write(|w| ..)` method takes [`priority0::W`](W) writer structure"]
impl crate::Writable for Priority0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY0 to value 0"]
impl crate::Resettable for Priority0Spec {
    const RESET_VALUE: u32 = 0;
}
