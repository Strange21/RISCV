#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ACTIVE` reader - Indicates whether RAM is active or not."]
pub type ActiveR = crate::BitReader;
#[doc = "Field `ACTIVE` writer - Indicates whether RAM is active or not."]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Enables the RAM to take trace packets."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enables the RAM to take trace packets."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_ON_WRAP` reader - Stop filling up packets into RAM when full"]
pub type StopOnWrapR = crate::BitReader;
#[doc = "Field `STOP_ON_WRAP` writer - Stop filling up packets into RAM when full"]
pub type StopOnWrapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates whether RAM is active or not."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the RAM to take trace packets."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop filling up packets into RAM when full"]
    #[inline(always)]
    pub fn stop_on_wrap(&self) -> StopOnWrapR {
        StopOnWrapR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates whether RAM is active or not."]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ActiveW<CtrlSpec> {
        ActiveW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the RAM to take trace packets."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CtrlSpec> {
        EnW::new(self, 1)
    }
    #[doc = "Bit 6 - Stop filling up packets into RAM when full"]
    #[inline(always)]
    #[must_use]
    pub fn stop_on_wrap(&mut self) -> StopOnWrapW<CtrlSpec> {
        StopOnWrapW::new(self, 6)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
