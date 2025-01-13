#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TEF` reader - Transfer error flag"]
pub type TefR = crate::BitReader;
#[doc = "Field `TCF` reader - Transfer complete flag"]
pub type TcfR = crate::BitReader;
#[doc = "Field `FTF` reader - FIFO threshold flag"]
pub type FtfR = crate::BitReader;
#[doc = "Field `SMF` reader - Status match flag"]
pub type SmfR = crate::BitReader;
#[doc = "Field `TOF` reader - Timeout flag"]
pub type TofR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `FLEVEL` reader - FIFO level"]
pub type FlevelR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transfer error flag"]
    #[inline(always)]
    pub fn tef(&self) -> TefR {
        TefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO threshold flag"]
    #[inline(always)]
    pub fn ftf(&self) -> FtfR {
        FtfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status match flag"]
    #[inline(always)]
    pub fn smf(&self) -> SmfR {
        SmfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout flag"]
    #[inline(always)]
    pub fn tof(&self) -> TofR {
        TofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:14 - FIFO level"]
    #[inline(always)]
    pub fn flevel(&self) -> FlevelR {
        FlevelR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
