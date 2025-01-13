#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CTRL_EN` reader - PWM controller enable flag"]
pub type CtrlEnR = crate::BitReader;
#[doc = "Field `CTRL_EN` writer - PWM controller enable flag"]
pub type CtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_START` reader - PWM signal generation start flag"]
pub type CtrlStartR = crate::BitReader;
#[doc = "Field `CTRL_START` writer - PWM signal generation start flag"]
pub type CtrlStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_OUTPUT_EN` reader - PWM output enable flag"]
pub type CtrlOutputEnR = crate::BitReader;
#[doc = "Field `CTRL_OUTPUT_EN` writer - PWM output enable flag"]
pub type CtrlOutputEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_OUTPUT_POLARITY` reader - PWM output polarity select flag"]
pub type CtrlOutputPolarityR = crate::BitReader;
#[doc = "Field `CTRL_OUTPUT_POLARITY` writer - PWM output polarity select flag"]
pub type CtrlOutputPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_COUNTER_RESET` reader - Resets the counter in PWM"]
pub type CtrlCounterResetR = crate::BitReader;
#[doc = "Field `CTRL_COUNTER_RESET` writer - Resets the counter in PWM"]
pub type CtrlCounterResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_COMP_OUT_ENABLE` reader - PWM Complementary Output"]
pub type CtrlCompOutEnableR = crate::BitReader;
#[doc = "Field `CTRL_COMP_OUT_ENABLE` writer - PWM Complementary Output"]
pub type CtrlCompOutEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_HALFPERIOD_INTR_EN` reader - PWM halfperiod interrupt enable"]
pub type CtrlHalfperiodIntrEnR = crate::BitReader;
#[doc = "Field `CTRL_HALFPERIOD_INTR_EN` writer - PWM halfperiod interrupt enable"]
pub type CtrlHalfperiodIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_FALL_INTR_EN` reader - PWM fall interrupt enable"]
pub type CtrlFallIntrEnR = crate::BitReader;
#[doc = "Field `CTRL_FALL_INTR_EN` writer - PWM fall interrupt enable"]
pub type CtrlFallIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_RISE_INTR_EN` reader - PWM rise interrupt enable"]
pub type CtrlRiseIntrEnR = crate::BitReader;
#[doc = "Field `CTRL_RISE_INTR_EN` writer - PWM rise interrupt enable"]
pub type CtrlRiseIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_HALFPERIOD_INTR` reader - PWM halfperiod interrupt bit"]
pub type CtrlHalfperiodIntrR = crate::BitReader;
#[doc = "Field `CTRL_HALFPERIOD_INTR` writer - PWM halfperiod interrupt bit"]
pub type CtrlHalfperiodIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_FALL_INTR` reader - PWM fall interrupt bit"]
pub type CtrlFallIntrR = crate::BitReader;
#[doc = "Field `CTRL_FALL_INTR` writer - PWM fall interrupt bit"]
pub type CtrlFallIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_RISE_INTR` reader - PWM rise interrupt bit"]
pub type CtrlRiseIntrR = crate::BitReader;
#[doc = "Field `CTRL_RISE_INTR` writer - PWM rise interrupt bit"]
pub type CtrlRiseIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_UPDATE_EN` reader - When this bit is set, the new values of period and duty cycle will be applied"]
pub type CtrlUpdateEnR = crate::BitReader;
#[doc = "Field `CTRL_UPDATE_EN` writer - When this bit is set, the new values of period and duty cycle will be applied"]
pub type CtrlUpdateEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PWM controller enable flag"]
    #[inline(always)]
    pub fn ctrl_en(&self) -> CtrlEnR {
        CtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM signal generation start flag"]
    #[inline(always)]
    pub fn ctrl_start(&self) -> CtrlStartR {
        CtrlStartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM output enable flag"]
    #[inline(always)]
    pub fn ctrl_output_en(&self) -> CtrlOutputEnR {
        CtrlOutputEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM output polarity select flag"]
    #[inline(always)]
    pub fn ctrl_output_polarity(&self) -> CtrlOutputPolarityR {
        CtrlOutputPolarityR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Resets the counter in PWM"]
    #[inline(always)]
    pub fn ctrl_counter_reset(&self) -> CtrlCounterResetR {
        CtrlCounterResetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM Complementary Output"]
    #[inline(always)]
    pub fn ctrl_comp_out_enable(&self) -> CtrlCompOutEnableR {
        CtrlCompOutEnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM halfperiod interrupt enable"]
    #[inline(always)]
    pub fn ctrl_halfperiod_intr_en(&self) -> CtrlHalfperiodIntrEnR {
        CtrlHalfperiodIntrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM fall interrupt enable"]
    #[inline(always)]
    pub fn ctrl_fall_intr_en(&self) -> CtrlFallIntrEnR {
        CtrlFallIntrEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PWM rise interrupt enable"]
    #[inline(always)]
    pub fn ctrl_rise_intr_en(&self) -> CtrlRiseIntrEnR {
        CtrlRiseIntrEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM halfperiod interrupt bit"]
    #[inline(always)]
    pub fn ctrl_halfperiod_intr(&self) -> CtrlHalfperiodIntrR {
        CtrlHalfperiodIntrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM fall interrupt bit"]
    #[inline(always)]
    pub fn ctrl_fall_intr(&self) -> CtrlFallIntrR {
        CtrlFallIntrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM rise interrupt bit"]
    #[inline(always)]
    pub fn ctrl_rise_intr(&self) -> CtrlRiseIntrR {
        CtrlRiseIntrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When this bit is set, the new values of period and duty cycle will be applied"]
    #[inline(always)]
    pub fn ctrl_update_en(&self) -> CtrlUpdateEnR {
        CtrlUpdateEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM controller enable flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_en(&mut self) -> CtrlEnW<CtrlSpec> {
        CtrlEnW::new(self, 0)
    }
    #[doc = "Bit 1 - PWM signal generation start flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_start(&mut self) -> CtrlStartW<CtrlSpec> {
        CtrlStartW::new(self, 1)
    }
    #[doc = "Bit 2 - PWM output enable flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_output_en(&mut self) -> CtrlOutputEnW<CtrlSpec> {
        CtrlOutputEnW::new(self, 2)
    }
    #[doc = "Bit 3 - PWM output polarity select flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_output_polarity(&mut self) -> CtrlOutputPolarityW<CtrlSpec> {
        CtrlOutputPolarityW::new(self, 3)
    }
    #[doc = "Bit 4 - Resets the counter in PWM"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_counter_reset(&mut self) -> CtrlCounterResetW<CtrlSpec> {
        CtrlCounterResetW::new(self, 4)
    }
    #[doc = "Bit 5 - PWM Complementary Output"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_comp_out_enable(&mut self) -> CtrlCompOutEnableW<CtrlSpec> {
        CtrlCompOutEnableW::new(self, 5)
    }
    #[doc = "Bit 6 - PWM halfperiod interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_halfperiod_intr_en(&mut self) -> CtrlHalfperiodIntrEnW<CtrlSpec> {
        CtrlHalfperiodIntrEnW::new(self, 6)
    }
    #[doc = "Bit 7 - PWM fall interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_fall_intr_en(&mut self) -> CtrlFallIntrEnW<CtrlSpec> {
        CtrlFallIntrEnW::new(self, 7)
    }
    #[doc = "Bit 8 - PWM rise interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_rise_intr_en(&mut self) -> CtrlRiseIntrEnW<CtrlSpec> {
        CtrlRiseIntrEnW::new(self, 8)
    }
    #[doc = "Bit 9 - PWM halfperiod interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_halfperiod_intr(&mut self) -> CtrlHalfperiodIntrW<CtrlSpec> {
        CtrlHalfperiodIntrW::new(self, 9)
    }
    #[doc = "Bit 10 - PWM fall interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_fall_intr(&mut self) -> CtrlFallIntrW<CtrlSpec> {
        CtrlFallIntrW::new(self, 10)
    }
    #[doc = "Bit 11 - PWM rise interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_rise_intr(&mut self) -> CtrlRiseIntrW<CtrlSpec> {
        CtrlRiseIntrW::new(self, 11)
    }
    #[doc = "Bit 12 - When this bit is set, the new values of period and duty cycle will be applied"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_update_en(&mut self) -> CtrlUpdateEnW<CtrlSpec> {
        CtrlUpdateEnW::new(self, 12)
    }
}
#[doc = "PWM Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
