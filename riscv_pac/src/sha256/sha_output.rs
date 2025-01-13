#[doc = "Register `SHA_OUTPUT` writer"]
pub type W = crate::W<ShaOutputSpec>;
impl core::fmt::Debug for crate::generic::Reg<ShaOutputSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Output text register. Input is given 64 bits at a time.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_output::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaOutputSpec;
impl crate::RegisterSpec for ShaOutputSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`sha_output::W`](W) writer structure"]
impl crate::Writable for ShaOutputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets SHA_OUTPUT to value 0"]
impl crate::Resettable for ShaOutputSpec {
    const RESET_VALUE: u64 = 0;
}
