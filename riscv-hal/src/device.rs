use crate::gpio::{GpioOps, GpioPin};

pub struct Device<GPIO: GpioOps> {
    pub gpio: GPIO,
    pub pins: [Option<GpioPin<GPIO>>; 32],
}

impl<GPIO: GpioOps + Clone> Device<GPIO> {
    pub fn new(gpio: GPIO) -> Self {
        let mut pins: [Option<GpioPin<GPIO>>; 32] = Default::default();
        for i in 0..32 {
            pins[i] = Some(GpioPin::new(gpio.clone(), i as u8));
        }
        Device { gpio, pins }
    }
}
