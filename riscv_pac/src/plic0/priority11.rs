#[doc = "Register `PRIORITY11` reader"]
pub type R = crate::R<Priority11Spec>;
#[doc = "Register `PRIORITY11` writer"]
pub type W = crate::W<Priority11Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 11\n\nYou can [`read`](crate::Reg::read) this register and get [`priority11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority11Spec;
impl crate::RegisterSpec for Priority11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority11::R`](R) reader structure"]
impl crate::Readable for Priority11Spec {}
#[doc = "`write(|w| ..)` method takes [`priority11::W`](W) writer structure"]
impl crate::Writable for Priority11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY11 to value 0"]
impl crate::Resettable for Priority11Spec {
    const RESET_VALUE: u32 = 0;
}
