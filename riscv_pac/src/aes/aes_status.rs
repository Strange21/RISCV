#[doc = "Register `AES_STATUS` reader"]
pub type R = crate::R<AesStatusSpec>;
#[doc = "Register `AES_STATUS` writer"]
pub type W = crate::W<AesStatusSpec>;
#[doc = "Field `AES_STATUS_OUTP_READ` reader - Once the output is ready and read, this bit becomes 0."]
pub type AesStatusOutpReadR = crate::BitReader;
#[doc = "Field `AES_STATUS_OUTP_READ` writer - Once the output is ready and read, this bit becomes 0."]
pub type AesStatusOutpReadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_STATUS_OUTP_READY` reader - Becomes 1 when the accelerator has computed the output. Becomes 0 once new input is given."]
pub type AesStatusOutpReadyR = crate::BitReader;
#[doc = "Field `AES_STATUS_OUTP_READY` writer - Becomes 1 when the accelerator has computed the output. Becomes 0 once new input is given."]
pub type AesStatusOutpReadyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Once the output is ready and read, this bit becomes 0."]
    #[inline(always)]
    pub fn aes_status_outp_read(&self) -> AesStatusOutpReadR {
        AesStatusOutpReadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Becomes 1 when the accelerator has computed the output. Becomes 0 once new input is given."]
    #[inline(always)]
    pub fn aes_status_outp_ready(&self) -> AesStatusOutpReadyR {
        AesStatusOutpReadyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Once the output is ready and read, this bit becomes 0."]
    #[inline(always)]
    #[must_use]
    pub fn aes_status_outp_read(&mut self) -> AesStatusOutpReadW<AesStatusSpec> {
        AesStatusOutpReadW::new(self, 0)
    }
    #[doc = "Bit 1 - Becomes 1 when the accelerator has computed the output. Becomes 0 once new input is given."]
    #[inline(always)]
    #[must_use]
    pub fn aes_status_outp_ready(&mut self) -> AesStatusOutpReadyW<AesStatusSpec> {
        AesStatusOutpReadyW::new(self, 1)
    }
}
#[doc = "To check the status.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesStatusSpec;
impl crate::RegisterSpec for AesStatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aes_status::R`](R) reader structure"]
impl crate::Readable for AesStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_status::W`](W) writer structure"]
impl crate::Writable for AesStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AES_STATUS to value 0"]
impl crate::Resettable for AesStatusSpec {
    const RESET_VALUE: u8 = 0;
}
