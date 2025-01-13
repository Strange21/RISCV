#[doc = "Register `PRIORITY21` reader"]
pub type R = crate::R<Priority21Spec>;
#[doc = "Register `PRIORITY21` writer"]
pub type W = crate::W<Priority21Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority register for interrupt source 21\n\nYou can [`read`](crate::Reg::read) this register and get [`priority21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Priority21Spec;
impl crate::RegisterSpec for Priority21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority21::R`](R) reader structure"]
impl crate::Readable for Priority21Spec {}
#[doc = "`write(|w| ..)` method takes [`priority21::W`](W) writer structure"]
impl crate::Writable for Priority21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY21 to value 0"]
impl crate::Resettable for Priority21Spec {
    const RESET_VALUE: u32 = 0;
}
