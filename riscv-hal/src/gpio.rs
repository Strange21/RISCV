use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};
use core::convert::Infallible;

/// Trait defining GPIO operations.
pub trait GpioOps {
    fn set_direction(&self, pin: u8, direction: bool);
    fn read_data(&self) -> u32;
    fn write_data(&self, value: u32);
    fn set_pin(&self, pin: u8);
    fn clear_pin(&self, pin: u8);
    fn toggle_pin(&self, pin: u8);
    fn enable_interrupt(&self, pin: u8);
    fn disable_interrupt(&self, pin: u8);
}

impl GpioOps for riscv_pac::Gpio {
    fn set_direction(&self, pin: u8, direction: bool) {
        let mut value = self.gpio_direction().read().bits();
        if direction {
            value |= 1 << pin;
        } else {
            value &= !(1 << pin);
        }
        self.gpio_direction().write(|w| unsafe { w.bits(value) });
    }

    fn read_data(&self) -> u32 {
        self.gpio_data().read().bits()
    }

    fn write_data(&self, value: u32) {
        self.gpio_data().write(|w| unsafe { w.bits(value) });
    }

    fn set_pin(&self, pin: u8) {
        self.gpio_set().write(|w| unsafe { w.bits(1 << pin) });
    }

    fn clear_pin(&self, pin: u8) {
        self.gpio_clear().write(|w| unsafe { w.bits(1 << pin) });
    }

    fn toggle_pin(&self, pin: u8) {
        self.gpio_toggle().write(|w| unsafe { w.bits(1 << pin) });
    }

    fn enable_interrupt(&self, pin: u8) {
        let mut value = self.gpio_intr().read().bits();
        value |= 1 << pin;
        self.gpio_intr().write(|w| unsafe { w.bits(value) });
    }

    fn disable_interrupt(&self, pin: u8) {
        let mut value = self.gpio_intr().read().bits();
        value &= !(1 << pin);
        self.gpio_intr().write(|w| unsafe { w.bits(value) });
    }
}

/// Trait defining GPIO pin operations.
pub trait GpioPinOps {
    fn set_high(&self);
    fn set_low(&self);
    fn toggle(&self);
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
}

pub struct GpioPin<GPIO: GpioOps> {
    index: u8,
    gpio: GPIO,
}

impl<GPIO: GpioOps> GpioPin<GPIO> {
    pub fn new(gpio: GPIO, index: u8) -> Self {
        GpioPin { index, gpio }
    }
}

impl<GPIO: GpioOps> GpioPinOps for GpioPin<GPIO> {
    fn set_high(&self) {
        self.gpio.set_pin(self.index);
    }

    fn set_low(&self) {
        self.gpio.clear_pin(self.index);
    }

    fn toggle(&self) {
        self.gpio.toggle_pin(self.index);
    }

    fn is_high(&self) -> bool {
        (self.gpio.read_data() & (1 << self.index)) != 0
    }

    fn is_low(&self) -> bool {
        (self.gpio.read_data() & (1 << self.index)) == 0
    }
}

impl<GPIO: GpioOps> OutputPin for GpioPin<GPIO> {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        GpioPinOps::set_high(self);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        GpioPinOps::set_low(self);
        Ok(())
    }
}

impl<GPIO: GpioOps> StatefulOutputPin for GpioPin<GPIO> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(GpioPinOps::is_high(self))
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(GpioPinOps::is_low(self))
    }
}

impl<GPIO: GpioOps> ToggleableOutputPin for GpioPin<GPIO> {
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        GpioPinOps::toggle(self);
        Ok(())
    }
}

impl<GPIO: GpioOps> InputPin for GpioPin<GPIO> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(GpioPinOps::is_high(self))
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(GpioPinOps::is_low(self))
    }
}

/// Macro to define GPIO pins.
macro_rules! gpio {
    ($($pin:ident, $index:expr),+) => {
        $(
            pub type $pin = GpioPin<riscv_pac::Gpio>;

        )+
    };
}

// Define GPIO pins using the macro.
gpio!(
    Pin0, 0,
    Pin1, 1,
    Pin2, 2,
    Pin3, 3,
    Pin4, 4,
    Pin5, 5,
    Pin6, 6,
    Pin7, 7,
    Pin8, 8,
    Pin9, 9,
    Pin10, 10,
    Pin11, 11,
    Pin12, 12,
    Pin13, 13,
    Pin14, 14,
    Pin15, 15,
    Pin16, 16,
    Pin17, 17,
    Pin18, 18,
    Pin19, 19,
    Pin20, 20,
    Pin21, 21,
    Pin22, 22,
    Pin23, 23,
    Pin24, 24,
    Pin25, 25,
    Pin26, 26,
    Pin27, 27,
    Pin28, 28,
    Pin29, 29,
    Pin30, 30,
    Pin31, 31
);
