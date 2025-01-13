#[doc = "Register `RPTD_COUNT` reader"]
pub type R = crate::R<RptdCountSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Repeated count register\n\nYou can [`read`](crate::Reg::read) this register and get [`rptd_count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RptdCountSpec;
impl crate::RegisterSpec for RptdCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rptd_count::R`](R) reader structure"]
impl crate::Readable for RptdCountSpec {}
#[doc = "`reset()` method sets RPTD_COUNT to value 0"]
impl crate::Resettable for RptdCountSpec {
    const RESET_VALUE: u32 = 0;
}
