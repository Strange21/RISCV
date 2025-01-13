#[doc = "Register `LIMIT_LOW` reader"]
pub type R = crate::R<LimitLowSpec>;
#[doc = "Register `LIMIT_LOW` writer"]
pub type W = crate::W<LimitLowSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The RAM end address register. Low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`limit_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimitLowSpec;
impl crate::RegisterSpec for LimitLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limit_low::R`](R) reader structure"]
impl crate::Readable for LimitLowSpec {}
#[doc = "`write(|w| ..)` method takes [`limit_low::W`](W) writer structure"]
impl crate::Writable for LimitLowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIMIT_LOW to value 0"]
impl crate::Resettable for LimitLowSpec {
    const RESET_VALUE: u32 = 0;
}
