#[doc = "Register `PRIORITY24` reader"]
pub type R = crate::R<Priority24Spec>;
#[doc = "Register `PRIORITY24` writer"]
pub type W = crate::W<Priority24Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 24\n\nYou can [`read`](crate::Reg::read) this register and get [`priority24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority24Spec;
impl crate::RegisterSpec for Priority24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority24::R`](R) reader structure"]
impl crate::Readable for Priority24Spec {}
#[doc = "`write(|w| ..)` method takes [`priority24::W`](W) writer structure"]
impl crate::Writable for Priority24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY24 to value 0"]
impl crate::Resettable for Priority24Spec {
    const RESET_VALUE: u32 = 0;
}
