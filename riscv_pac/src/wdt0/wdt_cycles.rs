#[doc = "Register `WDT_CYCLES` reader"]
pub type R = crate::R<WdtCyclesSpec>;
#[doc = "Register `WDT_CYCLES` writer"]
pub type W = crate::W<WdtCyclesSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "The number of cycles to count down for reset generation\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_cycles::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_cycles::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtCyclesSpec;
impl crate::RegisterSpec for WdtCyclesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_cycles::R`](R) reader structure"]
impl crate::Readable for WdtCyclesSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt_cycles::W`](W) writer structure"]
impl crate::Writable for WdtCyclesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDT_CYCLES to value 0"]
impl crate::Resettable for WdtCyclesSpec {
    const RESET_VALUE: u32 = 0;
}
