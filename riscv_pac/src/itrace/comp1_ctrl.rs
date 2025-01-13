#[doc = "Register `COMP1_CTRL` reader"]
pub type R = crate::R<Comp1CtrlSpec>;
#[doc = "Register `COMP1_CTRL` writer"]
pub type W = crate::W<Comp1CtrlSpec>;
#[doc = "Field `PINPUT_MODE` reader - Primary comparator input mode. 0: iaddr, 1: context, 2: tval, 3: daddr."]
pub type PinputModeR = crate::FieldReader;
#[doc = "Field `PINPUT_MODE` writer - Primary comparator input mode. 0: iaddr, 1: context, 2: tval, 3: daddr."]
pub type PinputModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SINPUT_MODE` reader - Secondary comparator input mode. 0: iaddr, 1: context, 2: tval, 3: daddr."]
pub type SinputModeR = crate::FieldReader;
#[doc = "Field `SINPUT_MODE` writer - Secondary comparator input mode. 0: iaddr, 1: context, 2: tval, 3: daddr."]
pub type SinputModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PINPUT_COMP_MODE` reader - Mentions the comparison to be done for primary input. 0-equality, 1-not equal, 2-less than, 3-less than equal to, 4-greater than, 5-greater than equal to, 6-False, 7-True"]
pub type PinputCompModeR = crate::FieldReader;
#[doc = "Field `PINPUT_COMP_MODE` writer - Mentions the comparison to be done for primary input. 0-equality, 1-not equal, 2-less than, 3-less than equal to, 4-greater than, 5-greater than equal to, 6-False, 7-True"]
pub type PinputCompModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SINPUT_COMP_MODE` reader - Mentions the comparison to be done for secondary input. 0-equality, 1-not equal, 2-less than, 3-less than equal to, 4-greater than, 5-greater than equal to, 6-False, 7-True"]
pub type SinputCompModeR = crate::FieldReader;
#[doc = "Field `SINPUT_COMP_MODE` writer - Mentions the comparison to be done for secondary input. 0-equality, 1-not equal, 2-less than, 3-less than equal to, 4-greater than, 5-greater than equal to, 6-False, 7-True"]
pub type SinputCompModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PS_COMP_MODE` reader - The comparison to be performed between primary and secondary input."]
pub type PsCompModeR = crate::FieldReader;
#[doc = "Field `PS_COMP_MODE` writer - The comparison to be performed between primary and secondary input."]
pub type PsCompModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PNOTIFY` reader - Sends a support packet when primary comparator matches"]
pub type PnotifyR = crate::BitReader;
#[doc = "Field `PNOTIFY` writer - Sends a support packet when primary comparator matches"]
pub type PnotifyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNOTIFY` reader - Sends a support packet when secondary comparator matches"]
pub type SnotifyR = crate::BitReader;
#[doc = "Field `SNOTIFY` writer - Sends a support packet when secondary comparator matches"]
pub type SnotifyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Primary comparator input mode. 0: iaddr, 1: context, 2: tval, 3: daddr."]
    #[inline(always)]
    pub fn pinput_mode(&self) -> PinputModeR {
        PinputModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Secondary comparator input mode. 0: iaddr, 1: context, 2: tval, 3: daddr."]
    #[inline(always)]
    pub fn sinput_mode(&self) -> SinputModeR {
        SinputModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Mentions the comparison to be done for primary input. 0-equality, 1-not equal, 2-less than, 3-less than equal to, 4-greater than, 5-greater than equal to, 6-False, 7-True"]
    #[inline(always)]
    pub fn pinput_comp_mode(&self) -> PinputCompModeR {
        PinputCompModeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Mentions the comparison to be done for secondary input. 0-equality, 1-not equal, 2-less than, 3-less than equal to, 4-greater than, 5-greater than equal to, 6-False, 7-True"]
    #[inline(always)]
    pub fn sinput_comp_mode(&self) -> SinputCompModeR {
        SinputCompModeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - The comparison to be performed between primary and secondary input."]
    #[inline(always)]
    pub fn ps_comp_mode(&self) -> PsCompModeR {
        PsCompModeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Sends a support packet when primary comparator matches"]
    #[inline(always)]
    pub fn pnotify(&self) -> PnotifyR {
        PnotifyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Sends a support packet when secondary comparator matches"]
    #[inline(always)]
    pub fn snotify(&self) -> SnotifyR {
        SnotifyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Primary comparator input mode. 0: iaddr, 1: context, 2: tval, 3: daddr."]
    #[inline(always)]
    #[must_use]
    pub fn pinput_mode(&mut self) -> PinputModeW<Comp1CtrlSpec> {
        PinputModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Secondary comparator input mode. 0: iaddr, 1: context, 2: tval, 3: daddr."]
    #[inline(always)]
    #[must_use]
    pub fn sinput_mode(&mut self) -> SinputModeW<Comp1CtrlSpec> {
        SinputModeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Mentions the comparison to be done for primary input. 0-equality, 1-not equal, 2-less than, 3-less than equal to, 4-greater than, 5-greater than equal to, 6-False, 7-True"]
    #[inline(always)]
    #[must_use]
    pub fn pinput_comp_mode(&mut self) -> PinputCompModeW<Comp1CtrlSpec> {
        PinputCompModeW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Mentions the comparison to be done for secondary input. 0-equality, 1-not equal, 2-less than, 3-less than equal to, 4-greater than, 5-greater than equal to, 6-False, 7-True"]
    #[inline(always)]
    #[must_use]
    pub fn sinput_comp_mode(&mut self) -> SinputCompModeW<Comp1CtrlSpec> {
        SinputCompModeW::new(self, 8)
    }
    #[doc = "Bits 12:13 - The comparison to be performed between primary and secondary input."]
    #[inline(always)]
    #[must_use]
    pub fn ps_comp_mode(&mut self) -> PsCompModeW<Comp1CtrlSpec> {
        PsCompModeW::new(self, 12)
    }
    #[doc = "Bit 14 - Sends a support packet when primary comparator matches"]
    #[inline(always)]
    #[must_use]
    pub fn pnotify(&mut self) -> PnotifyW<Comp1CtrlSpec> {
        PnotifyW::new(self, 14)
    }
    #[doc = "Bit 15 - Sends a support packet when secondary comparator matches"]
    #[inline(always)]
    #[must_use]
    pub fn snotify(&mut self) -> SnotifyW<Comp1CtrlSpec> {
        SnotifyW::new(self, 15)
    }
}
#[doc = "Control register for 1st comparator\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp1CtrlSpec;
impl crate::RegisterSpec for Comp1CtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`comp1_ctrl::R`](R) reader structure"]
impl crate::Readable for Comp1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`comp1_ctrl::W`](W) writer structure"]
impl crate::Writable for Comp1CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets COMP1_CTRL to value 0"]
impl crate::Resettable for Comp1CtrlSpec {
    const RESET_VALUE: u16 = 0;
}
