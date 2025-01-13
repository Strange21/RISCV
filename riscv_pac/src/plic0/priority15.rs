#[doc = "Register `PRIORITY15` reader"]
pub type R = crate::R<Priority15Spec>;
#[doc = "Register `PRIORITY15` writer"]
pub type W = crate::W<Priority15Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 15\n\nYou can [`read`](crate::Reg::read) this register and get [`priority15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority15Spec;
impl crate::RegisterSpec for Priority15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority15::R`](R) reader structure"]
impl crate::Readable for Priority15Spec {}
#[doc = "`write(|w| ..)` method takes [`priority15::W`](W) writer structure"]
impl crate::Writable for Priority15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY15 to value 0"]
impl crate::Resettable for Priority15Spec {
    const RESET_VALUE: u32 = 0;
}
