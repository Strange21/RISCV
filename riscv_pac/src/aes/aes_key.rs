#[doc = "Register `AES_KEY` writer"]
pub type W = crate::W<AesKeySpec>;
impl core::fmt::Debug for crate::generic::Reg<AesKeySpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Key data register. Key is given 64 bits at a time and needs to be written depending on the keylen select\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_key::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesKeySpec;
impl crate::RegisterSpec for AesKeySpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`aes_key::W`](W) writer structure"]
impl crate::Writable for AesKeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets AES_KEY to value 0"]
impl crate::Resettable for AesKeySpec {
    const RESET_VALUE: u64 = 0;
}
