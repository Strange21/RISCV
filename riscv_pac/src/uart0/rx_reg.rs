#[doc = "Register `RX_REG` reader"]
pub type R = crate::R<RxRegSpec>;
#[doc = "Register `RX_REG` writer"]
pub type W = crate::W<RxRegSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "RX data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxRegSpec;
impl crate::RegisterSpec for RxRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_reg::R`](R) reader structure"]
impl crate::Readable for RxRegSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_reg::W`](W) writer structure"]
impl crate::Writable for RxRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_REG to value 0"]
impl crate::Resettable for RxRegSpec {
    const RESET_VALUE: u32 = 0;
}
