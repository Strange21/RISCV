#[doc = "Register `PRIORITY60` reader"]
pub type R = crate::R<Priority60Spec>;
#[doc = "Register `PRIORITY60` writer"]
pub type W = crate::W<Priority60Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 60\n\nYou can [`read`](crate::Reg::read) this register and get [`priority60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority60Spec;
impl crate::RegisterSpec for Priority60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority60::R`](R) reader structure"]
impl crate::Readable for Priority60Spec {}
#[doc = "`write(|w| ..)` method takes [`priority60::W`](W) writer structure"]
impl crate::Writable for Priority60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY60 to value 0"]
impl crate::Resettable for Priority60Spec {
    const RESET_VALUE: u32 = 0;
}
