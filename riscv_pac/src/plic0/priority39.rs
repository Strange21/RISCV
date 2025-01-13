#[doc = "Register `PRIORITY39` reader"]
pub type R = crate::R<Priority39Spec>;
#[doc = "Register `PRIORITY39` writer"]
pub type W = crate::W<Priority39Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 39\n\nYou can [`read`](crate::Reg::read) this register and get [`priority39::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority39::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority39Spec;
impl crate::RegisterSpec for Priority39Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority39::R`](R) reader structure"]
impl crate::Readable for Priority39Spec {}
#[doc = "`write(|w| ..)` method takes [`priority39::W`](W) writer structure"]
impl crate::Writable for Priority39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY39 to value 0"]
impl crate::Resettable for Priority39Spec {
    const RESET_VALUE: u32 = 0;
}
