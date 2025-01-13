#[doc = "Register `RSA_EXP` writer"]
pub type W = crate::W<RsaExpSpec>;
impl core::fmt::Debug for crate::generic::Reg<RsaExpSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "RSA exponent register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_exp::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsaExpSpec;
impl crate::RegisterSpec for RsaExpSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`rsa_exp::W`](W) writer structure"]
impl crate::Writable for RsaExpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets RSA_EXP to value 0"]
impl crate::Resettable for RsaExpSpec {
    const RESET_VALUE: u64 = 0;
}
