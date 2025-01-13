#[doc = "Register `COMP1_PMATCH_HIGH` reader"]
pub type R = crate::R<Comp1PmatchHighSpec>;
#[doc = "Register `COMP1_PMATCH_HIGH` writer"]
pub type W = crate::W<Comp1PmatchHighSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Comparator 1 primary match data: High 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_pmatch_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_pmatch_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp1PmatchHighSpec;
impl crate::RegisterSpec for Comp1PmatchHighSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1_pmatch_high::R`](R) reader structure"]
impl crate::Readable for Comp1PmatchHighSpec {}
#[doc = "`write(|w| ..)` method takes [`comp1_pmatch_high::W`](W) writer structure"]
impl crate::Writable for Comp1PmatchHighSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMP1_PMATCH_HIGH to value 0"]
impl crate::Resettable for Comp1PmatchHighSpec {
    const RESET_VALUE: u32 = 0;
}
