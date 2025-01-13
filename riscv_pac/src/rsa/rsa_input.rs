#[doc = "Register `RSA_INPUT` writer"]
pub type W = crate::W<RsaInputSpec>;
impl core::fmt::Debug for crate::generic::Reg<RsaInputSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "RSA input register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_input::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RsaInputSpec;
impl crate::RegisterSpec for RsaInputSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`rsa_input::W`](W) writer structure"]
impl crate::Writable for RsaInputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets RSA_INPUT to value 0"]
impl crate::Resettable for RsaInputSpec {
    const RESET_VALUE: u64 = 0;
}
