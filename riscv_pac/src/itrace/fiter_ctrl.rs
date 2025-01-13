#[doc = "Register `FITER_CTRL` reader"]
pub type R = crate::R<FiterCtrlSpec>;
#[doc = "Register `FITER_CTRL` writer"]
pub type W = crate::W<FiterCtrlSpec>;
#[doc = "Field `EN` reader - Filter enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Filter enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRV_EN` reader - Match privielge mode enable"]
pub type PrvEnR = crate::BitReader;
#[doc = "Field `PRV_EN` writer - Match privielge mode enable"]
pub type PrvEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRV` reader - The privelege mode to filter when enabled"]
pub type PrvR = crate::FieldReader;
#[doc = "Field `PRV` writer - The privelege mode to filter when enabled"]
pub type PrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP1_EN` reader - Enable the first comparator"]
pub type Comp1EnR = crate::BitReader;
#[doc = "Field `COMP1_EN` writer - Enable the first comparator"]
pub type Comp1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_COMP1` reader - Mentions which comparator output to use for this 1st filter"]
pub type MatchComp1R = crate::FieldReader;
#[doc = "Field `MATCH_COMP1` writer - Mentions which comparator output to use for this 1st filter"]
pub type MatchComp1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP2_EN` reader - Enable the second comparator"]
pub type Comp2EnR = crate::BitReader;
#[doc = "Field `COMP2_EN` writer - Enable the second comparator"]
pub type Comp2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_COMP2` reader - Mentions which comparator output to use for this 2nd filter"]
pub type MatchComp2R = crate::FieldReader;
#[doc = "Field `MATCH_COMP2` writer - Mentions which comparator output to use for this 2nd filter"]
pub type MatchComp2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COMP3_EN` reader - Enable the first comparator"]
pub type Comp3EnR = crate::BitReader;
#[doc = "Field `COMP3_EN` writer - Enable the first comparator"]
pub type Comp3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_COMP3` reader - Mentions which comparator output to use for this 3rd filter"]
pub type MatchComp3R = crate::FieldReader;
#[doc = "Field `MATCH_COMP3` writer - Mentions which comparator output to use for this 3rd filter"]
pub type MatchComp3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Filter enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Match privielge mode enable"]
    #[inline(always)]
    pub fn prv_en(&self) -> PrvEnR {
        PrvEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - The privelege mode to filter when enabled"]
    #[inline(always)]
    pub fn prv(&self) -> PrvR {
        PrvR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - Enable the first comparator"]
    #[inline(always)]
    pub fn comp1_en(&self) -> Comp1EnR {
        Comp1EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Mentions which comparator output to use for this 1st filter"]
    #[inline(always)]
    pub fn match_comp1(&self) -> MatchComp1R {
        MatchComp1R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Enable the second comparator"]
    #[inline(always)]
    pub fn comp2_en(&self) -> Comp2EnR {
        Comp2EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Mentions which comparator output to use for this 2nd filter"]
    #[inline(always)]
    pub fn match_comp2(&self) -> MatchComp2R {
        MatchComp2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Enable the first comparator"]
    #[inline(always)]
    pub fn comp3_en(&self) -> Comp3EnR {
        Comp3EnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Mentions which comparator output to use for this 3rd filter"]
    #[inline(always)]
    pub fn match_comp3(&self) -> MatchComp3R {
        MatchComp3R::new(((self.bits >> 13) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<FiterCtrlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Match privielge mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn prv_en(&mut self) -> PrvEnW<FiterCtrlSpec> {
        PrvEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - The privelege mode to filter when enabled"]
    #[inline(always)]
    #[must_use]
    pub fn prv(&mut self) -> PrvW<FiterCtrlSpec> {
        PrvW::new(self, 2)
    }
    #[doc = "Bit 6 - Enable the first comparator"]
    #[inline(always)]
    #[must_use]
    pub fn comp1_en(&mut self) -> Comp1EnW<FiterCtrlSpec> {
        Comp1EnW::new(self, 6)
    }
    #[doc = "Bits 7:8 - Mentions which comparator output to use for this 1st filter"]
    #[inline(always)]
    #[must_use]
    pub fn match_comp1(&mut self) -> MatchComp1W<FiterCtrlSpec> {
        MatchComp1W::new(self, 7)
    }
    #[doc = "Bit 9 - Enable the second comparator"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_en(&mut self) -> Comp2EnW<FiterCtrlSpec> {
        Comp2EnW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Mentions which comparator output to use for this 2nd filter"]
    #[inline(always)]
    #[must_use]
    pub fn match_comp2(&mut self) -> MatchComp2W<FiterCtrlSpec> {
        MatchComp2W::new(self, 10)
    }
    #[doc = "Bit 12 - Enable the first comparator"]
    #[inline(always)]
    #[must_use]
    pub fn comp3_en(&mut self) -> Comp3EnW<FiterCtrlSpec> {
        Comp3EnW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Mentions which comparator output to use for this 3rd filter"]
    #[inline(always)]
    #[must_use]
    pub fn match_comp3(&mut self) -> MatchComp3W<FiterCtrlSpec> {
        MatchComp3W::new(self, 13)
    }
}
#[doc = "Instruction filter control\n\nYou can [`read`](crate::Reg::read) this register and get [`fiter_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiter_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FiterCtrlSpec;
impl crate::RegisterSpec for FiterCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiter_ctrl::R`](R) reader structure"]
impl crate::Readable for FiterCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fiter_ctrl::W`](W) writer structure"]
impl crate::Writable for FiterCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FITER_CTRL to value 0"]
impl crate::Resettable for FiterCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
