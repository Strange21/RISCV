#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aes_input: AesInput,
    _reserved1: [u8; 0x18],
    aes_key: AesKey,
    _reserved2: [u8; 0x18],
    aes_output: AesOutput,
    _reserved3: [u8; 0x08],
    aes_iv: AesIv,
    _reserved4: [u8; 0x08],
    aes_ctrl: AesCtrl,
    aes_status: AesStatus,
    _reserved6: [u8; 0x0e],
    aes_next_blk: AesNextBlk,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Input text register. Input is given 64 bits at a time."]
    #[inline(always)]
    pub const fn aes_input(&self) -> &AesInput {
        &self.aes_input
    }
    #[doc = "0x20..0x28 - Key data register. Key is given 64 bits at a time and needs to be written depending on the keylen select"]
    #[inline(always)]
    pub const fn aes_key(&self) -> &AesKey {
        &self.aes_key
    }
    #[doc = "0x40..0x48 - Output register. Output is read 64 bits at a time."]
    #[inline(always)]
    pub const fn aes_output(&self) -> &AesOutput {
        &self.aes_output
    }
    #[doc = "0x50..0x58 - Initialization vector register. IV is given 64 bits at a time."]
    #[inline(always)]
    pub const fn aes_iv(&self) -> &AesIv {
        &self.aes_iv
    }
    #[doc = "0x60 - Control register"]
    #[inline(always)]
    pub const fn aes_ctrl(&self) -> &AesCtrl {
        &self.aes_ctrl
    }
    #[doc = "0x61 - To check the status."]
    #[inline(always)]
    pub const fn aes_status(&self) -> &AesStatus {
        &self.aes_status
    }
    #[doc = "0x70..0x78 - The input data of the next block of the same message"]
    #[inline(always)]
    pub const fn aes_next_blk(&self) -> &AesNextBlk {
        &self.aes_next_blk
    }
}
#[doc = "AES_INPUT (w) register accessor: Input text register. Input is given 64 bits at a time.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_input`]
module"]
#[doc(alias = "AES_INPUT")]
pub type AesInput = crate::Reg<aes_input::AesInputSpec>;
#[doc = "Input text register. Input is given 64 bits at a time."]
pub mod aes_input;
#[doc = "AES_KEY (w) register accessor: Key data register. Key is given 64 bits at a time and needs to be written depending on the keylen select\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_key`]
module"]
#[doc(alias = "AES_KEY")]
pub type AesKey = crate::Reg<aes_key::AesKeySpec>;
#[doc = "Key data register. Key is given 64 bits at a time and needs to be written depending on the keylen select"]
pub mod aes_key;
#[doc = "AES_OUTPUT (r) register accessor: Output register. Output is read 64 bits at a time.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_output::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_output`]
module"]
#[doc(alias = "AES_OUTPUT")]
pub type AesOutput = crate::Reg<aes_output::AesOutputSpec>;
#[doc = "Output register. Output is read 64 bits at a time."]
pub mod aes_output;
#[doc = "AES_IV (w) register accessor: Initialization vector register. IV is given 64 bits at a time.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_iv::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_iv`]
module"]
#[doc(alias = "AES_IV")]
pub type AesIv = crate::Reg<aes_iv::AesIvSpec>;
#[doc = "Initialization vector register. IV is given 64 bits at a time."]
pub mod aes_iv;
#[doc = "AES_CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ctrl`]
module"]
#[doc(alias = "AES_CTRL")]
pub type AesCtrl = crate::Reg<aes_ctrl::AesCtrlSpec>;
#[doc = "Control register"]
pub mod aes_ctrl;
#[doc = "AES_STATUS (rw) register accessor: To check the status.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_status`]
module"]
#[doc(alias = "AES_STATUS")]
pub type AesStatus = crate::Reg<aes_status::AesStatusSpec>;
#[doc = "To check the status."]
pub mod aes_status;
#[doc = "AES_NEXT_BLK (w) register accessor: The input data of the next block of the same message\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_next_blk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_next_blk`]
module"]
#[doc(alias = "AES_NEXT_BLK")]
pub type AesNextBlk = crate::Reg<aes_next_blk::AesNextBlkSpec>;
#[doc = "The input data of the next block of the same message"]
pub mod aes_next_blk;
