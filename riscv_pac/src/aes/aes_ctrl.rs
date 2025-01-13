#[doc = "Register `AES_CTRL` reader"]
pub type R = crate::R<AesCtrlSpec>;
#[doc = "Register `AES_CTRL` writer"]
pub type W = crate::W<AesCtrlSpec>;
#[doc = "Field `AES_CTRL_ENCDEC` reader - 0 - Encrypt, 1 - Decrypt"]
pub type AesCtrlEncdecR = crate::BitReader;
#[doc = "Field `AES_CTRL_ENCDEC` writer - 0 - Encrypt, 1 - Decrypt"]
pub type AesCtrlEncdecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_CTRL_KEYLEN` reader - To select the lenght of key. 0 - 128 bits, 1 - 192 bits, 2 - 256 bits"]
pub type AesCtrlKeylenR = crate::FieldReader;
#[doc = "Field `AES_CTRL_KEYLEN` writer - To select the lenght of key. 0 - 128 bits, 1 - 192 bits, 2 - 256 bits"]
pub type AesCtrlKeylenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AES_CTRL_MODE` reader - 0-ECB, 1-CBC, 2-CFB, 3-OFB, 4-CTR"]
pub type AesCtrlModeR = crate::FieldReader;
#[doc = "Field `AES_CTRL_MODE` writer - 0-ECB, 1-CBC, 2-CFB, 3-OFB, 4-CTR"]
pub type AesCtrlModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AES_CTRL_END` reader - To specify end of message"]
pub type AesCtrlEndR = crate::BitReader;
#[doc = "Field `AES_CTRL_END` writer - To specify end of message"]
pub type AesCtrlEndW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0 - Encrypt, 1 - Decrypt"]
    #[inline(always)]
    pub fn aes_ctrl_encdec(&self) -> AesCtrlEncdecR {
        AesCtrlEncdecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - To select the lenght of key. 0 - 128 bits, 1 - 192 bits, 2 - 256 bits"]
    #[inline(always)]
    pub fn aes_ctrl_keylen(&self) -> AesCtrlKeylenR {
        AesCtrlKeylenR::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 3:5 - 0-ECB, 1-CBC, 2-CFB, 3-OFB, 4-CTR"]
    #[inline(always)]
    pub fn aes_ctrl_mode(&self) -> AesCtrlModeR {
        AesCtrlModeR::new((self.bits >> 3) & 7)
    }
    #[doc = "Bit 6 - To specify end of message"]
    #[inline(always)]
    pub fn aes_ctrl_end(&self) -> AesCtrlEndR {
        AesCtrlEndR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0 - Encrypt, 1 - Decrypt"]
    #[inline(always)]
    #[must_use]
    pub fn aes_ctrl_encdec(&mut self) -> AesCtrlEncdecW<AesCtrlSpec> {
        AesCtrlEncdecW::new(self, 0)
    }
    #[doc = "Bits 1:2 - To select the lenght of key. 0 - 128 bits, 1 - 192 bits, 2 - 256 bits"]
    #[inline(always)]
    #[must_use]
    pub fn aes_ctrl_keylen(&mut self) -> AesCtrlKeylenW<AesCtrlSpec> {
        AesCtrlKeylenW::new(self, 1)
    }
    #[doc = "Bits 3:5 - 0-ECB, 1-CBC, 2-CFB, 3-OFB, 4-CTR"]
    #[inline(always)]
    #[must_use]
    pub fn aes_ctrl_mode(&mut self) -> AesCtrlModeW<AesCtrlSpec> {
        AesCtrlModeW::new(self, 3)
    }
    #[doc = "Bit 6 - To specify end of message"]
    #[inline(always)]
    #[must_use]
    pub fn aes_ctrl_end(&mut self) -> AesCtrlEndW<AesCtrlSpec> {
        AesCtrlEndW::new(self, 6)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCtrlSpec;
impl crate::RegisterSpec for AesCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`aes_ctrl::R`](R) reader structure"]
impl crate::Readable for AesCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_ctrl::W`](W) writer structure"]
impl crate::Writable for AesCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AES_CTRL to value 0"]
impl crate::Resettable for AesCtrlSpec {
    const RESET_VALUE: u8 = 0;
}
