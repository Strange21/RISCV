#[doc = "Register `PRIORITY20` reader"]
pub type R = crate::R<Priority20Spec>;
#[doc = "Register `PRIORITY20` writer"]
pub type W = crate::W<Priority20Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 20\n\nYou can [`read`](crate::Reg::read) this register and get [`priority20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority20Spec;
impl crate::RegisterSpec for Priority20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority20::R`](R) reader structure"]
impl crate::Readable for Priority20Spec {}
#[doc = "`write(|w| ..)` method takes [`priority20::W`](W) writer structure"]
impl crate::Writable for Priority20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY20 to value 0"]
impl crate::Resettable for Priority20Spec {
    const RESET_VALUE: u32 = 0;
}
