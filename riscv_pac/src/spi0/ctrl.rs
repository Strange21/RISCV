#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `EN` reader - holds the spi enable control. Once set, spi transaction will start and it will be reset at the end of spi transaction"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - holds the spi enable control. Once set, spi transaction will start and it will be reset at the end of spi transaction"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBFIRST` reader - holds whether the spi transaction is LSB first. If set LSB first else MSB first"]
pub type LsbfirstR = crate::BitReader;
#[doc = "Field `LSBFIRST` writer - holds whether the spi transaction is LSB first. If set LSB first else MSB first"]
pub type LsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMM_MODE` reader - holds the communication mode of the spi transaction. 00 - only transmit; 01 - only receive; 10 - transmit and immediate receive; 11 - transmit and receive"]
pub type CommModeR = crate::FieldReader;
#[doc = "Field `COMM_MODE` writer - holds the communication mode of the spi transaction. 00 - only transmit; 01 - only receive; 10 - transmit and immediate receive; 11 - transmit and receive"]
pub type CommModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOTAL_BIT_TX` reader - holds the total number of bits to be received in a spi transaction"]
pub type TotalBitTxR = crate::FieldReader;
#[doc = "Field `TOTAL_BIT_TX` writer - holds the total number of bits to be received in a spi transaction"]
pub type TotalBitTxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TOTAL_BIT_RX` reader - holds the total number of bits to be transmitted in a spi transaction"]
pub type TotalBitRxR = crate::FieldReader;
#[doc = "Field `TOTAL_BIT_RX` writer - holds the total number of bits to be transmitted in a spi transaction"]
pub type TotalBitRxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLK_OUTEN` reader - holds the SCLK pin's output enable. If set, the controller generates the sclk else sclk is expected from the spi device"]
pub type SclkOutenR = crate::BitReader;
#[doc = "Field `SCLK_OUTEN` writer - holds the SCLK pin's output enable. If set, the controller generates the sclk else sclk is expected from the spi device"]
pub type SclkOutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NCS_OUTEN` reader - holds the NCS pin's output enable. If set, the controller generates the ncs else ncs is expected from the spi device"]
pub type NcsOutenR = crate::BitReader;
#[doc = "Field `NCS_OUTEN` writer - holds the NCS pin's output enable. If set, the controller generates the ncs else ncs is expected from the spi device"]
pub type NcsOutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISO_OUTEN` reader - holds the MISO pin's output enable. If set, output is transmitted through this pin else input is read from this pin"]
pub type MisoOutenR = crate::BitReader;
#[doc = "Field `MISO_OUTEN` writer - holds the MISO pin's output enable. If set, output is transmitted through this pin else input is read from this pin"]
pub type MisoOutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSI_OUTEN` reader - holds the MOSI pin's output enable. If set, output is transmitted through this pin else input is read from this pin"]
pub type MosiOutenR = crate::BitReader;
#[doc = "Field `MOSI_OUTEN` writer - holds the MOSI pin's output enable. If set, output is transmitted through this pin else input is read from this pin"]
pub type MosiOutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - holds the spi enable control. Once set, spi transaction will start and it will be reset at the end of spi transaction"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - holds whether the spi transaction is LSB first. If set LSB first else MSB first"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LsbfirstR {
        LsbfirstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - holds the communication mode of the spi transaction. 00 - only transmit; 01 - only receive; 10 - transmit and immediate receive; 11 - transmit and receive"]
    #[inline(always)]
    pub fn comm_mode(&self) -> CommModeR {
        CommModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:13 - holds the total number of bits to be received in a spi transaction"]
    #[inline(always)]
    pub fn total_bit_tx(&self) -> TotalBitTxR {
        TotalBitTxR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:21 - holds the total number of bits to be transmitted in a spi transaction"]
    #[inline(always)]
    pub fn total_bit_rx(&self) -> TotalBitRxR {
        TotalBitRxR::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bit 22 - holds the SCLK pin's output enable. If set, the controller generates the sclk else sclk is expected from the spi device"]
    #[inline(always)]
    pub fn sclk_outen(&self) -> SclkOutenR {
        SclkOutenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - holds the NCS pin's output enable. If set, the controller generates the ncs else ncs is expected from the spi device"]
    #[inline(always)]
    pub fn ncs_outen(&self) -> NcsOutenR {
        NcsOutenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - holds the MISO pin's output enable. If set, output is transmitted through this pin else input is read from this pin"]
    #[inline(always)]
    pub fn miso_outen(&self) -> MisoOutenR {
        MisoOutenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - holds the MOSI pin's output enable. If set, output is transmitted through this pin else input is read from this pin"]
    #[inline(always)]
    pub fn mosi_outen(&self) -> MosiOutenR {
        MosiOutenR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - holds the spi enable control. Once set, spi transaction will start and it will be reset at the end of spi transaction"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 1)
    }
    #[doc = "Bit 2 - holds whether the spi transaction is LSB first. If set LSB first else MSB first"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LsbfirstW<CtrlSpec> {
        LsbfirstW::new(self, 2)
    }
    #[doc = "Bits 4:5 - holds the communication mode of the spi transaction. 00 - only transmit; 01 - only receive; 10 - transmit and immediate receive; 11 - transmit and receive"]
    #[inline(always)]
    #[must_use]
    pub fn comm_mode(&mut self) -> CommModeW<CtrlSpec> {
        CommModeW::new(self, 4)
    }
    #[doc = "Bits 6:13 - holds the total number of bits to be received in a spi transaction"]
    #[inline(always)]
    #[must_use]
    pub fn total_bit_tx(&mut self) -> TotalBitTxW<CtrlSpec> {
        TotalBitTxW::new(self, 6)
    }
    #[doc = "Bits 14:21 - holds the total number of bits to be transmitted in a spi transaction"]
    #[inline(always)]
    #[must_use]
    pub fn total_bit_rx(&mut self) -> TotalBitRxW<CtrlSpec> {
        TotalBitRxW::new(self, 14)
    }
    #[doc = "Bit 22 - holds the SCLK pin's output enable. If set, the controller generates the sclk else sclk is expected from the spi device"]
    #[inline(always)]
    #[must_use]
    pub fn sclk_outen(&mut self) -> SclkOutenW<CtrlSpec> {
        SclkOutenW::new(self, 22)
    }
    #[doc = "Bit 23 - holds the NCS pin's output enable. If set, the controller generates the ncs else ncs is expected from the spi device"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_outen(&mut self) -> NcsOutenW<CtrlSpec> {
        NcsOutenW::new(self, 23)
    }
    #[doc = "Bit 24 - holds the MISO pin's output enable. If set, output is transmitted through this pin else input is read from this pin"]
    #[inline(always)]
    #[must_use]
    pub fn miso_outen(&mut self) -> MisoOutenW<CtrlSpec> {
        MisoOutenW::new(self, 24)
    }
    #[doc = "Bit 25 - holds the MOSI pin's output enable. If set, output is transmitted through this pin else input is read from this pin"]
    #[inline(always)]
    #[must_use]
    pub fn mosi_outen(&mut self) -> MosiOutenW<CtrlSpec> {
        MosiOutenW::new(self, 25)
    }
}
#[doc = "SPI communication control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
