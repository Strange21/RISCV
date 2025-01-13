#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ACTIVE` reader - Indicates whether trace is active or not. Used for gating."]
pub type ActiveR = crate::BitReader;
#[doc = "Field `ACTIVE` writer - Indicates whether trace is active or not. Used for gating."]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Enables the trace"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enables the trace"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I_EN` reader - Enables the Instruction trace"]
pub type IEnR = crate::BitReader;
#[doc = "Field `I_EN` writer - Enables the Instruction trace"]
pub type IEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESYNC_MODE` reader - Sets the resync packet mode. 0-Not supported, 1-Trace packet count, 2-Every clock, 3-Instruction retired"]
pub type ResyncModeR = crate::FieldReader;
#[doc = "Field `RESYNC_MODE` writer - Sets the resync packet mode. 0-Not supported, 1-Trace packet count, 2-Every clock, 3-Instruction retired"]
pub type ResyncModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESYNC_MAX` reader - Sets the resync interval time."]
pub type ResyncMaxR = crate::FieldReader;
#[doc = "Field `RESYNC_MAX` writer - Sets the resync interval time."]
pub type ResyncMaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Indicates whether trace is active or not. Used for gating."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the trace"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the Instruction trace"]
    #[inline(always)]
    pub fn i_en(&self) -> IEnR {
        IEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Sets the resync packet mode. 0-Not supported, 1-Trace packet count, 2-Every clock, 3-Instruction retired"]
    #[inline(always)]
    pub fn resync_mode(&self) -> ResyncModeR {
        ResyncModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Sets the resync interval time."]
    #[inline(always)]
    pub fn resync_max(&self) -> ResyncMaxR {
        ResyncMaxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether trace is active or not. Used for gating."]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ActiveW<CtrlSpec> {
        ActiveW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the trace"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the Instruction trace"]
    #[inline(always)]
    #[must_use]
    pub fn i_en(&mut self) -> IEnW<CtrlSpec> {
        IEnW::new(self, 2)
    }
    #[doc = "Bits 8:9 - Sets the resync packet mode. 0-Not supported, 1-Trace packet count, 2-Every clock, 3-Instruction retired"]
    #[inline(always)]
    #[must_use]
    pub fn resync_mode(&mut self) -> ResyncModeW<CtrlSpec> {
        ResyncModeW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Sets the resync interval time."]
    #[inline(always)]
    #[must_use]
    pub fn resync_max(&mut self) -> ResyncMaxW<CtrlSpec> {
        ResyncMaxW::new(self, 12)
    }
}
#[doc = "Itrace control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u16 = 0;
}
