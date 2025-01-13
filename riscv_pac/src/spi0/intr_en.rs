#[doc = "Register `INTR_EN` reader"]
pub type R = crate::R<IntrEnSpec>;
#[doc = "Register `INTR_EN` writer"]
pub type W = crate::W<IntrEnSpec>;
#[doc = "Field `TX_FIFO_EMPTY_INTR_EN` reader - TX FIFO empty interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is empty"]
pub type TxFifoEmptyIntrEnR = crate::BitReader;
#[doc = "Field `TX_FIFO_EMPTY_INTR_EN` writer - TX FIFO empty interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is empty"]
pub type TxFifoEmptyIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_DUAL_INTR_EN` reader - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 2 entries"]
pub type TxFifoDualIntrEnR = crate::BitReader;
#[doc = "Field `TX_FIFO_DUAL_INTR_EN` writer - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 2 entries"]
pub type TxFifoDualIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_QUAD_INTR_EN` reader - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 4 entries"]
pub type TxFifoQuadIntrEnR = crate::BitReader;
#[doc = "Field `TX_FIFO_QUAD_INTR_EN` writer - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 4 entries"]
pub type TxFifoQuadIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_OCTAL_INTR_EN` reader - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 8 entries"]
pub type TxFifoOctalIntrEnR = crate::BitReader;
#[doc = "Field `TX_FIFO_OCTAL_INTR_EN` writer - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 8 entries"]
pub type TxFifoOctalIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_HALF_INTR_EN` reader - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 16 entries"]
pub type TxFifoHalfIntrEnR = crate::BitReader;
#[doc = "Field `TX_FIFO_HALF_INTR_EN` writer - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 16 entries"]
pub type TxFifoHalfIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_24_INTR_EN` reader - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 24 entries"]
pub type TxFifo24IntrEnR = crate::BitReader;
#[doc = "Field `TX_FIFO_24_INTR_EN` writer - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 24 entries"]
pub type TxFifo24IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_28_INTR_EN` reader - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 28 entries"]
pub type TxFifo28IntrEnR = crate::BitReader;
#[doc = "Field `TX_FIFO_28_INTR_EN` writer - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 28 entries"]
pub type TxFifo28IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_30_INTR_EN` reader - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 30 entries"]
pub type TxFifo30IntrEnR = crate::BitReader;
#[doc = "Field `TX_FIFO_30_INTR_EN` writer - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 30 entries"]
pub type TxFifo30IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_FULL_INTR_EN` reader - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is full - 32 entries"]
pub type TxFifoFullIntrEnR = crate::BitReader;
#[doc = "Field `TX_FIFO_FULL_INTR_EN` writer - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is full - 32 entries"]
pub type TxFifoFullIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_EMPTY_INTR_EN` reader - RX FIFO empty interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is empty"]
pub type RxFifoEmptyIntrEnR = crate::BitReader;
#[doc = "Field `RX_FIFO_EMPTY_INTR_EN` writer - RX FIFO empty interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is empty"]
pub type RxFifoEmptyIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_DUAL_INTR_EN` reader - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 2 entries"]
pub type RxFifoDualIntrEnR = crate::BitReader;
#[doc = "Field `RX_FIFO_DUAL_INTR_EN` writer - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 2 entries"]
pub type RxFifoDualIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_QUAD_INTR_EN` reader - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 4 entries"]
pub type RxFifoQuadIntrEnR = crate::BitReader;
#[doc = "Field `RX_FIFO_QUAD_INTR_EN` writer - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 4 entries"]
pub type RxFifoQuadIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_OCTAL_INTR_EN` reader - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 8 entries"]
pub type RxFifoOctalIntrEnR = crate::BitReader;
#[doc = "Field `RX_FIFO_OCTAL_INTR_EN` writer - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 8 entries"]
pub type RxFifoOctalIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_HALF_INTR_EN` reader - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 16 entries"]
pub type RxFifoHalfIntrEnR = crate::BitReader;
#[doc = "Field `RX_FIFO_HALF_INTR_EN` writer - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 16 entries"]
pub type RxFifoHalfIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_24_INTR_EN` reader - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 24 entries"]
pub type RxFifo24IntrEnR = crate::BitReader;
#[doc = "Field `RX_FIFO_24_INTR_EN` writer - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 24 entries"]
pub type RxFifo24IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_28_INTR_EN` reader - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 28 entries"]
pub type RxFifo28IntrEnR = crate::BitReader;
#[doc = "Field `RX_FIFO_28_INTR_EN` writer - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 28 entries"]
pub type RxFifo28IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_30_INTR_EN` reader - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 30 entries"]
pub type RxFifo30IntrEnR = crate::BitReader;
#[doc = "Field `RX_FIFO_30_INTR_EN` writer - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 30 entries"]
pub type RxFifo30IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_FULL_INTR_EN` reader - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is full - 32 entries"]
pub type RxFifoFullIntrEnR = crate::BitReader;
#[doc = "Field `RX_FIFO_FULL_INTR_EN` writer - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is full - 32 entries"]
pub type RxFifoFullIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX FIFO empty interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is empty"]
    #[inline(always)]
    pub fn tx_fifo_empty_intr_en(&self) -> TxFifoEmptyIntrEnR {
        TxFifoEmptyIntrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 2 entries"]
    #[inline(always)]
    pub fn tx_fifo_dual_intr_en(&self) -> TxFifoDualIntrEnR {
        TxFifoDualIntrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 4 entries"]
    #[inline(always)]
    pub fn tx_fifo_quad_intr_en(&self) -> TxFifoQuadIntrEnR {
        TxFifoQuadIntrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 8 entries"]
    #[inline(always)]
    pub fn tx_fifo_octal_intr_en(&self) -> TxFifoOctalIntrEnR {
        TxFifoOctalIntrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 16 entries"]
    #[inline(always)]
    pub fn tx_fifo_half_intr_en(&self) -> TxFifoHalfIntrEnR {
        TxFifoHalfIntrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 24 entries"]
    #[inline(always)]
    pub fn tx_fifo_24_intr_en(&self) -> TxFifo24IntrEnR {
        TxFifo24IntrEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 28 entries"]
    #[inline(always)]
    pub fn tx_fifo_28_intr_en(&self) -> TxFifo28IntrEnR {
        TxFifo28IntrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 30 entries"]
    #[inline(always)]
    pub fn tx_fifo_30_intr_en(&self) -> TxFifo30IntrEnR {
        TxFifo30IntrEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is full - 32 entries"]
    #[inline(always)]
    pub fn tx_fifo_full_intr_en(&self) -> TxFifoFullIntrEnR {
        TxFifoFullIntrEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX FIFO empty interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is empty"]
    #[inline(always)]
    pub fn rx_fifo_empty_intr_en(&self) -> RxFifoEmptyIntrEnR {
        RxFifoEmptyIntrEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 2 entries"]
    #[inline(always)]
    pub fn rx_fifo_dual_intr_en(&self) -> RxFifoDualIntrEnR {
        RxFifoDualIntrEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 4 entries"]
    #[inline(always)]
    pub fn rx_fifo_quad_intr_en(&self) -> RxFifoQuadIntrEnR {
        RxFifoQuadIntrEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 8 entries"]
    #[inline(always)]
    pub fn rx_fifo_octal_intr_en(&self) -> RxFifoOctalIntrEnR {
        RxFifoOctalIntrEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 16 entries"]
    #[inline(always)]
    pub fn rx_fifo_half_intr_en(&self) -> RxFifoHalfIntrEnR {
        RxFifoHalfIntrEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 24 entries"]
    #[inline(always)]
    pub fn rx_fifo_24_intr_en(&self) -> RxFifo24IntrEnR {
        RxFifo24IntrEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 28 entries"]
    #[inline(always)]
    pub fn rx_fifo_28_intr_en(&self) -> RxFifo28IntrEnR {
        RxFifo28IntrEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 30 entries"]
    #[inline(always)]
    pub fn rx_fifo_30_intr_en(&self) -> RxFifo30IntrEnR {
        RxFifo30IntrEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is full - 32 entries"]
    #[inline(always)]
    pub fn rx_fifo_full_intr_en(&self) -> RxFifoFullIntrEnR {
        RxFifoFullIntrEnR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO empty interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_empty_intr_en(&mut self) -> TxFifoEmptyIntrEnW<IntrEnSpec> {
        TxFifoEmptyIntrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 2 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_dual_intr_en(&mut self) -> TxFifoDualIntrEnW<IntrEnSpec> {
        TxFifoDualIntrEnW::new(self, 1)
    }
    #[doc = "Bit 2 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 4 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_quad_intr_en(&mut self) -> TxFifoQuadIntrEnW<IntrEnSpec> {
        TxFifoQuadIntrEnW::new(self, 2)
    }
    #[doc = "Bit 3 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 8 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_octal_intr_en(&mut self) -> TxFifoOctalIntrEnW<IntrEnSpec> {
        TxFifoOctalIntrEnW::new(self, 3)
    }
    #[doc = "Bit 4 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 16 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_half_intr_en(&mut self) -> TxFifoHalfIntrEnW<IntrEnSpec> {
        TxFifoHalfIntrEnW::new(self, 4)
    }
    #[doc = "Bit 5 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 24 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_24_intr_en(&mut self) -> TxFifo24IntrEnW<IntrEnSpec> {
        TxFifo24IntrEnW::new(self, 5)
    }
    #[doc = "Bit 6 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 28 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_28_intr_en(&mut self) -> TxFifo28IntrEnW<IntrEnSpec> {
        TxFifo28IntrEnW::new(self, 6)
    }
    #[doc = "Bit 7 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is filled by 30 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_30_intr_en(&mut self) -> TxFifo30IntrEnW<IntrEnSpec> {
        TxFifo30IntrEnW::new(self, 7)
    }
    #[doc = "Bit 8 - TX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when TX FIFO is full - 32 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_full_intr_en(&mut self) -> TxFifoFullIntrEnW<IntrEnSpec> {
        TxFifoFullIntrEnW::new(self, 8)
    }
    #[doc = "Bit 9 - RX FIFO empty interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_empty_intr_en(&mut self) -> RxFifoEmptyIntrEnW<IntrEnSpec> {
        RxFifoEmptyIntrEnW::new(self, 9)
    }
    #[doc = "Bit 10 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 2 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_dual_intr_en(&mut self) -> RxFifoDualIntrEnW<IntrEnSpec> {
        RxFifoDualIntrEnW::new(self, 10)
    }
    #[doc = "Bit 11 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 4 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_quad_intr_en(&mut self) -> RxFifoQuadIntrEnW<IntrEnSpec> {
        RxFifoQuadIntrEnW::new(self, 11)
    }
    #[doc = "Bit 12 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 8 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_octal_intr_en(&mut self) -> RxFifoOctalIntrEnW<IntrEnSpec> {
        RxFifoOctalIntrEnW::new(self, 12)
    }
    #[doc = "Bit 13 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 16 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_half_intr_en(&mut self) -> RxFifoHalfIntrEnW<IntrEnSpec> {
        RxFifoHalfIntrEnW::new(self, 13)
    }
    #[doc = "Bit 14 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 24 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_24_intr_en(&mut self) -> RxFifo24IntrEnW<IntrEnSpec> {
        RxFifo24IntrEnW::new(self, 14)
    }
    #[doc = "Bit 15 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 28 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_28_intr_en(&mut self) -> RxFifo28IntrEnW<IntrEnSpec> {
        RxFifo28IntrEnW::new(self, 15)
    }
    #[doc = "Bit 16 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is filled by 30 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_30_intr_en(&mut self) -> RxFifo30IntrEnW<IntrEnSpec> {
        RxFifo30IntrEnW::new(self, 16)
    }
    #[doc = "Bit 17 - RX FIFO full interrupt enable bit. This when set, interrupt is sent to PLIC when RX FIFO is full - 32 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_full_intr_en(&mut self) -> RxFifoFullIntrEnW<IntrEnSpec> {
        RxFifoFullIntrEnW::new(self, 17)
    }
}
#[doc = "Spi interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrEnSpec;
impl crate::RegisterSpec for IntrEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_en::R`](R) reader structure"]
impl crate::Readable for IntrEnSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_en::W`](W) writer structure"]
impl crate::Writable for IntrEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_EN to value 0"]
impl crate::Resettable for IntrEnSpec {
    const RESET_VALUE: u32 = 0;
}
