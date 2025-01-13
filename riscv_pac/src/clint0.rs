#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    msip: Msip,
    _reserved1: [u8; 0x3ffc],
    timecmp: Timecmp,
    _reserved2: [u8; 0x7ff0],
    time: Time,
}
impl RegisterBlock {
    #[doc = "0x00 - Machine Software interrupt pending register"]
    #[inline(always)]
    pub const fn msip(&self) -> &Msip {
        &self.msip
    }
    #[doc = "0x4000..0x4008 - Time compare register"]
    #[inline(always)]
    pub const fn timecmp(&self) -> &Timecmp {
        &self.timecmp
    }
    #[doc = "0xbff8..0xc000 - System Time register - low"]
    #[inline(always)]
    pub const fn time(&self) -> &Time {
        &self.time
    }
}
#[doc = "MSIP (rw) register accessor: Machine Software interrupt pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`msip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip`]
module"]
#[doc(alias = "MSIP")]
pub type Msip = crate::Reg<msip::MsipSpec>;
#[doc = "Machine Software interrupt pending register"]
pub mod msip;
#[doc = "TIMECMP (rw) register accessor: Time compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`timecmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timecmp`]
module"]
#[doc(alias = "TIMECMP")]
pub type Timecmp = crate::Reg<timecmp::TimecmpSpec>;
#[doc = "Time compare register"]
pub mod timecmp;
#[doc = "TIME (rw) register accessor: System Time register - low\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`]
module"]
#[doc(alias = "TIME")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "System Time register - low"]
pub mod time;
