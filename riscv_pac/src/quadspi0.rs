#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    dcr: Dcr,
    sr: Sr,
    fcr: Fcr,
    dlr: Dlr,
    ccr: Ccr,
    ar: Ar,
    abr: Abr,
    dr: Dr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Device Configuration Register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &Dcr {
        &self.dcr
    }
    #[doc = "0x08 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - Flag Clear Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        &self.fcr
    }
    #[doc = "0x10 - data length register"]
    #[inline(always)]
    pub const fn dlr(&self) -> &Dlr {
        &self.dlr
    }
    #[doc = "0x14 - communication configuration register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x18 - Address Register"]
    #[inline(always)]
    pub const fn ar(&self) -> &Ar {
        &self.ar
    }
    #[doc = "0x1c - Alternate Byte Register"]
    #[inline(always)]
    pub const fn abr(&self) -> &Abr {
        &self.abr
    }
    #[doc = "0x20 - Data Register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
}
#[doc = "CR (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "DCR (rw) register accessor: Device Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`]
module"]
#[doc(alias = "DCR")]
pub type Dcr = crate::Reg<dcr::DcrSpec>;
#[doc = "Device Configuration Register"]
pub mod dcr;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "FCR (rw) register accessor: Flag Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "Flag Clear Register"]
pub mod fcr;
#[doc = "DLR (rw) register accessor: data length register\n\nYou can [`read`](crate::Reg::read) this register and get [`dlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlr`]
module"]
#[doc(alias = "DLR")]
pub type Dlr = crate::Reg<dlr::DlrSpec>;
#[doc = "data length register"]
pub mod dlr;
#[doc = "CCR (rw) register accessor: communication configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "communication configuration register"]
pub mod ccr;
#[doc = "AR (rw) register accessor: Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar`]
module"]
#[doc(alias = "AR")]
pub type Ar = crate::Reg<ar::ArSpec>;
#[doc = "Address Register"]
pub mod ar;
#[doc = "ABR (rw) register accessor: Alternate Byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`abr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abr`]
module"]
#[doc(alias = "ABR")]
pub type Abr = crate::Reg<abr::AbrSpec>;
#[doc = "Alternate Byte Register"]
pub mod abr;
#[doc = "DR (rw) register accessor: Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "Data Register"]
pub mod dr;
