#[doc = "Register `GPIO_INTR` reader"]
pub type R = crate::R<GpioIntrSpec>;
#[doc = "Register `GPIO_INTR` writer"]
pub type W = crate::W<GpioIntrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "To enable the interrupt of respective GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioIntrSpec;
impl crate::RegisterSpec for GpioIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_intr::R`](R) reader structure"]
impl crate::Readable for GpioIntrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_intr::W`](W) writer structure"]
impl crate::Writable for GpioIntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_INTR to value 0"]
impl crate::Resettable for GpioIntrSpec {
    const RESET_VALUE: u32 = 0;
}
