#[doc = "Register `COMM_STATUS` reader"]
pub type R = crate::R<CommStatusSpec>;
#[doc = "Register `COMM_STATUS` writer"]
pub type W = crate::W<CommStatusSpec>;
#[doc = "Field `BUSY` reader - SPI Busy bit. This will be set when NCS goes low and will be reset when NCS goes high"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - SPI Busy bit. This will be set when NCS goes low and will be reset when NCS goes high"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_STARTED` reader - Transmit enable bit. This bit will be set when the transmit operation starts and will be reset once the transmit operation is complete"]
pub type TxStartedR = crate::BitReader;
#[doc = "Field `TX_STARTED` writer - Transmit enable bit. This bit will be set when the transmit operation starts and will be reset once the transmit operation is complete"]
pub type TxStartedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_STARTED` reader - Receive not enable bit. This bit will be reset when the receive operation starts and will be set once the receive operation is complete"]
pub type RxStartedR = crate::BitReader;
#[doc = "Field `RX_STARTED` writer - Receive not enable bit. This bit will be reset when the receive operation starts and will be set once the receive operation is complete"]
pub type RxStartedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DEPTH` reader - TX FIFO Threshold bits to know the number of entries in the TX FIFO"]
pub type TxDepthR = crate::FieldReader;
#[doc = "Field `TX_DEPTH` writer - TX FIFO Threshold bits to know the number of entries in the TX FIFO"]
pub type TxDepthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_DEPTH` reader - RX FIFO Threshold bits to know the number of entries in the RX FIFO."]
pub type RxDepthR = crate::FieldReader;
#[doc = "Field `RX_DEPTH` writer - RX FIFO Threshold bits to know the number of entries in the RX FIFO."]
pub type RxDepthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OVERRUN` reader - Overrun bit. This will be set when there is an overrun during receive operation"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `OVERRUN` writer - Overrun bit. This will be set when there is an overrun during receive operation"]
pub type OverrunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Busy bit. This will be set when NCS goes low and will be reset when NCS goes high"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit enable bit. This bit will be set when the transmit operation starts and will be reset once the transmit operation is complete"]
    #[inline(always)]
    pub fn tx_started(&self) -> TxStartedR {
        TxStartedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive not enable bit. This bit will be reset when the receive operation starts and will be set once the receive operation is complete"]
    #[inline(always)]
    pub fn rx_started(&self) -> RxStartedR {
        RxStartedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - TX FIFO Threshold bits to know the number of entries in the TX FIFO"]
    #[inline(always)]
    pub fn tx_depth(&self) -> TxDepthR {
        TxDepthR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - RX FIFO Threshold bits to know the number of entries in the RX FIFO."]
    #[inline(always)]
    pub fn rx_depth(&self) -> RxDepthR {
        RxDepthR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9 - Overrun bit. This will be set when there is an overrun during receive operation"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Busy bit. This will be set when NCS goes low and will be reset when NCS goes high"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<CommStatusSpec> {
        BusyW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit enable bit. This bit will be set when the transmit operation starts and will be reset once the transmit operation is complete"]
    #[inline(always)]
    #[must_use]
    pub fn tx_started(&mut self) -> TxStartedW<CommStatusSpec> {
        TxStartedW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive not enable bit. This bit will be reset when the receive operation starts and will be set once the receive operation is complete"]
    #[inline(always)]
    #[must_use]
    pub fn rx_started(&mut self) -> RxStartedW<CommStatusSpec> {
        RxStartedW::new(self, 2)
    }
    #[doc = "Bits 3:5 - TX FIFO Threshold bits to know the number of entries in the TX FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn tx_depth(&mut self) -> TxDepthW<CommStatusSpec> {
        TxDepthW::new(self, 3)
    }
    #[doc = "Bits 6:8 - RX FIFO Threshold bits to know the number of entries in the RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rx_depth(&mut self) -> RxDepthW<CommStatusSpec> {
        RxDepthW::new(self, 6)
    }
    #[doc = "Bit 9 - Overrun bit. This will be set when there is an overrun during receive operation"]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OverrunW<CommStatusSpec> {
        OverrunW::new(self, 9)
    }
}
#[doc = "Status of SPI communication\n\nYou can [`read`](crate::Reg::read) this register and get [`comm_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comm_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommStatusSpec;
impl crate::RegisterSpec for CommStatusSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`comm_status::R`](R) reader structure"]
impl crate::Readable for CommStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`comm_status::W`](W) writer structure"]
impl crate::Writable for CommStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets COMM_STATUS to value 0"]
impl crate::Resettable for CommStatusSpec {
    const RESET_VALUE: u16 = 0;
}
