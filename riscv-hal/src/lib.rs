#![no_std]
pub use riscv_pac as pac;

pub mod gpio;
pub mod device;
pub mod timer;

#[cfg(test)]
mod tests {

    use riscv_pac as pac;

    use crate::timer;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_timer0(){
        let timer0 = unsafe { pac::Peripherals::steal().gptimer0 };
        let gptimer0 = timer::GPTimer::new(timer0);
        gptimer0.initialize();
        
        gptimer0.set_mode(timer::TimerMode::DownCounter);
        gptimer0.set_period(10000);
        gptimer0.set_duty_cycle(700);
        gptimer0.enable();

        // let count = gptimer0.read_counter_value();
        // assert_ne!(0, count);

    }
}
