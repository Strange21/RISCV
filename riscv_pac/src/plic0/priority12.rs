#[doc = "Register `PRIORITY12` reader"]
pub type R = crate::R<Priority12Spec>;
#[doc = "Register `PRIORITY12` writer"]
pub type W = crate::W<Priority12Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 12\n\nYou can [`read`](crate::Reg::read) this register and get [`priority12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority12Spec;
impl crate::RegisterSpec for Priority12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority12::R`](R) reader structure"]
impl crate::Readable for Priority12Spec {}
#[doc = "`write(|w| ..)` method takes [`priority12::W`](W) writer structure"]
impl crate::Writable for Priority12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY12 to value 0"]
impl crate::Resettable for Priority12Spec {
    const RESET_VALUE: u32 = 0;
}
