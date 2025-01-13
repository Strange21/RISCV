#[doc = "Register `PRIORITY6` reader"]
pub type R = crate::R<Priority6Spec>;
#[doc = "Register `PRIORITY6` writer"]
pub type W = crate::W<Priority6Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 6\n\nYou can [`read`](crate::Reg::read) this register and get [`priority6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority6Spec;
impl crate::RegisterSpec for Priority6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority6::R`](R) reader structure"]
impl crate::Readable for Priority6Spec {}
#[doc = "`write(|w| ..)` method takes [`priority6::W`](W) writer structure"]
impl crate::Writable for Priority6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY6 to value 0"]
impl crate::Resettable for Priority6Spec {
    const RESET_VALUE: u32 = 0;
}
