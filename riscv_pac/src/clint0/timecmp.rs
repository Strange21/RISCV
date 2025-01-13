#[doc = "Register `TIMECMP` reader"]
pub type R = crate::R<TimecmpSpec>;
#[doc = "Register `TIMECMP` writer"]
pub type W = crate::W<TimecmpSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Time compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`timecmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimecmpSpec;
impl crate::RegisterSpec for TimecmpSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`timecmp::R`](R) reader structure"]
impl crate::Readable for TimecmpSpec {}
#[doc = "`write(|w| ..)` method takes [`timecmp::W`](W) writer structure"]
impl crate::Writable for TimecmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets TIMECMP to value 0"]
impl crate::Resettable for TimecmpSpec {
    const RESET_VALUE: u64 = 0;
}
