#[doc = "Register `WDT_RESET_CYCLES` reader"]
pub type R = crate::R<WdtResetCyclesSpec>;
#[doc = "Register `WDT_RESET_CYCLES` writer"]
pub type W = crate::W<WdtResetCyclesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The number of cycles for which the interrupt needs to be held\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_reset_cycles::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_reset_cycles::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtResetCyclesSpec;
impl crate::RegisterSpec for WdtResetCyclesSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wdt_reset_cycles::R`](R) reader structure"]
impl crate::Readable for WdtResetCyclesSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt_reset_cycles::W`](W) writer structure"]
impl crate::Writable for WdtResetCyclesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WDT_RESET_CYCLES to value 0"]
impl crate::Resettable for WdtResetCyclesSpec {
    const RESET_VALUE: u16 = 0;
}
