#[doc = "Register `PRIORITY40` reader"]
pub type R = crate::R<Priority40Spec>;
#[doc = "Register `PRIORITY40` writer"]
pub type W = crate::W<Priority40Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 40\n\nYou can [`read`](crate::Reg::read) this register and get [`priority40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority40Spec;
impl crate::RegisterSpec for Priority40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority40::R`](R) reader structure"]
impl crate::Readable for Priority40Spec {}
#[doc = "`write(|w| ..)` method takes [`priority40::W`](W) writer structure"]
impl crate::Writable for Priority40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY40 to value 0"]
impl crate::Resettable for Priority40Spec {
    const RESET_VALUE: u32 = 0;
}
