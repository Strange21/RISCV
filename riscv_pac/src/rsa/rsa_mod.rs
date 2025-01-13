#[doc = "Register `RSA_MOD` writer"]
pub type W = crate::W<RsaModSpec>;
impl core::fmt::Debug for crate::generic::Reg<RsaModSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "RSA modulus register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_mod::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsaModSpec;
impl crate::RegisterSpec for RsaModSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`rsa_mod::W`](W) writer structure"]
impl crate::Writable for RsaModSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets RSA_MOD to value 0"]
impl crate::Resettable for RsaModSpec {
    const RESET_VALUE: u64 = 0;
}
