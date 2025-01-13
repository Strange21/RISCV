#[doc = "Register `RP_LOW` reader"]
pub type R = crate::R<RpLowSpec>;
#[doc = "Register `RP_LOW` writer"]
pub type W = crate::W<RpLowSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Read pointer of trace packet into RAM. Low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`rp_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rp_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RpLowSpec;
impl crate::RegisterSpec for RpLowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rp_low::R`](R) reader structure"]
impl crate::Readable for RpLowSpec {}
#[doc = "`write(|w| ..)` method takes [`rp_low::W`](W) writer structure"]
impl crate::Writable for RpLowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RP_LOW to value 0"]
impl crate::Resettable for RpLowSpec {
    const RESET_VALUE: u32 = 0;
}
