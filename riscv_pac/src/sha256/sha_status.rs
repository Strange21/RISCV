#[doc = "Register `SHA_STATUS` reader"]
pub type R = crate::R<ShaStatusSpec>;
#[doc = "Field `SHA_STATUS_READY` reader - Sha is ready to take another input"]
pub type ShaStatusReadyR = crate::BitReader;
#[doc = "Field `SHA_STATUS_OUT_READY` reader - The output of the SHA is ready"]
pub type ShaStatusOutReadyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Sha is ready to take another input"]
    #[inline(always)]
    pub fn sha_status_ready(&self) -> ShaStatusReadyR {
        ShaStatusReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The output of the SHA is ready"]
    #[inline(always)]
    pub fn sha_status_out_ready(&self) -> ShaStatusOutReadyR {
        ShaStatusOutReadyR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "To read the status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaStatusSpec;
impl crate::RegisterSpec for ShaStatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sha_status::R`](R) reader structure"]
impl crate::Readable for ShaStatusSpec {}
#[doc = "`reset()` method sets SHA_STATUS to value 0"]
impl crate::Resettable for ShaStatusSpec {
    const RESET_VALUE: u8 = 0;
}
