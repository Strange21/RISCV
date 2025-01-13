#[doc = "Register `AES_IV` writer"]
pub type W = crate::W<AesIvSpec>;
impl core::fmt::Debug for crate::generic::Reg<AesIvSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Initialization vector register. IV is given 64 bits at a time.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_iv::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesIvSpec;
impl crate::RegisterSpec for AesIvSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`aes_iv::W`](W) writer structure"]
impl crate::Writable for AesIvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets AES_IV to value 0"]
impl crate::Resettable for AesIvSpec {
    const RESET_VALUE: u64 = 0;
}
