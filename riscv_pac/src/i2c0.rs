#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    s2: S2,
    _reserved1: [u8; 0x07],
    ctrl: Ctrl,
    _reserved2: [u8; 0x07],
    s0: S0,
    _reserved3: [u8; 0x07],
    status: Status,
    _reserved4: [u8; 0x07],
    s01: S01,
    _reserved5: [u8; 0x04],
    s3: S3,
    _reserved6: [u8; 0x04],
    time: Time,
    _reserved7: [u8; 0x04],
    scl: Scl,
}
impl RegisterBlock {
    #[doc = "0x00 - Prescalar register"]
    #[inline(always)]
    pub const fn s2(&self) -> &S2 {
        &self.s2
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x10 - Data Transmission register"]
    #[inline(always)]
    pub const fn s0(&self) -> &S0 {
        &self.s0
    }
    #[doc = "0x18 - Status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x20 - I2C Own Address Slave Register"]
    #[inline(always)]
    pub const fn s01(&self) -> &S01 {
        &self.s01
    }
    #[doc = "0x28 - Interrupt vector register"]
    #[inline(always)]
    pub const fn s3(&self) -> &S3 {
        &self.s3
    }
    #[doc = "0x30 - I2C timeout calculation and interrupt register"]
    #[inline(always)]
    pub const fn time(&self) -> &Time {
        &self.time
    }
    #[doc = "0x38 - Clock period register"]
    #[inline(always)]
    pub const fn scl(&self) -> &Scl {
        &self.scl
    }
}
#[doc = "S2 (rw) register accessor: Prescalar register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2`]
module"]
pub type S2 = crate::Reg<s2::S2Spec>;
#[doc = "Prescalar register"]
pub mod s2;
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "S0 (rw) register accessor: Data Transmission register\n\nYou can [`read`](crate::Reg::read) this register and get [`s0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s0`]
module"]
pub type S0 = crate::Reg<s0::S0Spec>;
#[doc = "Data Transmission register"]
pub mod s0;
#[doc = "STATUS (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register"]
pub mod status;
#[doc = "S01 (rw) register accessor: I2C Own Address Slave Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s01`]
module"]
pub type S01 = crate::Reg<s01::S01Spec>;
#[doc = "I2C Own Address Slave Register"]
pub mod s01;
#[doc = "S3 (rw) register accessor: Interrupt vector register\n\nYou can [`read`](crate::Reg::read) this register and get [`s3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3`]
module"]
pub type S3 = crate::Reg<s3::S3Spec>;
#[doc = "Interrupt vector register"]
pub mod s3;
#[doc = "TIME (rw) register accessor: I2C timeout calculation and interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`]
module"]
#[doc(alias = "TIME")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "I2C timeout calculation and interrupt register"]
pub mod time;
#[doc = "SCL (rw) register accessor: Clock period register\n\nYou can [`read`](crate::Reg::read) this register and get [`scl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scl`]
module"]
#[doc(alias = "SCL")]
pub type Scl = crate::Reg<scl::SclSpec>;
#[doc = "Clock period register"]
pub mod scl;
