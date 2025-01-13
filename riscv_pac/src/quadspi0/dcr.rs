#[doc = "Register `DCR` reader"]
pub type R = crate::R<DcrSpec>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DcrSpec>;
#[doc = "Field `CKMODE` reader - Mode 0 / mode 3"]
pub type CkmodeR = crate::BitReader;
#[doc = "Field `CKMODE` writer - Mode 0 / mode 3"]
pub type CkmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSIZE` reader - FLASH memory size"]
pub type FsizeR = crate::FieldReader;
#[doc = "Field `FSIZE` writer - FLASH memory size"]
pub type FsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    pub fn ckmode(&self) -> CkmodeR {
        CkmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:20 - FLASH memory size"]
    #[inline(always)]
    pub fn fsize(&self) -> FsizeR {
        FsizeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Mode 0 / mode 3"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CkmodeW<DcrSpec> {
        CkmodeW::new(self, 0)
    }
    #[doc = "Bits 16:20 - FLASH memory size"]
    #[inline(always)]
    #[must_use]
    pub fn fsize(&mut self) -> FsizeW<DcrSpec> {
        FsizeW::new(self, 16)
    }
}
#[doc = "Device Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrSpec;
impl crate::RegisterSpec for DcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DcrSpec {
    const RESET_VALUE: u32 = 0;
}
