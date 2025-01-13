#[doc = "Register `AES_INPUT` writer"]
pub type W = crate::W<AesInputSpec>;
impl core::fmt::Debug for crate::generic::Reg<AesInputSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Input text register. Input is given 64 bits at a time.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_input::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesInputSpec;
impl crate::RegisterSpec for AesInputSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`aes_input::W`](W) writer structure"]
impl crate::Writable for AesInputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets AES_INPUT to value 0"]
impl crate::Resettable for AesInputSpec {
    const RESET_VALUE: u64 = 0;
}
