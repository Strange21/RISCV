#[doc = "Register `FIFO_STATUS` reader"]
pub type R = crate::R<FifoStatusSpec>;
#[doc = "Register `FIFO_STATUS` writer"]
pub type W = crate::W<FifoStatusSpec>;
#[doc = "Field `TX_FIFO_EMPTY` reader - TX FIFO empty. TX FIFO is empty"]
pub type TxFifoEmptyR = crate::BitReader;
#[doc = "Field `TX_FIFO_EMPTY` writer - TX FIFO empty. TX FIFO is empty"]
pub type TxFifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_DUAL` reader - TX FIFO full. TX FIFO is filled by 2 entries"]
pub type TxFifoDualR = crate::BitReader;
#[doc = "Field `TX_FIFO_DUAL` writer - TX FIFO full. TX FIFO is filled by 2 entries"]
pub type TxFifoDualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_QUAD` reader - TX FIFO full. TX FIFO is filled by 4 entries"]
pub type TxFifoQuadR = crate::BitReader;
#[doc = "Field `TX_FIFO_QUAD` writer - TX FIFO full. TX FIFO is filled by 4 entries"]
pub type TxFifoQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_OCTAL` reader - TX FIFO full. TX FIFO is filled by 8 entries"]
pub type TxFifoOctalR = crate::BitReader;
#[doc = "Field `TX_FIFO_OCTAL` writer - TX FIFO full. TX FIFO is filled by 8 entries"]
pub type TxFifoOctalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_HALF` reader - TX FIFO full. TX FIFO is filled by 16 entries"]
pub type TxFifoHalfR = crate::BitReader;
#[doc = "Field `TX_FIFO_HALF` writer - TX FIFO full. TX FIFO is filled by 16 entries"]
pub type TxFifoHalfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_24` reader - TX FIFO full. TX FIFO is filled by 24 entries"]
pub type TxFifo24R = crate::BitReader;
#[doc = "Field `TX_FIFO_24` writer - TX FIFO full. TX FIFO is filled by 24 entries"]
pub type TxFifo24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_28` reader - TX FIFO full. TX FIFO is filled by 28 entries"]
pub type TxFifo28R = crate::BitReader;
#[doc = "Field `TX_FIFO_28` writer - TX FIFO full. TX FIFO is filled by 28 entries"]
pub type TxFifo28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_30` reader - TX FIFO full. TX FIFO is filled by 30 entries"]
pub type TxFifo30R = crate::BitReader;
#[doc = "Field `TX_FIFO_30` writer - TX FIFO full. TX FIFO is filled by 30 entries"]
pub type TxFifo30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_FULL` reader - TX FIFO full. TX FIFO is full - 32 entries"]
pub type TxFifoFullR = crate::BitReader;
#[doc = "Field `TX_FIFO_FULL` writer - TX FIFO full. TX FIFO is full - 32 entries"]
pub type TxFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_EMPTY` reader - RX FIFO empty. RX FIFO is empty"]
pub type RxFifoEmptyR = crate::BitReader;
#[doc = "Field `RX_FIFO_EMPTY` writer - RX FIFO empty. RX FIFO is empty"]
pub type RxFifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_DUAL` reader - RX FIFO full. RX FIFO is filled by 2 entries"]
pub type RxFifoDualR = crate::BitReader;
#[doc = "Field `RX_FIFO_DUAL` writer - RX FIFO full. RX FIFO is filled by 2 entries"]
pub type RxFifoDualW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_QUAD` reader - RX FIFO full. RX FIFO is filled by 4 entries"]
pub type RxFifoQuadR = crate::BitReader;
#[doc = "Field `RX_FIFO_QUAD` writer - RX FIFO full. RX FIFO is filled by 4 entries"]
pub type RxFifoQuadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_OCTAL` reader - RX FIFO full. RX FIFO is filled by 8 entries"]
pub type RxFifoOctalR = crate::BitReader;
#[doc = "Field `RX_FIFO_OCTAL` writer - RX FIFO full. RX FIFO is filled by 8 entries"]
pub type RxFifoOctalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_HALF` reader - RX FIFO full. RX FIFO is filled by 16 entries"]
pub type RxFifoHalfR = crate::BitReader;
#[doc = "Field `RX_FIFO_HALF` writer - RX FIFO full. RX FIFO is filled by 16 entries"]
pub type RxFifoHalfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_24` reader - RX FIFO full. RX FIFO is filled by 24 entries"]
pub type RxFifo24R = crate::BitReader;
#[doc = "Field `RX_FIFO_24` writer - RX FIFO full. RX FIFO is filled by 24 entries"]
pub type RxFifo24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_28` reader - RX FIFO full. RX FIFO is filled by 28 entries"]
pub type RxFifo28R = crate::BitReader;
#[doc = "Field `RX_FIFO_28` writer - RX FIFO full. RX FIFO is filled by 28 entries"]
pub type RxFifo28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_30` reader - RX FIFO full. RX FIFO is filled by 30 entries"]
pub type RxFifo30R = crate::BitReader;
#[doc = "Field `RX_FIFO_30` writer - RX FIFO full. RX FIFO is filled by 30 entries"]
pub type RxFifo30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_FULL` reader - RX FIFO full. RX FIFO is full - 32 entries"]
pub type RxFifoFullR = crate::BitReader;
#[doc = "Field `RX_FIFO_FULL` writer - RX FIFO full. RX FIFO is full - 32 entries"]
pub type RxFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TX FIFO empty. TX FIFO is empty"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TxFifoEmptyR {
        TxFifoEmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO full. TX FIFO is filled by 2 entries"]
    #[inline(always)]
    pub fn tx_fifo_dual(&self) -> TxFifoDualR {
        TxFifoDualR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO full. TX FIFO is filled by 4 entries"]
    #[inline(always)]
    pub fn tx_fifo_quad(&self) -> TxFifoQuadR {
        TxFifoQuadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FIFO full. TX FIFO is filled by 8 entries"]
    #[inline(always)]
    pub fn tx_fifo_octal(&self) -> TxFifoOctalR {
        TxFifoOctalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO full. TX FIFO is filled by 16 entries"]
    #[inline(always)]
    pub fn tx_fifo_half(&self) -> TxFifoHalfR {
        TxFifoHalfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX FIFO full. TX FIFO is filled by 24 entries"]
    #[inline(always)]
    pub fn tx_fifo_24(&self) -> TxFifo24R {
        TxFifo24R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO full. TX FIFO is filled by 28 entries"]
    #[inline(always)]
    pub fn tx_fifo_28(&self) -> TxFifo28R {
        TxFifo28R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO full. TX FIFO is filled by 30 entries"]
    #[inline(always)]
    pub fn tx_fifo_30(&self) -> TxFifo30R {
        TxFifo30R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TX FIFO full. TX FIFO is full - 32 entries"]
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> TxFifoFullR {
        TxFifoFullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RX FIFO empty. RX FIFO is empty"]
    #[inline(always)]
    pub fn rx_fifo_empty(&self) -> RxFifoEmptyR {
        RxFifoEmptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RX FIFO full. RX FIFO is filled by 2 entries"]
    #[inline(always)]
    pub fn rx_fifo_dual(&self) -> RxFifoDualR {
        RxFifoDualR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX FIFO full. RX FIFO is filled by 4 entries"]
    #[inline(always)]
    pub fn rx_fifo_quad(&self) -> RxFifoQuadR {
        RxFifoQuadR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RX FIFO full. RX FIFO is filled by 8 entries"]
    #[inline(always)]
    pub fn rx_fifo_octal(&self) -> RxFifoOctalR {
        RxFifoOctalR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RX FIFO full. RX FIFO is filled by 16 entries"]
    #[inline(always)]
    pub fn rx_fifo_half(&self) -> RxFifoHalfR {
        RxFifoHalfR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RX FIFO full. RX FIFO is filled by 24 entries"]
    #[inline(always)]
    pub fn rx_fifo_24(&self) -> RxFifo24R {
        RxFifo24R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RX FIFO full. RX FIFO is filled by 28 entries"]
    #[inline(always)]
    pub fn rx_fifo_28(&self) -> RxFifo28R {
        RxFifo28R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX FIFO full. RX FIFO is filled by 30 entries"]
    #[inline(always)]
    pub fn rx_fifo_30(&self) -> RxFifo30R {
        RxFifo30R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX FIFO full. RX FIFO is full - 32 entries"]
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> RxFifoFullR {
        RxFifoFullR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO empty. TX FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_empty(&mut self) -> TxFifoEmptyW<FifoStatusSpec> {
        TxFifoEmptyW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO full. TX FIFO is filled by 2 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_dual(&mut self) -> TxFifoDualW<FifoStatusSpec> {
        TxFifoDualW::new(self, 1)
    }
    #[doc = "Bit 2 - TX FIFO full. TX FIFO is filled by 4 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_quad(&mut self) -> TxFifoQuadW<FifoStatusSpec> {
        TxFifoQuadW::new(self, 2)
    }
    #[doc = "Bit 3 - TX FIFO full. TX FIFO is filled by 8 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_octal(&mut self) -> TxFifoOctalW<FifoStatusSpec> {
        TxFifoOctalW::new(self, 3)
    }
    #[doc = "Bit 4 - TX FIFO full. TX FIFO is filled by 16 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_half(&mut self) -> TxFifoHalfW<FifoStatusSpec> {
        TxFifoHalfW::new(self, 4)
    }
    #[doc = "Bit 5 - TX FIFO full. TX FIFO is filled by 24 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_24(&mut self) -> TxFifo24W<FifoStatusSpec> {
        TxFifo24W::new(self, 5)
    }
    #[doc = "Bit 6 - TX FIFO full. TX FIFO is filled by 28 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_28(&mut self) -> TxFifo28W<FifoStatusSpec> {
        TxFifo28W::new(self, 6)
    }
    #[doc = "Bit 7 - TX FIFO full. TX FIFO is filled by 30 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_30(&mut self) -> TxFifo30W<FifoStatusSpec> {
        TxFifo30W::new(self, 7)
    }
    #[doc = "Bit 8 - TX FIFO full. TX FIFO is full - 32 entries"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_full(&mut self) -> TxFifoFullW<FifoStatusSpec> {
        TxFifoFullW::new(self, 8)
    }
    #[doc = "Bit 9 - RX FIFO empty. RX FIFO is empty"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_empty(&mut self) -> RxFifoEmptyW<FifoStatusSpec> {
        RxFifoEmptyW::new(self, 9)
    }
    #[doc = "Bit 10 - RX FIFO full. RX FIFO is filled by 2 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_dual(&mut self) -> RxFifoDualW<FifoStatusSpec> {
        RxFifoDualW::new(self, 10)
    }
    #[doc = "Bit 11 - RX FIFO full. RX FIFO is filled by 4 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_quad(&mut self) -> RxFifoQuadW<FifoStatusSpec> {
        RxFifoQuadW::new(self, 11)
    }
    #[doc = "Bit 12 - RX FIFO full. RX FIFO is filled by 8 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_octal(&mut self) -> RxFifoOctalW<FifoStatusSpec> {
        RxFifoOctalW::new(self, 12)
    }
    #[doc = "Bit 13 - RX FIFO full. RX FIFO is filled by 16 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_half(&mut self) -> RxFifoHalfW<FifoStatusSpec> {
        RxFifoHalfW::new(self, 13)
    }
    #[doc = "Bit 14 - RX FIFO full. RX FIFO is filled by 24 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_24(&mut self) -> RxFifo24W<FifoStatusSpec> {
        RxFifo24W::new(self, 14)
    }
    #[doc = "Bit 15 - RX FIFO full. RX FIFO is filled by 28 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_28(&mut self) -> RxFifo28W<FifoStatusSpec> {
        RxFifo28W::new(self, 15)
    }
    #[doc = "Bit 16 - RX FIFO full. RX FIFO is filled by 30 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_30(&mut self) -> RxFifo30W<FifoStatusSpec> {
        RxFifo30W::new(self, 16)
    }
    #[doc = "Bit 17 - RX FIFO full. RX FIFO is full - 32 entries"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_full(&mut self) -> RxFifoFullW<FifoStatusSpec> {
        RxFifoFullW::new(self, 17)
    }
}
#[doc = "Gives the status of TX/RX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoStatusSpec;
impl crate::RegisterSpec for FifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_status::R`](R) reader structure"]
impl crate::Readable for FifoStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fifo_status::W`](W) writer structure"]
impl crate::Writable for FifoStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_STATUS to value 0"]
impl crate::Resettable for FifoStatusSpec {
    const RESET_VALUE: u32 = 0;
}
