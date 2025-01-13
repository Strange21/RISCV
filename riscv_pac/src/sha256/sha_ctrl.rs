#[doc = "Register `SHA_CTRL` reader"]
pub type R = crate::R<ShaCtrlSpec>;
#[doc = "Register `SHA_CTRL` writer"]
pub type W = crate::W<ShaCtrlSpec>;
#[doc = "Field `CONT_PREHASH` reader - To continue the hash calculated from previous block to next block or not. 1 - continue from previous block. 0 - Initial prehash"]
pub type ContPrehashR = crate::BitReader;
#[doc = "Field `CONT_PREHASH` writer - To continue the hash calculated from previous block to next block or not. 1 - continue from previous block. 0 - Initial prehash"]
pub type ContPrehashW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - To continue the hash calculated from previous block to next block or not. 1 - continue from previous block. 0 - Initial prehash"]
    #[inline(always)]
    pub fn cont_prehash(&self) -> ContPrehashR {
        ContPrehashR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - To continue the hash calculated from previous block to next block or not. 1 - continue from previous block. 0 - Initial prehash"]
    #[inline(always)]
    #[must_use]
    pub fn cont_prehash(&mut self) -> ContPrehashW<ShaCtrlSpec> {
        ContPrehashW::new(self, 0)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaCtrlSpec;
impl crate::RegisterSpec for ShaCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sha_ctrl::R`](R) reader structure"]
impl crate::Readable for ShaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`sha_ctrl::W`](W) writer structure"]
impl crate::Writable for ShaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHA_CTRL to value 0"]
impl crate::Resettable for ShaCtrlSpec {
    const RESET_VALUE: u8 = 0;
}
