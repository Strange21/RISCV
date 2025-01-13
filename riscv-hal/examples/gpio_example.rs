#![no_std]
#![no_main]

use panic_halt as _;
use riscv_pac::Gpio;
use riscv_rt::entry;
use embedded_hal::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};
use riscv_hal::{device::Device, gpio::GpioPin};

#[entry]
fn main() -> ! {
    // Initialize the device with Gpio
    let peripherals = unsafe { riscv_pac::Peripherals::steal() };
    let mut device = Device::new(peripherals.gpio);
    
    // Get GPIO pin 0
    let mut pin0 = device.pins[0].take().unwrap();
    
    // Set pin as output (high)
    pin0.set_high().unwrap();
    
    // Toggle the pin
    pin0.toggle().unwrap();
    
    // Read pin state
    let _is_high = pin0.is_high().unwrap();
    
    // Set pin as output (low)
    pin0.set_low().unwrap();
    
    // Using pin as input
    let _is_low = pin0.is_low().unwrap();

    loop {
        // Main loop
    }
}