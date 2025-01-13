#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x02],
    clock_ctrl: ClockCtrl,
    count: Count,
    rptd_count: RptdCount,
    duty_cycle: DutyCycle,
    period: Period,
    capture_inp: CaptureInp,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Clock control register"]
    #[inline(always)]
    pub const fn clock_ctrl(&self) -> &ClockCtrl {
        &self.clock_ctrl
    }
    #[doc = "0x08 - Counter register"]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x0c - Repeated count register"]
    #[inline(always)]
    pub const fn rptd_count(&self) -> &RptdCount {
        &self.rptd_count
    }
    #[doc = "0x10 - PWM duty cycle register"]
    #[inline(always)]
    pub const fn duty_cycle(&self) -> &DutyCycle {
        &self.duty_cycle
    }
    #[doc = "0x14 - PWM period register"]
    #[inline(always)]
    pub const fn period(&self) -> &Period {
        &self.period
    }
    #[doc = "0x18 - Timer capture input register"]
    #[inline(always)]
    pub const fn capture_inp(&self) -> &CaptureInp {
        &self.capture_inp
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "CLOCK_CTRL (rw) register accessor: Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctrl`]
module"]
#[doc(alias = "CLOCK_CTRL")]
pub type ClockCtrl = crate::Reg<clock_ctrl::ClockCtrlSpec>;
#[doc = "Clock control register"]
pub mod clock_ctrl;
#[doc = "COUNT (r) register accessor: Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
#[doc(alias = "COUNT")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "Counter register"]
pub mod count;
#[doc = "RPTD_COUNT (r) register accessor: Repeated count register\n\nYou can [`read`](crate::Reg::read) this register and get [`rptd_count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rptd_count`]
module"]
#[doc(alias = "RPTD_COUNT")]
pub type RptdCount = crate::Reg<rptd_count::RptdCountSpec>;
#[doc = "Repeated count register"]
pub mod rptd_count;
#[doc = "DUTY_CYCLE (rw) register accessor: PWM duty cycle register\n\nYou can [`read`](crate::Reg::read) this register and get [`duty_cycle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duty_cycle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@duty_cycle`]
module"]
#[doc(alias = "DUTY_CYCLE")]
pub type DutyCycle = crate::Reg<duty_cycle::DutyCycleSpec>;
#[doc = "PWM duty cycle register"]
pub mod duty_cycle;
#[doc = "PERIOD (rw) register accessor: PWM period register\n\nYou can [`read`](crate::Reg::read) this register and get [`period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period`]
module"]
#[doc(alias = "PERIOD")]
pub type Period = crate::Reg<period::PeriodSpec>;
#[doc = "PWM period register"]
pub mod period;
#[doc = "CAPTURE_INP (rw) register accessor: Timer capture input register\n\nYou can [`read`](crate::Reg::read) this register and get [`capture_inp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capture_inp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capture_inp`]
module"]
#[doc(alias = "CAPTURE_INP")]
pub type CaptureInp = crate::Reg<capture_inp::CaptureInpSpec>;
#[doc = "Timer capture input register"]
pub mod capture_inp;
