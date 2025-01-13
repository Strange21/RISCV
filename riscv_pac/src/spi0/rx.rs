#[doc = "Register `RX` reader"]
pub type R = crate::R<RxSpec>;
#[doc = "Register `RX` writer"]
pub type W = crate::W<RxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "holds the tx data. This register is read by the AXI read request. The data is written from the RX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxSpec;
impl crate::RegisterSpec for RxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx::R`](R) reader structure"]
impl crate::Readable for RxSpec {}
#[doc = "`write(|w| ..)` method takes [`rx::W`](W) writer structure"]
impl crate::Writable for RxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX to value 0"]
impl crate::Resettable for RxSpec {
    const RESET_VALUE: u32 = 0;
}
