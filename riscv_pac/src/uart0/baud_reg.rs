#[doc = "Register `BAUD_REG` reader"]
pub type R = crate::R<BaudRegSpec>;
#[doc = "Register `BAUD_REG` writer"]
pub type W = crate::W<BaudRegSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Baud register\n\nYou can [`read`](crate::Reg::read) this register and get [`baud_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudRegSpec;
impl crate::RegisterSpec for BaudRegSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`baud_reg::R`](R) reader structure"]
impl crate::Readable for BaudRegSpec {}
#[doc = "`write(|w| ..)` method takes [`baud_reg::W`](W) writer structure"]
impl crate::Writable for BaudRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BAUD_REG to value 0"]
impl crate::Resettable for BaudRegSpec {
    const RESET_VALUE: u16 = 0;
}
