#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CTRL_ACK` reader - Sends the acknoledge bit"]
pub type CtrlAckR = crate::BitReader;
#[doc = "Field `CTRL_ACK` writer - Sends the acknoledge bit"]
pub type CtrlAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_STO` reader - Sends the stop bit"]
pub type CtrlStoR = crate::BitReader;
#[doc = "Field `CTRL_STO` writer - Sends the stop bit"]
pub type CtrlStoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_STA` reader - Sends the start bit"]
pub type CtrlStaR = crate::BitReader;
#[doc = "Field `CTRL_STA` writer - Sends the start bit"]
pub type CtrlStaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_ENI` reader - Enables the external interrupt output"]
pub type CtrlEniR = crate::BitReader;
#[doc = "Field `CTRL_ENI` writer - Enables the external interrupt output"]
pub type CtrlEniW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_ESO` reader - Enable Serial Output. ESO = 0 - Registers can be initialized. ESO = 1 - I2C Serial Transmission"]
pub type CtrlEsoR = crate::BitReader;
#[doc = "Field `CTRL_ESO` writer - Enable Serial Output. ESO = 0 - Registers can be initialized. ESO = 1 - I2C Serial Transmission"]
pub type CtrlEsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_PIN` reader - Used as a software reset. If pin is 1 all status bits are reset.Used as transmission complete status in polled applications"]
pub type CtrlPinR = crate::BitReader;
#[doc = "Field `CTRL_PIN` writer - Used as a software reset. If pin is 1 all status bits are reset.Used as transmission complete status in polled applications"]
pub type CtrlPinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sends the acknoledge bit"]
    #[inline(always)]
    pub fn ctrl_ack(&self) -> CtrlAckR {
        CtrlAckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sends the stop bit"]
    #[inline(always)]
    pub fn ctrl_sto(&self) -> CtrlStoR {
        CtrlStoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sends the start bit"]
    #[inline(always)]
    pub fn ctrl_sta(&self) -> CtrlStaR {
        CtrlStaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the external interrupt output"]
    #[inline(always)]
    pub fn ctrl_eni(&self) -> CtrlEniR {
        CtrlEniR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Serial Output. ESO = 0 - Registers can be initialized. ESO = 1 - I2C Serial Transmission"]
    #[inline(always)]
    pub fn ctrl_eso(&self) -> CtrlEsoR {
        CtrlEsoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Used as a software reset. If pin is 1 all status bits are reset.Used as transmission complete status in polled applications"]
    #[inline(always)]
    pub fn ctrl_pin(&self) -> CtrlPinR {
        CtrlPinR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sends the acknoledge bit"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_ack(&mut self) -> CtrlAckW<CtrlSpec> {
        CtrlAckW::new(self, 0)
    }
    #[doc = "Bit 1 - Sends the stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_sto(&mut self) -> CtrlStoW<CtrlSpec> {
        CtrlStoW::new(self, 1)
    }
    #[doc = "Bit 2 - Sends the start bit"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_sta(&mut self) -> CtrlStaW<CtrlSpec> {
        CtrlStaW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables the external interrupt output"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_eni(&mut self) -> CtrlEniW<CtrlSpec> {
        CtrlEniW::new(self, 3)
    }
    #[doc = "Bit 6 - Enable Serial Output. ESO = 0 - Registers can be initialized. ESO = 1 - I2C Serial Transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_eso(&mut self) -> CtrlEsoW<CtrlSpec> {
        CtrlEsoW::new(self, 6)
    }
    #[doc = "Bit 7 - Used as a software reset. If pin is 1 all status bits are reset.Used as transmission complete status in polled applications"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_pin(&mut self) -> CtrlPinW<CtrlSpec> {
        CtrlPinW::new(self, 7)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u8 = 0;
}
