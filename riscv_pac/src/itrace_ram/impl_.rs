#[doc = "Register `IMPL` reader"]
pub type R = crate::R<ImplSpec>;
#[doc = "Field `VER_MAJOR` reader - Major version number"]
pub type VerMajorR = crate::BitReader;
#[doc = "Field `VER_MINOR` reader - Minor version number"]
pub type VerMinorR = crate::BitReader;
#[doc = "Field `COMP_TYPE` reader - Trace RAM sink component type"]
pub type CompTypeR = crate::BitReader;
#[doc = "Field `HAS_SRAM` reader - Indicates whether SRAM is present"]
pub type HasSramR = crate::BitReader;
#[doc = "Field `HAS_SMEM` reader - Indicates whether SREM is present"]
pub type HasSmemR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Major version number"]
    #[inline(always)]
    pub fn ver_major(&self) -> VerMajorR {
        VerMajorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Minor version number"]
    #[inline(always)]
    pub fn ver_minor(&self) -> VerMinorR {
        VerMinorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trace RAM sink component type"]
    #[inline(always)]
    pub fn comp_type(&self) -> CompTypeR {
        CompTypeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates whether SRAM is present"]
    #[inline(always)]
    pub fn has_sram(&self) -> HasSramR {
        HasSramR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates whether SREM is present"]
    #[inline(always)]
    pub fn has_smem(&self) -> HasSmemR {
        HasSmemR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Implementation details register\n\nYou can [`read`](crate::Reg::read) this register and get [`impl_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImplSpec;
impl crate::RegisterSpec for ImplSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`impl_::R`](R) reader structure"]
impl crate::Readable for ImplSpec {}
#[doc = "`reset()` method sets IMPL to value 0"]
impl crate::Resettable for ImplSpec {
    const RESET_VALUE: u32 = 0;
}
