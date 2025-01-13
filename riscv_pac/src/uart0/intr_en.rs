#[doc = "Register `INTR_EN` reader"]
pub type R = crate::R<IntrEnSpec>;
#[doc = "Register `INTR_EN` writer"]
pub type W = crate::W<IntrEnSpec>;
#[doc = "Field `INTR_TX_EMPTY_EN` reader - Enable for interrupt of transmission fifo empty"]
pub type IntrTxEmptyEnR = crate::BitReader;
#[doc = "Field `INTR_TX_EMPTY_EN` writer - Enable for interrupt of transmission fifo empty"]
pub type IntrTxEmptyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_TX_FULL_EN` reader - Enable for interrupt of transmission fifo full"]
pub type IntrTxFullEnR = crate::BitReader;
#[doc = "Field `INTR_TX_FULL_EN` writer - Enable for interrupt of transmission fifo full"]
pub type IntrTxFullEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_RX_NOT_EMPTY_EN` reader - Enable for interrupt of receiver fifo not empty"]
pub type IntrRxNotEmptyEnR = crate::BitReader;
#[doc = "Field `INTR_RX_NOT_EMPTY_EN` writer - Enable for interrupt of receiver fifo not empty"]
pub type IntrRxNotEmptyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_RX_FULL_EN` reader - Enable for interrupt of receiver fifo full"]
pub type IntrRxFullEnR = crate::BitReader;
#[doc = "Field `INTR_RX_FULL_EN` writer - Enable for interrupt of receiver fifo full"]
pub type IntrRxFullEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_PARITY_EN` reader - Enable for interrupt of parity error"]
pub type IntrParityEnR = crate::BitReader;
#[doc = "Field `INTR_PARITY_EN` writer - Enable for interrupt of parity error"]
pub type IntrParityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_OVERRUN_EN` reader - Enable for interrupt of overrun error"]
pub type IntrOverrunEnR = crate::BitReader;
#[doc = "Field `INTR_OVERRUN_EN` writer - Enable for interrupt of overrun error"]
pub type IntrOverrunEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_FRAME_EN` reader - Enable for interrupt of frame error"]
pub type IntrFrameEnR = crate::BitReader;
#[doc = "Field `INTR_FRAME_EN` writer - Enable for interrupt of frame error"]
pub type IntrFrameEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_BREAK_EN` reader - Enable for interrupt of break error"]
pub type IntrBreakEnR = crate::BitReader;
#[doc = "Field `INTR_BREAK_EN` writer - Enable for interrupt of break error"]
pub type IntrBreakEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR_RX_ALMOST_FULL` reader - Enable for interrupt RX fifo almost full depending on the RX_threshold register"]
pub type IntrRxAlmostFullR = crate::BitReader;
#[doc = "Field `INTR_RX_ALMOST_FULL` writer - Enable for interrupt RX fifo almost full depending on the RX_threshold register"]
pub type IntrRxAlmostFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable for interrupt of transmission fifo empty"]
    #[inline(always)]
    pub fn intr_tx_empty_en(&self) -> IntrTxEmptyEnR {
        IntrTxEmptyEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable for interrupt of transmission fifo full"]
    #[inline(always)]
    pub fn intr_tx_full_en(&self) -> IntrTxFullEnR {
        IntrTxFullEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable for interrupt of receiver fifo not empty"]
    #[inline(always)]
    pub fn intr_rx_not_empty_en(&self) -> IntrRxNotEmptyEnR {
        IntrRxNotEmptyEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for interrupt of receiver fifo full"]
    #[inline(always)]
    pub fn intr_rx_full_en(&self) -> IntrRxFullEnR {
        IntrRxFullEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable for interrupt of parity error"]
    #[inline(always)]
    pub fn intr_parity_en(&self) -> IntrParityEnR {
        IntrParityEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable for interrupt of overrun error"]
    #[inline(always)]
    pub fn intr_overrun_en(&self) -> IntrOverrunEnR {
        IntrOverrunEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable for interrupt of frame error"]
    #[inline(always)]
    pub fn intr_frame_en(&self) -> IntrFrameEnR {
        IntrFrameEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable for interrupt of break error"]
    #[inline(always)]
    pub fn intr_break_en(&self) -> IntrBreakEnR {
        IntrBreakEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable for interrupt RX fifo almost full depending on the RX_threshold register"]
    #[inline(always)]
    pub fn intr_rx_almost_full(&self) -> IntrRxAlmostFullR {
        IntrRxAlmostFullR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for interrupt of transmission fifo empty"]
    #[inline(always)]
    #[must_use]
    pub fn intr_tx_empty_en(&mut self) -> IntrTxEmptyEnW<IntrEnSpec> {
        IntrTxEmptyEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable for interrupt of transmission fifo full"]
    #[inline(always)]
    #[must_use]
    pub fn intr_tx_full_en(&mut self) -> IntrTxFullEnW<IntrEnSpec> {
        IntrTxFullEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable for interrupt of receiver fifo not empty"]
    #[inline(always)]
    #[must_use]
    pub fn intr_rx_not_empty_en(&mut self) -> IntrRxNotEmptyEnW<IntrEnSpec> {
        IntrRxNotEmptyEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable for interrupt of receiver fifo full"]
    #[inline(always)]
    #[must_use]
    pub fn intr_rx_full_en(&mut self) -> IntrRxFullEnW<IntrEnSpec> {
        IntrRxFullEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable for interrupt of parity error"]
    #[inline(always)]
    #[must_use]
    pub fn intr_parity_en(&mut self) -> IntrParityEnW<IntrEnSpec> {
        IntrParityEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable for interrupt of overrun error"]
    #[inline(always)]
    #[must_use]
    pub fn intr_overrun_en(&mut self) -> IntrOverrunEnW<IntrEnSpec> {
        IntrOverrunEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable for interrupt of frame error"]
    #[inline(always)]
    #[must_use]
    pub fn intr_frame_en(&mut self) -> IntrFrameEnW<IntrEnSpec> {
        IntrFrameEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable for interrupt of break error"]
    #[inline(always)]
    #[must_use]
    pub fn intr_break_en(&mut self) -> IntrBreakEnW<IntrEnSpec> {
        IntrBreakEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable for interrupt RX fifo almost full depending on the RX_threshold register"]
    #[inline(always)]
    #[must_use]
    pub fn intr_rx_almost_full(&mut self) -> IntrRxAlmostFullW<IntrEnSpec> {
        IntrRxAlmostFullW::new(self, 8)
    }
}
#[doc = "Interrupts enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrEnSpec;
impl crate::RegisterSpec for IntrEnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intr_en::R`](R) reader structure"]
impl crate::Readable for IntrEnSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_en::W`](W) writer structure"]
impl crate::Writable for IntrEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets INTR_EN to value 0"]
impl crate::Resettable for IntrEnSpec {
    const RESET_VALUE: u16 = 0;
}
