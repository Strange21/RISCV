#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpio_direction: GpioDirection,
    _reserved1: [u8; 0x04],
    gpio_data: GpioData,
    _reserved2: [u8; 0x04],
    gpio_set: GpioSet,
    _reserved3: [u8; 0x04],
    gpio_clear: GpioClear,
    _reserved4: [u8; 0x04],
    gpio_toggle: GpioToggle,
    _reserved5: [u8; 0x0c],
    gpio_intr: GpioIntr,
}
impl RegisterBlock {
    #[doc = "0x00 - Select the direction of the GPIOs. Each bit position corresponds to the respective GPIO pin. 0 - Output, 1 - Input"]
    #[inline(always)]
    pub const fn gpio_direction(&self) -> &GpioDirection {
        &self.gpio_direction
    }
    #[doc = "0x08 - Contains the data to be sent out if the GPIO pin is configured as output and the data recieved when configured as input. Each bit position corresponds to the respective GPIO pin."]
    #[inline(always)]
    pub const fn gpio_data(&self) -> &GpioData {
        &self.gpio_data
    }
    #[doc = "0x10 - To set the respective GPIO pins"]
    #[inline(always)]
    pub const fn gpio_set(&self) -> &GpioSet {
        &self.gpio_set
    }
    #[doc = "0x18 - To clear the respective GPIO pins"]
    #[inline(always)]
    pub const fn gpio_clear(&self) -> &GpioClear {
        &self.gpio_clear
    }
    #[doc = "0x20 - To invert the respective GPIO pins"]
    #[inline(always)]
    pub const fn gpio_toggle(&self) -> &GpioToggle {
        &self.gpio_toggle
    }
    #[doc = "0x30 - To enable the interrupt of respective GPIO pins"]
    #[inline(always)]
    pub const fn gpio_intr(&self) -> &GpioIntr {
        &self.gpio_intr
    }
}
#[doc = "GPIO_DIRECTION (rw) register accessor: Select the direction of the GPIOs. Each bit position corresponds to the respective GPIO pin. 0 - Output, 1 - Input\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_direction::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_direction::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_direction`]
module"]
#[doc(alias = "GPIO_DIRECTION")]
pub type GpioDirection = crate::Reg<gpio_direction::GpioDirectionSpec>;
#[doc = "Select the direction of the GPIOs. Each bit position corresponds to the respective GPIO pin. 0 - Output, 1 - Input"]
pub mod gpio_direction;
#[doc = "GPIO_DATA (rw) register accessor: Contains the data to be sent out if the GPIO pin is configured as output and the data recieved when configured as input. Each bit position corresponds to the respective GPIO pin.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_data`]
module"]
#[doc(alias = "GPIO_DATA")]
pub type GpioData = crate::Reg<gpio_data::GpioDataSpec>;
#[doc = "Contains the data to be sent out if the GPIO pin is configured as output and the data recieved when configured as input. Each bit position corresponds to the respective GPIO pin."]
pub mod gpio_data;
#[doc = "GPIO_SET (rw) register accessor: To set the respective GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_set`]
module"]
#[doc(alias = "GPIO_SET")]
pub type GpioSet = crate::Reg<gpio_set::GpioSetSpec>;
#[doc = "To set the respective GPIO pins"]
pub mod gpio_set;
#[doc = "GPIO_CLEAR (rw) register accessor: To clear the respective GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_clear`]
module"]
#[doc(alias = "GPIO_CLEAR")]
pub type GpioClear = crate::Reg<gpio_clear::GpioClearSpec>;
#[doc = "To clear the respective GPIO pins"]
pub mod gpio_clear;
#[doc = "GPIO_TOGGLE (rw) register accessor: To invert the respective GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_toggle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_toggle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_toggle`]
module"]
#[doc(alias = "GPIO_TOGGLE")]
pub type GpioToggle = crate::Reg<gpio_toggle::GpioToggleSpec>;
#[doc = "To invert the respective GPIO pins"]
pub mod gpio_toggle;
#[doc = "GPIO_INTR (rw) register accessor: To enable the interrupt of respective GPIO pins\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_intr`]
module"]
#[doc(alias = "GPIO_INTR")]
pub type GpioIntr = crate::Reg<gpio_intr::GpioIntrSpec>;
#[doc = "To enable the interrupt of respective GPIO pins"]
pub mod gpio_intr;
