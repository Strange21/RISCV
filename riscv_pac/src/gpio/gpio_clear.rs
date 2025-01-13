#[doc = "Register `GPIO_CLEAR` reader"]
pub type R = crate::R<GpioClearSpec>;
#[doc = "Register `GPIO_CLEAR` writer"]
pub type W = crate::W<GpioClearSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "To clear the respective GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioClearSpec;
impl crate::RegisterSpec for GpioClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_clear::R`](R) reader structure"]
impl crate::Readable for GpioClearSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_clear::W`](W) writer structure"]
impl crate::Writable for GpioClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_CLEAR to value 0"]
impl crate::Resettable for GpioClearSpec {
    const RESET_VALUE: u32 = 0;
}
