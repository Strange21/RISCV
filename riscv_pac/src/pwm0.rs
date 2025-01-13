#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clock_ctrl: ClockCtrl,
    _reserved1: [u8; 0x02],
    ctrl: Ctrl,
    _reserved2: [u8; 0x02],
    period: Period,
    duty_cycle: DutyCycle,
    deadband_delay: DeadbandDelay,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock prescalar register"]
    #[inline(always)]
    pub const fn clock_ctrl(&self) -> &ClockCtrl {
        &self.clock_ctrl
    }
    #[doc = "0x04 - PWM Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - PWM period register"]
    #[inline(always)]
    pub const fn period(&self) -> &Period {
        &self.period
    }
    #[doc = "0x0c - PWM Duty_Cycle register"]
    #[inline(always)]
    pub const fn duty_cycle(&self) -> &DutyCycle {
        &self.duty_cycle
    }
    #[doc = "0x10 - PWM Deadband delay register"]
    #[inline(always)]
    pub const fn deadband_delay(&self) -> &DeadbandDelay {
        &self.deadband_delay
    }
}
#[doc = "CLOCK_CTRL (rw) register accessor: Clock prescalar register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctrl`]
module"]
#[doc(alias = "CLOCK_CTRL")]
pub type ClockCtrl = crate::Reg<clock_ctrl::ClockCtrlSpec>;
#[doc = "Clock prescalar register"]
pub mod clock_ctrl;
#[doc = "CTRL (rw) register accessor: PWM Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "PWM Control register"]
pub mod ctrl;
#[doc = "PERIOD (rw) register accessor: PWM period register\n\nYou can [`read`](crate::Reg::read) this register and get [`period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period`]
module"]
#[doc(alias = "PERIOD")]
pub type Period = crate::Reg<period::PeriodSpec>;
#[doc = "PWM period register"]
pub mod period;
#[doc = "DUTY_CYCLE (rw) register accessor: PWM Duty_Cycle register\n\nYou can [`read`](crate::Reg::read) this register and get [`duty_cycle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duty_cycle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@duty_cycle`]
module"]
#[doc(alias = "DUTY_CYCLE")]
pub type DutyCycle = crate::Reg<duty_cycle::DutyCycleSpec>;
#[doc = "PWM Duty_Cycle register"]
pub mod duty_cycle;
#[doc = "DEADBAND_DELAY (rw) register accessor: PWM Deadband delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`deadband_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deadband_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deadband_delay`]
module"]
#[doc(alias = "DEADBAND_DELAY")]
pub type DeadbandDelay = crate::Reg<deadband_delay::DeadbandDelaySpec>;
#[doc = "PWM Deadband delay register"]
pub mod deadband_delay;
