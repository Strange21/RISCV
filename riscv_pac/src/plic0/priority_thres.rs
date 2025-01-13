#[doc = "Register `PRIORITY_THRES` reader"]
pub type R = crate::R<PriorityThresSpec>;
#[doc = "Register `PRIORITY_THRES` writer"]
pub type W = crate::W<PriorityThresSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Priority threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`priority_thres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority_thres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PriorityThresSpec;
impl crate::RegisterSpec for PriorityThresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority_thres::R`](R) reader structure"]
impl crate::Readable for PriorityThresSpec {}
#[doc = "`write(|w| ..)` method takes [`priority_thres::W`](W) writer structure"]
impl crate::Writable for PriorityThresSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIORITY_THRES to value 0"]
impl crate::Resettable for PriorityThresSpec {
    const RESET_VALUE: u32 = 0;
}
