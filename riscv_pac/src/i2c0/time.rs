#[doc = "Register `TIME` reader"]
pub type R = crate::R<TimeSpec>;
#[doc = "Register `TIME` writer"]
pub type W = crate::W<TimeSpec>;
#[doc = "Field `TIMEOUT` reader - The transmission timeout value"]
pub type TimeoutR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUT` writer - The transmission timeout value"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `TIMEOUT_EN` reader - Enable the transmission timeout calculation and interrupt generation"]
pub type TimeoutEnR = crate::BitReader;
#[doc = "Field `TIMEOUT_EN` writer - Enable the transmission timeout calculation and interrupt generation"]
pub type TimeoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_INTR` reader - Interrupt output for transmission timeout"]
pub type TimeoutIntrR = crate::BitReader;
#[doc = "Field `TIMEOUT_INTR` writer - Interrupt output for transmission timeout"]
pub type TimeoutIntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - The transmission timeout value"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - Enable the transmission timeout calculation and interrupt generation"]
    #[inline(always)]
    pub fn timeout_en(&self) -> TimeoutEnR {
        TimeoutEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt output for transmission timeout"]
    #[inline(always)]
    pub fn timeout_intr(&self) -> TimeoutIntrR {
        TimeoutIntrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - The transmission timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<TimeSpec> {
        TimeoutW::new(self, 0)
    }
    #[doc = "Bit 14 - Enable the transmission timeout calculation and interrupt generation"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_en(&mut self) -> TimeoutEnW<TimeSpec> {
        TimeoutEnW::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt output for transmission timeout"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_intr(&mut self) -> TimeoutIntrW<TimeSpec> {
        TimeoutIntrW::new(self, 15)
    }
}
#[doc = "I2C timeout calculation and interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeSpec;
impl crate::RegisterSpec for TimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time::R`](R) reader structure"]
impl crate::Readable for TimeSpec {}
#[doc = "`write(|w| ..)` method takes [`time::W`](W) writer structure"]
impl crate::Writable for TimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TimeSpec {
    const RESET_VALUE: u32 = 0;
}
