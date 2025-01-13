#[doc = "Register `START_LOW` reader"]
pub type R = crate::R<StartLowSpec>;
#[doc = "Register `START_LOW` writer"]
pub type W = crate::W<StartLowSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The RAM start address register. Low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`start_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartLowSpec;
impl crate::RegisterSpec for StartLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start_low::R`](R) reader structure"]
impl crate::Readable for StartLowSpec {}
#[doc = "`write(|w| ..)` method takes [`start_low::W`](W) writer structure"]
impl crate::Writable for StartLowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets START_LOW to value 0"]
impl crate::Resettable for StartLowSpec {
    const RESET_VALUE: u32 = 0;
}
