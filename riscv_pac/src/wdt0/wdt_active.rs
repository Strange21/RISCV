#[doc = "Register `WDT_ACTIVE` reader"]
pub type R = crate::R<WdtActiveSpec>;
#[doc = "Register `WDT_ACTIVE` writer"]
pub type W = crate::W<WdtActiveSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Update the internal WD counter with the WD_CYCLES register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_active::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_active::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtActiveSpec;
impl crate::RegisterSpec for WdtActiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_active::R`](R) reader structure"]
impl crate::Readable for WdtActiveSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt_active::W`](W) writer structure"]
impl crate::Writable for WdtActiveSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDT_ACTIVE to value 0"]
impl crate::Resettable for WdtActiveSpec {
    const RESET_VALUE: u32 = 0;
}
