#[doc = "Register `GPIO_DIRECTION` reader"]
pub type R = crate::R<GpioDirectionSpec>;
#[doc = "Register `GPIO_DIRECTION` writer"]
pub type W = crate::W<GpioDirectionSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Select the direction of the GPIOs. Each bit position corresponds to the respective GPIO pin. 0 - Output, 1 - Input\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_direction::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_direction::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioDirectionSpec;
impl crate::RegisterSpec for GpioDirectionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_direction::R`](R) reader structure"]
impl crate::Readable for GpioDirectionSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_direction::W`](W) writer structure"]
impl crate::Writable for GpioDirectionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_DIRECTION to value 0"]
impl crate::Resettable for GpioDirectionSpec {
    const RESET_VALUE: u32 = 0;
}
