#[doc = "Register `RX_THRESHOLD` reader"]
pub type R = crate::R<RxThresholdSpec>;
#[doc = "Register `RX_THRESHOLD` writer"]
pub type W = crate::W<RxThresholdSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The threshold value to indicate the RX FIFO almost full interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxThresholdSpec;
impl crate::RegisterSpec for RxThresholdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rx_threshold::R`](R) reader structure"]
impl crate::Readable for RxThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_threshold::W`](W) writer structure"]
impl crate::Writable for RxThresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RX_THRESHOLD to value 0"]
impl crate::Resettable for RxThresholdSpec {
    const RESET_VALUE: u8 = 0;
}
