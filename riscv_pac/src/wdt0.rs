#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wdt_cycles: WdtCycles,
    _reserved1: [u8; 0x04],
    wdt_ctrl: WdtCtrl,
    _reserved2: [u8; 0x06],
    wdt_reset_cycles: WdtResetCycles,
    _reserved3: [u8; 0x06],
    wdt_active: WdtActive,
}
impl RegisterBlock {
    #[doc = "0x00 - The number of cycles to count down for reset generation"]
    #[inline(always)]
    pub const fn wdt_cycles(&self) -> &WdtCycles {
        &self.wdt_cycles
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn wdt_ctrl(&self) -> &WdtCtrl {
        &self.wdt_ctrl
    }
    #[doc = "0x10 - The number of cycles for which the interrupt needs to be held"]
    #[inline(always)]
    pub const fn wdt_reset_cycles(&self) -> &WdtResetCycles {
        &self.wdt_reset_cycles
    }
    #[doc = "0x18 - Update the internal WD counter with the WD_CYCLES register"]
    #[inline(always)]
    pub const fn wdt_active(&self) -> &WdtActive {
        &self.wdt_active
    }
}
#[doc = "WDT_CYCLES (rw) register accessor: The number of cycles to count down for reset generation\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_cycles::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_cycles::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_cycles`]
module"]
#[doc(alias = "WDT_CYCLES")]
pub type WdtCycles = crate::Reg<wdt_cycles::WdtCyclesSpec>;
#[doc = "The number of cycles to count down for reset generation"]
pub mod wdt_cycles;
#[doc = "WDT_CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_ctrl`]
module"]
#[doc(alias = "WDT_CTRL")]
pub type WdtCtrl = crate::Reg<wdt_ctrl::WdtCtrlSpec>;
#[doc = "Control register"]
pub mod wdt_ctrl;
#[doc = "WDT_RESET_CYCLES (rw) register accessor: The number of cycles for which the interrupt needs to be held\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_reset_cycles::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_reset_cycles::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_reset_cycles`]
module"]
#[doc(alias = "WDT_RESET_CYCLES")]
pub type WdtResetCycles = crate::Reg<wdt_reset_cycles::WdtResetCyclesSpec>;
#[doc = "The number of cycles for which the interrupt needs to be held"]
pub mod wdt_reset_cycles;
#[doc = "WDT_ACTIVE (rw) register accessor: Update the internal WD counter with the WD_CYCLES register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_active::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_active::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_active`]
module"]
#[doc(alias = "WDT_ACTIVE")]
pub type WdtActive = crate::Reg<wdt_active::WdtActiveSpec>;
#[doc = "Update the internal WD counter with the WD_CYCLES register"]
pub mod wdt_active;
