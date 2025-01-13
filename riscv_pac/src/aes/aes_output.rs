#[doc = "Register `AES_OUTPUT` reader"]
pub type R = crate::R<AesOutputSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Output register. Output is read 64 bits at a time.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_output::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesOutputSpec;
impl crate::RegisterSpec for AesOutputSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`aes_output::R`](R) reader structure"]
impl crate::Readable for AesOutputSpec {}
#[doc = "`reset()` method sets AES_OUTPUT to value 0"]
impl crate::Resettable for AesOutputSpec {
    const RESET_VALUE: u64 = 0;
}
