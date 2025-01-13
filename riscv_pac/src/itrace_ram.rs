#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    impl_: Impl,
    _reserved2: [u8; 0x08],
    start_low: StartLow,
    start_high: StartHigh,
    limit_low: LimitLow,
    limit_high: LimitHigh,
    wp_low: WpLow,
    wp_high: WpHigh,
    rp_low: RpLow,
    rp_high: RpHigh,
    _reserved10: [u8; 0x10],
    data: Data,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Implementation details register"]
    #[inline(always)]
    pub const fn impl_(&self) -> &Impl {
        &self.impl_
    }
    #[doc = "0x10 - The RAM start address register. Low 32 bits"]
    #[inline(always)]
    pub const fn start_low(&self) -> &StartLow {
        &self.start_low
    }
    #[doc = "0x14 - The RAM start address register. High 32 bits"]
    #[inline(always)]
    pub const fn start_high(&self) -> &StartHigh {
        &self.start_high
    }
    #[doc = "0x18 - The RAM end address register. Low 32 bits"]
    #[inline(always)]
    pub const fn limit_low(&self) -> &LimitLow {
        &self.limit_low
    }
    #[doc = "0x1c - The RAM end address register. High 32 bits"]
    #[inline(always)]
    pub const fn limit_high(&self) -> &LimitHigh {
        &self.limit_high
    }
    #[doc = "0x20 - Write pointer of trace packet into RAM. Low 32 bits"]
    #[inline(always)]
    pub const fn wp_low(&self) -> &WpLow {
        &self.wp_low
    }
    #[doc = "0x24 - Write pointer of trace packet into RAM. High 32 bits"]
    #[inline(always)]
    pub const fn wp_high(&self) -> &WpHigh {
        &self.wp_high
    }
    #[doc = "0x28 - Read pointer of trace packet into RAM. Low 32 bits"]
    #[inline(always)]
    pub const fn rp_low(&self) -> &RpLow {
        &self.rp_low
    }
    #[doc = "0x2c - Read pointer of trace packet into RAM. High 32 bits"]
    #[inline(always)]
    pub const fn rp_high(&self) -> &RpHigh {
        &self.rp_high
    }
    #[doc = "0x40 - RAM data is read by external host via this register."]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
}
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "IMPL (r) register accessor: Implementation details register\n\nYou can [`read`](crate::Reg::read) this register and get [`impl_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@impl_`]
module"]
#[doc(alias = "IMPL")]
pub type Impl = crate::Reg<impl_::ImplSpec>;
#[doc = "Implementation details register"]
pub mod impl_;
#[doc = "START_LOW (rw) register accessor: The RAM start address register. Low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`start_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start_low`]
module"]
#[doc(alias = "START_LOW")]
pub type StartLow = crate::Reg<start_low::StartLowSpec>;
#[doc = "The RAM start address register. Low 32 bits"]
pub mod start_low;
#[doc = "START_HIGH (rw) register accessor: The RAM start address register. High 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`start_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start_high`]
module"]
#[doc(alias = "START_HIGH")]
pub type StartHigh = crate::Reg<start_high::StartHighSpec>;
#[doc = "The RAM start address register. High 32 bits"]
pub mod start_high;
#[doc = "LIMIT_LOW (rw) register accessor: The RAM end address register. Low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`limit_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limit_low`]
module"]
#[doc(alias = "LIMIT_LOW")]
pub type LimitLow = crate::Reg<limit_low::LimitLowSpec>;
#[doc = "The RAM end address register. Low 32 bits"]
pub mod limit_low;
#[doc = "LIMIT_HIGH (rw) register accessor: The RAM end address register. High 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`limit_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limit_high`]
module"]
#[doc(alias = "LIMIT_HIGH")]
pub type LimitHigh = crate::Reg<limit_high::LimitHighSpec>;
#[doc = "The RAM end address register. High 32 bits"]
pub mod limit_high;
#[doc = "WP_LOW (rw) register accessor: Write pointer of trace packet into RAM. Low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`wp_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wp_low`]
module"]
#[doc(alias = "WP_LOW")]
pub type WpLow = crate::Reg<wp_low::WpLowSpec>;
#[doc = "Write pointer of trace packet into RAM. Low 32 bits"]
pub mod wp_low;
#[doc = "WP_HIGH (rw) register accessor: Write pointer of trace packet into RAM. High 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`wp_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wp_high`]
module"]
#[doc(alias = "WP_HIGH")]
pub type WpHigh = crate::Reg<wp_high::WpHighSpec>;
#[doc = "Write pointer of trace packet into RAM. High 32 bits"]
pub mod wp_high;
#[doc = "RP_LOW (rw) register accessor: Read pointer of trace packet into RAM. Low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`rp_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rp_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rp_low`]
module"]
#[doc(alias = "RP_LOW")]
pub type RpLow = crate::Reg<rp_low::RpLowSpec>;
#[doc = "Read pointer of trace packet into RAM. Low 32 bits"]
pub mod rp_low;
#[doc = "RP_HIGH (rw) register accessor: Read pointer of trace packet into RAM. High 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`rp_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rp_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rp_high`]
module"]
#[doc(alias = "RP_HIGH")]
pub type RpHigh = crate::Reg<rp_high::RpHighSpec>;
#[doc = "Read pointer of trace packet into RAM. High 32 bits"]
pub mod rp_high;
#[doc = "DATA (r) register accessor: RAM data is read by external host via this register.\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "RAM data is read by external host via this register."]
pub mod data;
