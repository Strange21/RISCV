#[doc = "Register `WDT_CTRL` reader"]
pub type R = crate::R<WdtCtrlSpec>;
#[doc = "Register `WDT_CTRL` writer"]
pub type W = crate::W<WdtCtrlSpec>;
#[doc = "Field `WDT_CTRL_EN` reader - Enable the watchdog timer"]
pub type WdtCtrlEnR = crate::BitReader;
#[doc = "Field `WDT_CTRL_EN` writer - Enable the watchdog timer"]
pub type WdtCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_CTRL_MODE` reader - Mode of watchdog timer. 0 - Interrupt, 1 - Reset"]
pub type WdtCtrlModeR = crate::BitReader;
#[doc = "Field `WDT_CTRL_MODE` writer - Mode of watchdog timer. 0 - Interrupt, 1 - Reset"]
pub type WdtCtrlModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_CTRL_SOFT` reader - Software reset"]
pub type WdtCtrlSoftR = crate::BitReader;
#[doc = "Field `WDT_CTRL_SOFT` writer - Software reset"]
pub type WdtCtrlSoftW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the watchdog timer"]
    #[inline(always)]
    pub fn wdt_ctrl_en(&self) -> WdtCtrlEnR {
        WdtCtrlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode of watchdog timer. 0 - Interrupt, 1 - Reset"]
    #[inline(always)]
    pub fn wdt_ctrl_mode(&self) -> WdtCtrlModeR {
        WdtCtrlModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software reset"]
    #[inline(always)]
    pub fn wdt_ctrl_soft(&self) -> WdtCtrlSoftR {
        WdtCtrlSoftR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the watchdog timer"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctrl_en(&mut self) -> WdtCtrlEnW<WdtCtrlSpec> {
        WdtCtrlEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Mode of watchdog timer. 0 - Interrupt, 1 - Reset"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctrl_mode(&mut self) -> WdtCtrlModeW<WdtCtrlSpec> {
        WdtCtrlModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_ctrl_soft(&mut self) -> WdtCtrlSoftW<WdtCtrlSpec> {
        WdtCtrlSoftW::new(self, 2)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtCtrlSpec;
impl crate::RegisterSpec for WdtCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wdt_ctrl::R`](R) reader structure"]
impl crate::Readable for WdtCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt_ctrl::W`](W) writer structure"]
impl crate::Writable for WdtCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WDT_CTRL to value 0"]
impl crate::Resettable for WdtCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
