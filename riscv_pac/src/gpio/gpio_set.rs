#[doc = "Register `GPIO_SET` reader"]
pub type R = crate::R<GpioSetSpec>;
#[doc = "Register `GPIO_SET` writer"]
pub type W = crate::W<GpioSetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "To set the respective GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioSetSpec;
impl crate::RegisterSpec for GpioSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_set::R`](R) reader structure"]
impl crate::Readable for GpioSetSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_set::W`](W) writer structure"]
impl crate::Writable for GpioSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_SET to value 0"]
impl crate::Resettable for GpioSetSpec {
    const RESET_VALUE: u32 = 0;
}
