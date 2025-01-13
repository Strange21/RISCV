#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sha_input: ShaInput,
    _reserved1: [u8; 0x78],
    sha_output: ShaOutput,
    _reserved2: [u8; 0x38],
    sha_ctrl: ShaCtrl,
    sha_status: ShaStatus,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Input text register. Input is given 64 bits at a time."]
    #[inline(always)]
    pub const fn sha_input(&self) -> &ShaInput {
        &self.sha_input
    }
    #[doc = "0x80..0x88 - Output text register. Input is given 64 bits at a time."]
    #[inline(always)]
    pub const fn sha_output(&self) -> &ShaOutput {
        &self.sha_output
    }
    #[doc = "0xc0 - Control register"]
    #[inline(always)]
    pub const fn sha_ctrl(&self) -> &ShaCtrl {
        &self.sha_ctrl
    }
    #[doc = "0xc1 - To read the status register"]
    #[inline(always)]
    pub const fn sha_status(&self) -> &ShaStatus {
        &self.sha_status
    }
}
#[doc = "SHA_INPUT (w) register accessor: Input text register. Input is given 64 bits at a time.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_input`]
module"]
#[doc(alias = "SHA_INPUT")]
pub type ShaInput = crate::Reg<sha_input::ShaInputSpec>;
#[doc = "Input text register. Input is given 64 bits at a time."]
pub mod sha_input;
#[doc = "SHA_OUTPUT (w) register accessor: Output text register. Input is given 64 bits at a time.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_output::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_output`]
module"]
#[doc(alias = "SHA_OUTPUT")]
pub type ShaOutput = crate::Reg<sha_output::ShaOutputSpec>;
#[doc = "Output text register. Input is given 64 bits at a time."]
pub mod sha_output;
#[doc = "SHA_CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_ctrl`]
module"]
#[doc(alias = "SHA_CTRL")]
pub type ShaCtrl = crate::Reg<sha_ctrl::ShaCtrlSpec>;
#[doc = "Control register"]
pub mod sha_ctrl;
#[doc = "SHA_STATUS (r) register accessor: To read the status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sha_status`]
module"]
#[doc(alias = "SHA_STATUS")]
pub type ShaStatus = crate::Reg<sha_status::ShaStatusSpec>;
#[doc = "To read the status register"]
pub mod sha_status;
