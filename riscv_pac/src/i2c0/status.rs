#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `STATUS_BB` reader - Bus Busy bit - Indicates that the bus is busy(0 = busy). Also used in multi master systems only. 0 = busy"]
pub type StatusBbR = crate::BitReader;
#[doc = "Field `STATUS_BB` writer - Bus Busy bit - Indicates that the bus is busy(0 = busy). Also used in multi master systems only. 0 = busy"]
pub type StatusBbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_LAB` reader - Lost Arbitration bit - Used in Multiple Master systems only to denote that the master lost the arbitration"]
pub type StatusLabR = crate::BitReader;
#[doc = "Field `STATUS_LAB` writer - Lost Arbitration bit - Used in Multiple Master systems only to denote that the master lost the arbitration"]
pub type StatusLabW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_AAS` reader - Addressed as slave - Used in Slave Receiver mode"]
pub type StatusAasR = crate::BitReader;
#[doc = "Field `STATUS_AAS` writer - Addressed as slave - Used in Slave Receiver mode"]
pub type StatusAasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_AD0_LRB` reader - LRB - holds the last received bit through I2C bus. AD0 - Generall Call bit used for broadcast. Valid only while PIN=0"]
pub type StatusAd0LrbR = crate::BitReader;
#[doc = "Field `STATUS_AD0_LRB` writer - LRB - holds the last received bit through I2C bus. AD0 - Generall Call bit used for broadcast. Valid only while PIN=0"]
pub type StatusAd0LrbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_BER` reader - Bus Error - Set to 1 when there is a misplaced START, STOP bit"]
pub type StatusBerR = crate::BitReader;
#[doc = "Field `STATUS_BER` writer - Bus Error - Set to 1 when there is a misplaced START, STOP bit"]
pub type StatusBerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATUS_STS` reader - Read the PIN bit of control register"]
pub type StatusStsR = crate::BitReader;
#[doc = "Field `STATUS_STS` writer - Read the PIN bit of control register"]
pub type StatusStsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bus Busy bit - Indicates that the bus is busy(0 = busy). Also used in multi master systems only. 0 = busy"]
    #[inline(always)]
    pub fn status_bb(&self) -> StatusBbR {
        StatusBbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lost Arbitration bit - Used in Multiple Master systems only to denote that the master lost the arbitration"]
    #[inline(always)]
    pub fn status_lab(&self) -> StatusLabR {
        StatusLabR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Addressed as slave - Used in Slave Receiver mode"]
    #[inline(always)]
    pub fn status_aas(&self) -> StatusAasR {
        StatusAasR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LRB - holds the last received bit through I2C bus. AD0 - Generall Call bit used for broadcast. Valid only while PIN=0"]
    #[inline(always)]
    pub fn status_ad0_lrb(&self) -> StatusAd0LrbR {
        StatusAd0LrbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Error - Set to 1 when there is a misplaced START, STOP bit"]
    #[inline(always)]
    pub fn status_ber(&self) -> StatusBerR {
        StatusBerR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Read the PIN bit of control register"]
    #[inline(always)]
    pub fn status_sts(&self) -> StatusStsR {
        StatusStsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bus Busy bit - Indicates that the bus is busy(0 = busy). Also used in multi master systems only. 0 = busy"]
    #[inline(always)]
    #[must_use]
    pub fn status_bb(&mut self) -> StatusBbW<StatusSpec> {
        StatusBbW::new(self, 0)
    }
    #[doc = "Bit 1 - Lost Arbitration bit - Used in Multiple Master systems only to denote that the master lost the arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn status_lab(&mut self) -> StatusLabW<StatusSpec> {
        StatusLabW::new(self, 1)
    }
    #[doc = "Bit 2 - Addressed as slave - Used in Slave Receiver mode"]
    #[inline(always)]
    #[must_use]
    pub fn status_aas(&mut self) -> StatusAasW<StatusSpec> {
        StatusAasW::new(self, 2)
    }
    #[doc = "Bit 3 - LRB - holds the last received bit through I2C bus. AD0 - Generall Call bit used for broadcast. Valid only while PIN=0"]
    #[inline(always)]
    #[must_use]
    pub fn status_ad0_lrb(&mut self) -> StatusAd0LrbW<StatusSpec> {
        StatusAd0LrbW::new(self, 3)
    }
    #[doc = "Bit 4 - Bus Error - Set to 1 when there is a misplaced START, STOP bit"]
    #[inline(always)]
    #[must_use]
    pub fn status_ber(&mut self) -> StatusBerW<StatusSpec> {
        StatusBerW::new(self, 4)
    }
    #[doc = "Bit 7 - Read the PIN bit of control register"]
    #[inline(always)]
    #[must_use]
    pub fn status_sts(&mut self) -> StatusStsW<StatusSpec> {
        StatusStsW::new(self, 7)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u8 = 0;
}
