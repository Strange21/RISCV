#[doc = "Register `AES_NEXT_BLK` writer"]
pub type W = crate::W<AesNextBlkSpec>;
impl core::fmt::Debug for crate::generic::Reg<AesNextBlkSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "The input data of the next block of the same message\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_next_blk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesNextBlkSpec;
impl crate::RegisterSpec for AesNextBlkSpec {
    type Ux = u64;
}
#[doc = "`write(|w| ..)` method takes [`aes_next_blk::W`](W) writer structure"]
impl crate::Writable for AesNextBlkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets AES_NEXT_BLK to value 0"]
impl crate::Resettable for AesNextBlkSpec {
    const RESET_VALUE: u64 = 0;
}
