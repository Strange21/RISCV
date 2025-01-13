#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rsa_input: RsaInput,
    _reserved1: [u8; 0x18],
    rsa_exp: RsaExp,
    _reserved2: [u8; 0x18],
    rsa_mod: RsaMod,
    _reserved3: [u8; 0x18],
    rsa_rsqr_modn: RsaRsqrModn,
    _reserved4: [u8; 0x18],
    rsa_output: RsaOutput,
    _reserved5: [u8; 0x38],
    rsa_ready: RsaReady,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - RSA input register"]
    #[inline(always)]
    pub const fn rsa_input(&self) -> &RsaInput {
        &self.rsa_input
    }
    #[doc = "0x20..0x28 - RSA exponent register"]
    #[inline(always)]
    pub const fn rsa_exp(&self) -> &RsaExp {
        &self.rsa_exp
    }
    #[doc = "0x40..0x48 - RSA modulus register"]
    #[inline(always)]
    pub const fn rsa_mod(&self) -> &RsaMod {
        &self.rsa_mod
    }
    #[doc = "0x60..0x68 - RSA (R^2) % N register"]
    #[inline(always)]
    pub const fn rsa_rsqr_modn(&self) -> &RsaRsqrModn {
        &self.rsa_rsqr_modn
    }
    #[doc = "0x80..0x88 - RSA output register"]
    #[inline(always)]
    pub const fn rsa_output(&self) -> &RsaOutput {
        &self.rsa_output
    }
    #[doc = "0xc0 - RSA is ready to take another input and computation of previous is done"]
    #[inline(always)]
    pub const fn rsa_ready(&self) -> &RsaReady {
        &self.rsa_ready
    }
}
#[doc = "RSA_INPUT (w) register accessor: RSA input register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_input::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_input`]
module"]
#[doc(alias = "RSA_INPUT")]
pub type RsaInput = crate::Reg<rsa_input::RsaInputSpec>;
#[doc = "RSA input register"]
pub mod rsa_input;
#[doc = "RSA_EXP (w) register accessor: RSA exponent register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_exp::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_exp`]
module"]
#[doc(alias = "RSA_EXP")]
pub type RsaExp = crate::Reg<rsa_exp::RsaExpSpec>;
#[doc = "RSA exponent register"]
pub mod rsa_exp;
#[doc = "RSA_MOD (w) register accessor: RSA modulus register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_mod::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_mod`]
module"]
#[doc(alias = "RSA_MOD")]
pub type RsaMod = crate::Reg<rsa_mod::RsaModSpec>;
#[doc = "RSA modulus register"]
pub mod rsa_mod;
#[doc = "RSA_RSqrMODN (w) register accessor: RSA (R^2) % N register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_rsqr_modn::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_rsqr_modn`]
module"]
#[doc(alias = "RSA_RSqrMODN")]
pub type RsaRsqrModn = crate::Reg<rsa_rsqr_modn::RsaRsqrModnSpec>;
#[doc = "RSA (R^2) % N register"]
pub mod rsa_rsqr_modn;
#[doc = "RSA_OUTPUT (r) register accessor: RSA output register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_output::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_output`]
module"]
#[doc(alias = "RSA_OUTPUT")]
pub type RsaOutput = crate::Reg<rsa_output::RsaOutputSpec>;
#[doc = "RSA output register"]
pub mod rsa_output;
#[doc = "RSA_READY (r) register accessor: RSA is ready to take another input and computation of previous is done\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_ready::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsa_ready`]
module"]
#[doc(alias = "RSA_READY")]
pub type RsaReady = crate::Reg<rsa_ready::RsaReadySpec>;
#[doc = "RSA is ready to take another input and computation of previous is done"]
pub mod rsa_ready;
