#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mux0: Mux0,
    mux1: Mux1,
    mux2: Mux2,
    mux3: Mux3,
    mux4: Mux4,
    mux5: Mux5,
    mux6: Mux6,
    mux7: Mux7,
}
impl RegisterBlock {
    #[doc = "0x00 - Select between GPIO0 and PWM0. 0 - GPIO, 1 - PWM"]
    #[inline(always)]
    pub const fn mux0(&self) -> &Mux0 {
        &self.mux0
    }
    #[doc = "0x04 - Select between GPIO1 and PWM1. 0 - GPIO, 1 - PWM"]
    #[inline(always)]
    pub const fn mux1(&self) -> &Mux1 {
        &self.mux1
    }
    #[doc = "0x08 - Select between GPIO2 and PWM2. 0 - GPIO, 1 - PWM"]
    #[inline(always)]
    pub const fn mux2(&self) -> &Mux2 {
        &self.mux2
    }
    #[doc = "0x0c - Select between GPIO3 and PWM3. 0 - GPIO, 1 - PWM"]
    #[inline(always)]
    pub const fn mux3(&self) -> &Mux3 {
        &self.mux3
    }
    #[doc = "0x10 - Select between GPIO4 and PWM4. 0 - GPIO, 1 - PWM"]
    #[inline(always)]
    pub const fn mux4(&self) -> &Mux4 {
        &self.mux4
    }
    #[doc = "0x14 - Select between GPIO5 and PWM5. 0 - GPIO, 1 - PWM"]
    #[inline(always)]
    pub const fn mux5(&self) -> &Mux5 {
        &self.mux5
    }
    #[doc = "0x18 - Select between GPIO6 and PWM6. 0 - GPIO, 1 - PWM"]
    #[inline(always)]
    pub const fn mux6(&self) -> &Mux6 {
        &self.mux6
    }
    #[doc = "0x1c - Select between GPIO7 and PWM7. 0 - GPIO, 1 - PWM"]
    #[inline(always)]
    pub const fn mux7(&self) -> &Mux7 {
        &self.mux7
    }
}
#[doc = "MUX0 (rw) register accessor: Select between GPIO0 and PWM0. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux0`]
module"]
#[doc(alias = "MUX0")]
pub type Mux0 = crate::Reg<mux0::Mux0Spec>;
#[doc = "Select between GPIO0 and PWM0. 0 - GPIO, 1 - PWM"]
pub mod mux0;
#[doc = "MUX1 (rw) register accessor: Select between GPIO1 and PWM1. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux1`]
module"]
#[doc(alias = "MUX1")]
pub type Mux1 = crate::Reg<mux1::Mux1Spec>;
#[doc = "Select between GPIO1 and PWM1. 0 - GPIO, 1 - PWM"]
pub mod mux1;
#[doc = "MUX2 (rw) register accessor: Select between GPIO2 and PWM2. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux2`]
module"]
#[doc(alias = "MUX2")]
pub type Mux2 = crate::Reg<mux2::Mux2Spec>;
#[doc = "Select between GPIO2 and PWM2. 0 - GPIO, 1 - PWM"]
pub mod mux2;
#[doc = "MUX3 (rw) register accessor: Select between GPIO3 and PWM3. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux3`]
module"]
#[doc(alias = "MUX3")]
pub type Mux3 = crate::Reg<mux3::Mux3Spec>;
#[doc = "Select between GPIO3 and PWM3. 0 - GPIO, 1 - PWM"]
pub mod mux3;
#[doc = "MUX4 (rw) register accessor: Select between GPIO4 and PWM4. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux4`]
module"]
#[doc(alias = "MUX4")]
pub type Mux4 = crate::Reg<mux4::Mux4Spec>;
#[doc = "Select between GPIO4 and PWM4. 0 - GPIO, 1 - PWM"]
pub mod mux4;
#[doc = "MUX5 (rw) register accessor: Select between GPIO5 and PWM5. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux5`]
module"]
#[doc(alias = "MUX5")]
pub type Mux5 = crate::Reg<mux5::Mux5Spec>;
#[doc = "Select between GPIO5 and PWM5. 0 - GPIO, 1 - PWM"]
pub mod mux5;
#[doc = "MUX6 (rw) register accessor: Select between GPIO6 and PWM6. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux6`]
module"]
#[doc(alias = "MUX6")]
pub type Mux6 = crate::Reg<mux6::Mux6Spec>;
#[doc = "Select between GPIO6 and PWM6. 0 - GPIO, 1 - PWM"]
pub mod mux6;
#[doc = "MUX7 (rw) register accessor: Select between GPIO7 and PWM7. 0 - GPIO, 1 - PWM\n\nYou can [`read`](crate::Reg::read) this register and get [`mux7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux7`]
module"]
#[doc(alias = "MUX7")]
pub type Mux7 = crate::Reg<mux7::Mux7Spec>;
#[doc = "Select between GPIO7 and PWM7. 0 - GPIO, 1 - PWM"]
pub mod mux7;
