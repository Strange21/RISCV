#[doc = "Register `CLOCK_CTRL` reader"]
pub type R = crate::R<ClockCtrlSpec>;
#[doc = "Register `CLOCK_CTRL` writer"]
pub type W = crate::W<ClockCtrlSpec>;
#[doc = "Field `CLK_SRC` reader - PWM clock select bit"]
pub type ClkSrcR = crate::BitReader;
#[doc = "Field `CLK_SRC` writer - PWM clock select bit"]
pub type ClkSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PRESCALAR` reader - PWM prescalar value"]
pub type ClkPrescalarR = crate::FieldReader<u16>;
#[doc = "Field `CLK_PRESCALAR` writer - PWM prescalar value"]
pub type ClkPrescalarW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - PWM clock select bit"]
    #[inline(always)]
    pub fn clk_src(&self) -> ClkSrcR {
        ClkSrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15 - PWM prescalar value"]
    #[inline(always)]
    pub fn clk_prescalar(&self) -> ClkPrescalarR {
        ClkPrescalarR::new((self.bits >> 1) & 0x7fff)
    }
}
impl W {
    #[doc = "Bit 0 - PWM clock select bit"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src(&mut self) -> ClkSrcW<ClockCtrlSpec> {
        ClkSrcW::new(self, 0)
    }
    #[doc = "Bits 1:15 - PWM prescalar value"]
    #[inline(always)]
    #[must_use]
    pub fn clk_prescalar(&mut self) -> ClkPrescalarW<ClockCtrlSpec> {
        ClkPrescalarW::new(self, 1)
    }
}
#[doc = "Clock prescalar register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockCtrlSpec;
impl crate::RegisterSpec for ClockCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clock_ctrl::R`](R) reader structure"]
impl crate::Readable for ClockCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clock_ctrl::W`](W) writer structure"]
impl crate::Writable for ClockCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CLOCK_CTRL to value 0"]
impl crate::Resettable for ClockCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
