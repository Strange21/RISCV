#[doc = "Register `PENDING_0_32` reader"]
pub type R = crate::R<Pending0_32Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Interrupt pending bits of sources 0-32\n\nYou can [`read`](crate::Reg::read) this register and get [`pending_0_32::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pending0_32Spec;
impl crate::RegisterSpec for Pending0_32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending_0_32::R`](R) reader structure"]
impl crate::Readable for Pending0_32Spec {}
#[doc = "`reset()` method sets PENDING_0_32 to value 0"]
impl crate::Resettable for Pending0_32Spec {
    const RESET_VALUE: u32 = 0;
}
