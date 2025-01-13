#[doc = "Register `RSA_OUTPUT` reader"]
pub type R = crate::R<RsaOutputSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "RSA output register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_output::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsaOutputSpec;
impl crate::RegisterSpec for RsaOutputSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`rsa_output::R`](R) reader structure"]
impl crate::Readable for RsaOutputSpec {}
#[doc = "`reset()` method sets RSA_OUTPUT to value 0"]
impl crate::Resettable for RsaOutputSpec {
    const RESET_VALUE: u64 = 0;
}
