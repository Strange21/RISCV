#[doc = "Register `GPIO_TOGGLE` reader"]
pub type R = crate::R<GpioToggleSpec>;
#[doc = "Register `GPIO_TOGGLE` writer"]
pub type W = crate::W<GpioToggleSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "To invert the respective GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_toggle::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_toggle::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioToggleSpec;
impl crate::RegisterSpec for GpioToggleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_toggle::R`](R) reader structure"]
impl crate::Readable for GpioToggleSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_toggle::W`](W) writer structure"]
impl crate::Writable for GpioToggleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_TOGGLE to value 0"]
impl crate::Resettable for GpioToggleSpec {
    const RESET_VALUE: u32 = 0;
}
