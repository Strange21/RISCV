#[doc = "Register `RP_HIGH` reader"]
pub type R = crate::R<RpHighSpec>;
#[doc = "Register `RP_HIGH` writer"]
pub type W = crate::W<RpHighSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Read pointer of trace packet into RAM. High 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`rp_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rp_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RpHighSpec;
impl crate::RegisterSpec for RpHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rp_high::R`](R) reader structure"]
impl crate::Readable for RpHighSpec {}
#[doc = "`write(|w| ..)` method takes [`rp_high::W`](W) writer structure"]
impl crate::Writable for RpHighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RP_HIGH to value 0"]
impl crate::Resettable for RpHighSpec {
    const RESET_VALUE: u32 = 0;
}
