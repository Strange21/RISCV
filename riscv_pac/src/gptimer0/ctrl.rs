#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CTRL_EN` reader - Timer enable flag"]
pub type CtrlEnR = crate::BitReader;
#[doc = "Field `CTRL_EN` writer - Timer enable flag"]
pub type CtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_MODE` reader - Timer mode select: 0=PWM, 1=Down, 2=Up, 3=UpDown"]
pub type CtrlModeR = crate::FieldReader;
#[doc = "Field `CTRL_MODE` writer - Timer mode select: 0=PWM, 1=Down, 2=Up, 3=UpDown"]
pub type CtrlModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTRL_OUTPUT_EN` reader - Timer output enable bit"]
pub type CtrlOutputEnR = crate::BitReader;
#[doc = "Field `CTRL_OUTPUT_EN` writer - Timer output enable bit"]
pub type CtrlOutputEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_COUNT_RESET` reader - Timer counter reset bit"]
pub type CtrlCountResetR = crate::BitReader;
#[doc = "Field `CTRL_COUNT_RESET` writer - Timer counter reset bit"]
pub type CtrlCountResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_CNT_COUNT_EN` reader - Timer continuous count enable"]
pub type CtrlCntCountEnR = crate::BitReader;
#[doc = "Field `CTRL_CNT_COUNT_EN` writer - Timer continuous count enable"]
pub type CtrlCntCountEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_PWM_FALL_INTR_EN` reader - PWM fall interupt enable"]
pub type CtrlPwmFallIntrEnR = crate::BitReader;
#[doc = "Field `CTRL_PWM_FALL_INTR_EN` writer - PWM fall interupt enable"]
pub type CtrlPwmFallIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_PWM_RISE_INTR_EN` reader - PWM rise interupt enable"]
pub type CtrlPwmRiseIntrEnR = crate::BitReader;
#[doc = "Field `CTRL_PWM_RISE_INTR_EN` writer - PWM rise interupt enable"]
pub type CtrlPwmRiseIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_OFLOW_INTR_EN` reader - Counter overflow interrupt enable"]
pub type CtrlOflowIntrEnR = crate::BitReader;
#[doc = "Field `CTRL_OFLOW_INTR_EN` writer - Counter overflow interrupt enable"]
pub type CtrlOflowIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_UFLOW_INTR_EN` reader - Counter underflow interrupt enable"]
pub type CtrlUflowIntrEnR = crate::BitReader;
#[doc = "Field `CTRL_UFLOW_INTR_EN` writer - Counter underflow interrupt enable"]
pub type CtrlUflowIntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRL_PWM_FALL_INTR` reader - PWM fall interupt bit"]
pub type CtrlPwmFallIntrR = crate::BitReader;
#[doc = "Field `CTRL_PWM_RISE_INTR` reader - PWM rise interupt bit"]
pub type CtrlPwmRiseIntrR = crate::BitReader;
#[doc = "Field `CTRL_OFLOW_INTR` reader - Counter overflow interrupt bit"]
pub type CtrlOflowIntrR = crate::BitReader;
#[doc = "Field `CTRL_UFLOW_INTR` reader - Counter underflow interrupt bit"]
pub type CtrlUflowIntrR = crate::BitReader;
#[doc = "Field `CTRL_CAPTURE_INP_EN` reader - Counter capture input enable"]
pub type CtrlCaptureInpEnR = crate::BitReader;
#[doc = "Field `CTRL_CAPTURE_INP_EN` writer - Counter capture input enable"]
pub type CtrlCaptureInpEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer enable flag"]
    #[inline(always)]
    pub fn ctrl_en(&self) -> CtrlEnR {
        CtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Timer mode select: 0=PWM, 1=Down, 2=Up, 3=UpDown"]
    #[inline(always)]
    pub fn ctrl_mode(&self) -> CtrlModeR {
        CtrlModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Timer output enable bit"]
    #[inline(always)]
    pub fn ctrl_output_en(&self) -> CtrlOutputEnR {
        CtrlOutputEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer counter reset bit"]
    #[inline(always)]
    pub fn ctrl_count_reset(&self) -> CtrlCountResetR {
        CtrlCountResetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer continuous count enable"]
    #[inline(always)]
    pub fn ctrl_cnt_count_en(&self) -> CtrlCntCountEnR {
        CtrlCntCountEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM fall interupt enable"]
    #[inline(always)]
    pub fn ctrl_pwm_fall_intr_en(&self) -> CtrlPwmFallIntrEnR {
        CtrlPwmFallIntrEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PWM rise interupt enable"]
    #[inline(always)]
    pub fn ctrl_pwm_rise_intr_en(&self) -> CtrlPwmRiseIntrEnR {
        CtrlPwmRiseIntrEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Counter overflow interrupt enable"]
    #[inline(always)]
    pub fn ctrl_oflow_intr_en(&self) -> CtrlOflowIntrEnR {
        CtrlOflowIntrEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Counter underflow interrupt enable"]
    #[inline(always)]
    pub fn ctrl_uflow_intr_en(&self) -> CtrlUflowIntrEnR {
        CtrlUflowIntrEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM fall interupt bit"]
    #[inline(always)]
    pub fn ctrl_pwm_fall_intr(&self) -> CtrlPwmFallIntrR {
        CtrlPwmFallIntrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM rise interupt bit"]
    #[inline(always)]
    pub fn ctrl_pwm_rise_intr(&self) -> CtrlPwmRiseIntrR {
        CtrlPwmRiseIntrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Counter overflow interrupt bit"]
    #[inline(always)]
    pub fn ctrl_oflow_intr(&self) -> CtrlOflowIntrR {
        CtrlOflowIntrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Counter underflow interrupt bit"]
    #[inline(always)]
    pub fn ctrl_uflow_intr(&self) -> CtrlUflowIntrR {
        CtrlUflowIntrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Counter capture input enable"]
    #[inline(always)]
    pub fn ctrl_capture_inp_en(&self) -> CtrlCaptureInpEnR {
        CtrlCaptureInpEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer enable flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_en(&mut self) -> CtrlEnW<CtrlSpec> {
        CtrlEnW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Timer mode select: 0=PWM, 1=Down, 2=Up, 3=UpDown"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_mode(&mut self) -> CtrlModeW<CtrlSpec> {
        CtrlModeW::new(self, 2)
    }
    #[doc = "Bit 4 - Timer output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_output_en(&mut self) -> CtrlOutputEnW<CtrlSpec> {
        CtrlOutputEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer counter reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_count_reset(&mut self) -> CtrlCountResetW<CtrlSpec> {
        CtrlCountResetW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer continuous count enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_cnt_count_en(&mut self) -> CtrlCntCountEnW<CtrlSpec> {
        CtrlCntCountEnW::new(self, 6)
    }
    #[doc = "Bit 7 - PWM fall interupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_pwm_fall_intr_en(&mut self) -> CtrlPwmFallIntrEnW<CtrlSpec> {
        CtrlPwmFallIntrEnW::new(self, 7)
    }
    #[doc = "Bit 8 - PWM rise interupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_pwm_rise_intr_en(&mut self) -> CtrlPwmRiseIntrEnW<CtrlSpec> {
        CtrlPwmRiseIntrEnW::new(self, 8)
    }
    #[doc = "Bit 9 - Counter overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_oflow_intr_en(&mut self) -> CtrlOflowIntrEnW<CtrlSpec> {
        CtrlOflowIntrEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Counter underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_uflow_intr_en(&mut self) -> CtrlUflowIntrEnW<CtrlSpec> {
        CtrlUflowIntrEnW::new(self, 10)
    }
    #[doc = "Bit 15 - Counter capture input enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_capture_inp_en(&mut self) -> CtrlCaptureInpEnW<CtrlSpec> {
        CtrlCaptureInpEnW::new(self, 15)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
