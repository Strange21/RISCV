#[doc = "Register `CLK_CTRL` reader"]
pub type R = crate::R<ClkCtrlSpec>;
#[doc = "Register `CLK_CTRL` writer"]
pub type W = crate::W<ClkCtrlSpec>;
#[doc = "Field `CLK_POLARITY` reader - holds the clock polarity"]
pub type ClkPolarityR = crate::BitReader;
#[doc = "Field `CLK_POLARITY` writer - holds the clock polarity"]
pub type ClkPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PHASE` reader - holds the clock phase"]
pub type ClkPhaseR = crate::BitReader;
#[doc = "Field `CLK_PHASE` writer - holds the clock phase"]
pub type ClkPhaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PRESCALAR` reader - holds the prescaller value of the sclk"]
pub type ClkPrescalarR = crate::FieldReader;
#[doc = "Field `CLK_PRESCALAR` writer - holds the prescaller value of the sclk"]
pub type ClkPrescalarW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SETUP_DELAY` reader - holds the setup delay"]
pub type SetupDelayR = crate::FieldReader;
#[doc = "Field `SETUP_DELAY` writer - holds the setup delay"]
pub type SetupDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOLD_DELAY` reader - holds the hold delay"]
pub type HoldDelayR = crate::FieldReader;
#[doc = "Field `HOLD_DELAY` writer - holds the hold delay"]
pub type HoldDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - holds the clock polarity"]
    #[inline(always)]
    pub fn clk_polarity(&self) -> ClkPolarityR {
        ClkPolarityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - holds the clock phase"]
    #[inline(always)]
    pub fn clk_phase(&self) -> ClkPhaseR {
        ClkPhaseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - holds the prescaller value of the sclk"]
    #[inline(always)]
    pub fn clk_prescalar(&self) -> ClkPrescalarR {
        ClkPrescalarR::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 10:17 - holds the setup delay"]
    #[inline(always)]
    pub fn setup_delay(&self) -> SetupDelayR {
        SetupDelayR::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - holds the hold delay"]
    #[inline(always)]
    pub fn hold_delay(&self) -> HoldDelayR {
        HoldDelayR::new(((self.bits >> 18) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - holds the clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clk_polarity(&mut self) -> ClkPolarityW<ClkCtrlSpec> {
        ClkPolarityW::new(self, 0)
    }
    #[doc = "Bit 1 - holds the clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn clk_phase(&mut self) -> ClkPhaseW<ClkCtrlSpec> {
        ClkPhaseW::new(self, 1)
    }
    #[doc = "Bits 2:9 - holds the prescaller value of the sclk"]
    #[inline(always)]
    #[must_use]
    pub fn clk_prescalar(&mut self) -> ClkPrescalarW<ClkCtrlSpec> {
        ClkPrescalarW::new(self, 2)
    }
    #[doc = "Bits 10:17 - holds the setup delay"]
    #[inline(always)]
    #[must_use]
    pub fn setup_delay(&mut self) -> SetupDelayW<ClkCtrlSpec> {
        SetupDelayW::new(self, 10)
    }
    #[doc = "Bits 18:25 - holds the hold delay"]
    #[inline(always)]
    #[must_use]
    pub fn hold_delay(&mut self) -> HoldDelayW<ClkCtrlSpec> {
        HoldDelayW::new(self, 18)
    }
}
#[doc = "SPI clock generation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkCtrlSpec;
impl crate::RegisterSpec for ClkCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ctrl::R`](R) reader structure"]
impl crate::Readable for ClkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_ctrl::W`](W) writer structure"]
impl crate::Writable for ClkCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_CTRL to value 0"]
impl crate::Resettable for ClkCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
