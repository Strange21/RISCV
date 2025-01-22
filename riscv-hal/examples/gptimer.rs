#![no_std]
#![no_main]

use core::panic::PanicInfo;
use riscv_hal::timer::*;
use riscv_pac as pac;

#[riscv_rt::entry]
fn main() -> !{
    let gpt0 = unsafe {pac::Peripherals::steal().gptimer0}; // Replace with actual initialization
    let gptimer = GPTimer::new(gpt0);
    let timer_config = TimerConfig{
        period : 1000,
        prescalar : 7,
        mode: TimerMode::DownCounter,
        interrupt_en: true,
        duty_cycle: 7,
        cnt_en: 1000,
        capture_val: 100,
        output_en: 1,
    };
    // gptimer.initialize(timer_config);
    gptimer.delay_millisecond(timer_config, 100);
    loop {
        
    }
    
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe { riscv::asm::nop() };
    }
}
