#[doc = "Register `PRIORITY1` reader"]
pub type R = crate::R<Priority1Spec>;
#[doc = "Register `PRIORITY1` writer"]
pub type W = crate::W<Priority1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`priority1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority1Spec;
impl crate::RegisterSpec for Priority1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority1::R`](R) reader structure"]
impl crate::Readable for Priority1Spec {}
#[doc = "`write(|w| ..)` method takes [`priority1::W`](W) writer structure"]
impl crate::Writable for Priority1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY1 to value 0"]
impl crate::Resettable for Priority1Spec {
    const RESET_VALUE: u32 = 0;
}
