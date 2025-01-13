#[doc = "Register `SHA_INPUT` writer"]
pub type W = crate::W<ShaInputSpec>;
impl core::fmt::Debug for crate::generic::Reg<ShaInputSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Input text register. Input is given 64 bits at a time.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_input::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaInputSpec;
impl crate::RegisterSpec for ShaInputSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`sha_input::W`](W) writer structure"]
impl crate::Writable for ShaInputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets SHA_INPUT to value 0"]
impl crate::Resettable for ShaInputSpec {
    const RESET_VALUE: u64 = 0;
}
