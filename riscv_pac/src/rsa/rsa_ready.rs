#[doc = "Register `RSA_READY` reader"]
pub type R = crate::R<RsaReadySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "RSA is ready to take another input and computation of previous is done\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_ready::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsaReadySpec;
impl crate::RegisterSpec for RsaReadySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsa_ready::R`](R) reader structure"]
impl crate::Readable for RsaReadySpec {}
#[doc = "`reset()` method sets RSA_READY to value 0"]
impl crate::Resettable for RsaReadySpec {
    const RESET_VALUE: u32 = 0;
}
