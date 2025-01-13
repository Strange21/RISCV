#[doc = "Register `GPIO_DATA` reader"]
pub type R = crate::R<GpioDataSpec>;
#[doc = "Register `GPIO_DATA` writer"]
pub type W = crate::W<GpioDataSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Contains the data to be sent out if the GPIO pin is configured as output and the data recieved when configured as input. Each bit position corresponds to the respective GPIO pin.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioDataSpec;
impl crate::RegisterSpec for GpioDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_data::R`](R) reader structure"]
impl crate::Readable for GpioDataSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_data::W`](W) writer structure"]
impl crate::Writable for GpioDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_DATA to value 0"]
impl crate::Resettable for GpioDataSpec {
    const RESET_VALUE: u32 = 0;
}
