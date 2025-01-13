#[doc = "Register `RSA_RSqrMODN` writer"]
pub type W = crate::W<RsaRsqrModnSpec>;
impl core::fmt::Debug for crate::generic::Reg<RsaRsqrModnSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "RSA (R^2) % N register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_rsqr_modn::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsaRsqrModnSpec;
impl crate::RegisterSpec for RsaRsqrModnSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`rsa_rsqr_modn::W`](W) writer structure"]
impl crate::Writable for RsaRsqrModnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets RSA_RSqrMODN to value 0"]
impl crate::Resettable for RsaRsqrModnSpec {
    const RESET_VALUE: u64 = 0;
}
