#[doc = "Register `PENDING_33_63` reader"]
pub type R = crate::R<Pending33_63Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Interrupt pending bits of sources 33-63\n\nYou can [`read`](crate::Reg::read) this register and get [`pending_33_63::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pending33_63Spec;
impl crate::RegisterSpec for Pending33_63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending_33_63::R`](R) reader structure"]
impl crate::Readable for Pending33_63Spec {}
#[doc = "`reset()` method sets PENDING_33_63 to value 0"]
impl crate::Resettable for Pending33_63Spec {
    const RESET_VALUE: u32 = 0;
}
