#[doc = "Register `DELAY_REG` reader"]
pub type R = crate::R<DelayRegSpec>;
#[doc = "Register `DELAY_REG` writer"]
pub type W = crate::W<DelayRegSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Stores the delay to have before Tranmission\n\nYou can [`read`](crate::Reg::read) this register and get [`delay_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DelayRegSpec;
impl crate::RegisterSpec for DelayRegSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`delay_reg::R`](R) reader structure"]
impl crate::Readable for DelayRegSpec {}
#[doc = "`write(|w| ..)` method takes [`delay_reg::W`](W) writer structure"]
impl crate::Writable for DelayRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DELAY_REG to value 0"]
impl crate::Resettable for DelayRegSpec {
    const RESET_VALUE: u16 = 0;
}
