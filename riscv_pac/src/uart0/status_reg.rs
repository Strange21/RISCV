#[doc = "Register `STATUS_REG` reader"]
pub type R = crate::R<StatusRegSpec>;
#[doc = "Register `STATUS_REG` writer"]
pub type W = crate::W<StatusRegSpec>;
#[doc = "Field `STATUS_TX_EMPTY` reader - Transmittor register empty flag"]
pub type StatusTxEmptyR = crate::BitReader;
#[doc = "Field `STATUS_TX_EMPTY` writer - Transmittor register empty flag"]
pub type StatusTxEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_TX_FULL` reader - Transmittor register full flag"]
pub type StatusTxFullR = crate::BitReader;
#[doc = "Field `STATUS_TX_FULL` writer - Transmittor register full flag"]
pub type StatusTxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_RX_NOT_EMPTY` reader - Receiver register not empty flag"]
pub type StatusRxNotEmptyR = crate::BitReader;
#[doc = "Field `STATUS_RX_NOT_EMPTY` writer - Receiver register not empty flag"]
pub type StatusRxNotEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_RX_FULL` reader - Receiver register full flag"]
pub type StatusRxFullR = crate::BitReader;
#[doc = "Field `STATUS_RX_FULL` writer - Receiver register full flag"]
pub type StatusRxFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_PARITY_ERR` reader - Parity error in received data flag"]
pub type StatusParityErrR = crate::BitReader;
#[doc = "Field `STATUS_PARITY_ERR` writer - Parity error in received data flag"]
pub type StatusParityErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_OVERRUN_ERR` reader - Overrun error flag"]
pub type StatusOverrunErrR = crate::BitReader;
#[doc = "Field `STATUS_OVERRUN_ERR` writer - Overrun error flag"]
pub type StatusOverrunErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_FRAME_ERR` reader - Frame error flag"]
pub type StatusFrameErrR = crate::BitReader;
#[doc = "Field `STATUS_FRAME_ERR` writer - Frame error flag"]
pub type StatusFrameErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_BREAK_ERR` reader - Break error flag"]
pub type StatusBreakErrR = crate::BitReader;
#[doc = "Field `STATUS_BREAK_ERR` writer - Break error flag"]
pub type StatusBreakErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_RX_ALMOST_FULL` reader - Indicates that the RXFIFO is almost full depending on RX_threshold register"]
pub type StatusRxAlmostFullR = crate::BitReader;
#[doc = "Field `STATUS_RX_ALMOST_FULL` writer - Indicates that the RXFIFO is almost full depending on RX_threshold register"]
pub type StatusRxAlmostFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmittor register empty flag"]
    #[inline(always)]
    pub fn status_tx_empty(&self) -> StatusTxEmptyR {
        StatusTxEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmittor register full flag"]
    #[inline(always)]
    pub fn status_tx_full(&self) -> StatusTxFullR {
        StatusTxFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver register not empty flag"]
    #[inline(always)]
    pub fn status_rx_not_empty(&self) -> StatusRxNotEmptyR {
        StatusRxNotEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver register full flag"]
    #[inline(always)]
    pub fn status_rx_full(&self) -> StatusRxFullR {
        StatusRxFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity error in received data flag"]
    #[inline(always)]
    pub fn status_parity_err(&self) -> StatusParityErrR {
        StatusParityErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun error flag"]
    #[inline(always)]
    pub fn status_overrun_err(&self) -> StatusOverrunErrR {
        StatusOverrunErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Frame error flag"]
    #[inline(always)]
    pub fn status_frame_err(&self) -> StatusFrameErrR {
        StatusFrameErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break error flag"]
    #[inline(always)]
    pub fn status_break_err(&self) -> StatusBreakErrR {
        StatusBreakErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates that the RXFIFO is almost full depending on RX_threshold register"]
    #[inline(always)]
    pub fn status_rx_almost_full(&self) -> StatusRxAlmostFullR {
        StatusRxAlmostFullR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmittor register empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn status_tx_empty(&mut self) -> StatusTxEmptyW<StatusRegSpec> {
        StatusTxEmptyW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmittor register full flag"]
    #[inline(always)]
    #[must_use]
    pub fn status_tx_full(&mut self) -> StatusTxFullW<StatusRegSpec> {
        StatusTxFullW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver register not empty flag"]
    #[inline(always)]
    #[must_use]
    pub fn status_rx_not_empty(&mut self) -> StatusRxNotEmptyW<StatusRegSpec> {
        StatusRxNotEmptyW::new(self, 2)
    }
    #[doc = "Bit 3 - Receiver register full flag"]
    #[inline(always)]
    #[must_use]
    pub fn status_rx_full(&mut self) -> StatusRxFullW<StatusRegSpec> {
        StatusRxFullW::new(self, 3)
    }
    #[doc = "Bit 4 - Parity error in received data flag"]
    #[inline(always)]
    #[must_use]
    pub fn status_parity_err(&mut self) -> StatusParityErrW<StatusRegSpec> {
        StatusParityErrW::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun error flag"]
    #[inline(always)]
    #[must_use]
    pub fn status_overrun_err(&mut self) -> StatusOverrunErrW<StatusRegSpec> {
        StatusOverrunErrW::new(self, 5)
    }
    #[doc = "Bit 6 - Frame error flag"]
    #[inline(always)]
    #[must_use]
    pub fn status_frame_err(&mut self) -> StatusFrameErrW<StatusRegSpec> {
        StatusFrameErrW::new(self, 6)
    }
    #[doc = "Bit 7 - Break error flag"]
    #[inline(always)]
    #[must_use]
    pub fn status_break_err(&mut self) -> StatusBreakErrW<StatusRegSpec> {
        StatusBreakErrW::new(self, 7)
    }
    #[doc = "Bit 8 - Indicates that the RXFIFO is almost full depending on RX_threshold register"]
    #[inline(always)]
    #[must_use]
    pub fn status_rx_almost_full(&mut self) -> StatusRxAlmostFullW<StatusRegSpec> {
        StatusRxAlmostFullW::new(self, 8)
    }
}
#[doc = "UART Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusRegSpec;
impl crate::RegisterSpec for StatusRegSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`status_reg::R`](R) reader structure"]
impl crate::Readable for StatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`status_reg::W`](W) writer structure"]
impl crate::Writable for StatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets STATUS_REG to value 0"]
impl crate::Resettable for StatusRegSpec {
    const RESET_VALUE: u16 = 0;
}
