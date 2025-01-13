#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x06],
    fiter_ctrl: FiterCtrl,
    comp1_ctrl: Comp1Ctrl,
    _reserved3: [u8; 0x02],
    comp2_ctrl: Comp2Ctrl,
    _reserved4: [u8; 0x02],
    comp3_ctrl: Comp3Ctrl,
    _reserved5: [u8; 0x02],
    comp1_pmatch_low: Comp1PmatchLow,
    comp1_pmatch_high: Comp1PmatchHigh,
    comp1_smatch_low: Comp1SmatchLow,
}
impl RegisterBlock {
    #[doc = "0x00 - Itrace control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - Instruction filter control"]
    #[inline(always)]
    pub const fn fiter_ctrl(&self) -> &FiterCtrl {
        &self.fiter_ctrl
    }
    #[doc = "0x0c - Control register for 1st comparator"]
    #[inline(always)]
    pub const fn comp1_ctrl(&self) -> &Comp1Ctrl {
        &self.comp1_ctrl
    }
    #[doc = "0x10 - Control register for 1st comparator"]
    #[inline(always)]
    pub const fn comp2_ctrl(&self) -> &Comp2Ctrl {
        &self.comp2_ctrl
    }
    #[doc = "0x14 - Control register for 1st comparator"]
    #[inline(always)]
    pub const fn comp3_ctrl(&self) -> &Comp3Ctrl {
        &self.comp3_ctrl
    }
    #[doc = "0x18 - Comparator 1 primary match data: Low 32 bits"]
    #[inline(always)]
    pub const fn comp1_pmatch_low(&self) -> &Comp1PmatchLow {
        &self.comp1_pmatch_low
    }
    #[doc = "0x1c - Comparator 1 primary match data: High 32 bits"]
    #[inline(always)]
    pub const fn comp1_pmatch_high(&self) -> &Comp1PmatchHigh {
        &self.comp1_pmatch_high
    }
    #[doc = "0x20 - Comparator 1 secondary match data: Low 32 bits"]
    #[inline(always)]
    pub const fn comp1_smatch_low(&self) -> &Comp1SmatchLow {
        &self.comp1_smatch_low
    }
}
#[doc = "CTRL (rw) register accessor: Itrace control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Itrace control register"]
pub mod ctrl;
#[doc = "FITER_CTRL (rw) register accessor: Instruction filter control\n\nYou can [`read`](crate::Reg::read) this register and get [`fiter_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fiter_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fiter_ctrl`]
module"]
#[doc(alias = "FITER_CTRL")]
pub type FiterCtrl = crate::Reg<fiter_ctrl::FiterCtrlSpec>;
#[doc = "Instruction filter control"]
pub mod fiter_ctrl;
#[doc = "COMP1_CTRL (rw) register accessor: Control register for 1st comparator\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_ctrl`]
module"]
#[doc(alias = "COMP1_CTRL")]
pub type Comp1Ctrl = crate::Reg<comp1_ctrl::Comp1CtrlSpec>;
#[doc = "Control register for 1st comparator"]
pub mod comp1_ctrl;
#[doc = "COMP2_CTRL (rw) register accessor: Control register for 1st comparator\n\nYou can [`read`](crate::Reg::read) this register and get [`comp2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_ctrl`]
module"]
#[doc(alias = "COMP2_CTRL")]
pub type Comp2Ctrl = crate::Reg<comp2_ctrl::Comp2CtrlSpec>;
#[doc = "Control register for 1st comparator"]
pub mod comp2_ctrl;
#[doc = "COMP3_CTRL (rw) register accessor: Control register for 1st comparator\n\nYou can [`read`](crate::Reg::read) this register and get [`comp3_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp3_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp3_ctrl`]
module"]
#[doc(alias = "COMP3_CTRL")]
pub type Comp3Ctrl = crate::Reg<comp3_ctrl::Comp3CtrlSpec>;
#[doc = "Control register for 1st comparator"]
pub mod comp3_ctrl;
#[doc = "COMP1_PMATCH_LOW (rw) register accessor: Comparator 1 primary match data: Low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_pmatch_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_pmatch_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_pmatch_low`]
module"]
#[doc(alias = "COMP1_PMATCH_LOW")]
pub type Comp1PmatchLow = crate::Reg<comp1_pmatch_low::Comp1PmatchLowSpec>;
#[doc = "Comparator 1 primary match data: Low 32 bits"]
pub mod comp1_pmatch_low;
#[doc = "COMP1_PMATCH_HIGH (rw) register accessor: Comparator 1 primary match data: High 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_pmatch_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_pmatch_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_pmatch_high`]
module"]
#[doc(alias = "COMP1_PMATCH_HIGH")]
pub type Comp1PmatchHigh = crate::Reg<comp1_pmatch_high::Comp1PmatchHighSpec>;
#[doc = "Comparator 1 primary match data: High 32 bits"]
pub mod comp1_pmatch_high;
#[doc = "COMP1_SMATCH_LOW (rw) register accessor: Comparator 1 secondary match data: Low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1_smatch_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_smatch_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_smatch_low`]
module"]
#[doc(alias = "COMP1_SMATCH_LOW")]
pub type Comp1SmatchLow = crate::Reg<comp1_smatch_low::Comp1SmatchLowSpec>;
#[doc = "Comparator 1 secondary match data: Low 32 bits"]
pub mod comp1_smatch_low;
