#[doc = "Register `LIMIT_HIGH` reader"]
pub type R = crate::R<LimitHighSpec>;
#[doc = "Register `LIMIT_HIGH` writer"]
pub type W = crate::W<LimitHighSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The RAM end address register. High 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`limit_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimitHighSpec;
impl crate::RegisterSpec for LimitHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`limit_high::R`](R) reader structure"]
impl crate::Readable for LimitHighSpec {}
#[doc = "`write(|w| ..)` method takes [`limit_high::W`](W) writer structure"]
impl crate::Writable for LimitHighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LIMIT_HIGH to value 0"]
impl crate::Resettable for LimitHighSpec {
    const RESET_VALUE: u32 = 0;
}
