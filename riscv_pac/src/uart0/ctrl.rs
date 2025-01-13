#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CTRL_STOP_BITS` reader - To select the number of stop bits. 00 - 1 Stop bits, 01 - 1.5 Stop bits, 10 - 2 Stop bits"]
pub type CtrlStopBitsR = crate::FieldReader;
#[doc = "Field `CTRL_STOP_BITS` writer - To select the number of stop bits. 00 - 1 Stop bits, 01 - 1.5 Stop bits, 10 - 2 Stop bits"]
pub type CtrlStopBitsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTRL_PARITY` reader - To select the type of parity. 00 - None, 01 - Odd, 10 - Even"]
pub type CtrlParityR = crate::FieldReader;
#[doc = "Field `CTRL_PARITY` writer - To select the type of parity. 00 - None, 01 - Odd, 10 - Even"]
pub type CtrlParityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHAR_SIZE` reader - Selects the transmission data size 00 - 8 bits, 01 - 7 bits, 10 - 6 bits, 11 - 5 bits"]
pub type CharSizeR = crate::FieldReader;
#[doc = "Field `CHAR_SIZE` writer - Selects the transmission data size 00 - 8 bits, 01 - 7 bits, 10 - 6 bits, 11 - 5 bits"]
pub type CharSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 1:2 - To select the number of stop bits. 00 - 1 Stop bits, 01 - 1.5 Stop bits, 10 - 2 Stop bits"]
    #[inline(always)]
    pub fn ctrl_stop_bits(&self) -> CtrlStopBitsR {
        CtrlStopBitsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - To select the type of parity. 00 - None, 01 - Odd, 10 - Even"]
    #[inline(always)]
    pub fn ctrl_parity(&self) -> CtrlParityR {
        CtrlParityR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Selects the transmission data size 00 - 8 bits, 01 - 7 bits, 10 - 6 bits, 11 - 5 bits"]
    #[inline(always)]
    pub fn char_size(&self) -> CharSizeR {
        CharSizeR::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - To select the number of stop bits. 00 - 1 Stop bits, 01 - 1.5 Stop bits, 10 - 2 Stop bits"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_stop_bits(&mut self) -> CtrlStopBitsW<CtrlSpec> {
        CtrlStopBitsW::new(self, 1)
    }
    #[doc = "Bits 3:4 - To select the type of parity. 00 - None, 01 - Odd, 10 - Even"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_parity(&mut self) -> CtrlParityW<CtrlSpec> {
        CtrlParityW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Selects the transmission data size 00 - 8 bits, 01 - 7 bits, 10 - 6 bits, 11 - 5 bits"]
    #[inline(always)]
    #[must_use]
    pub fn char_size(&mut self) -> CharSizeW<CtrlSpec> {
        CharSizeW::new(self, 5)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u16 = 0;
}
