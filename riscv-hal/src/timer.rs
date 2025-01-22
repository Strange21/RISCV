use embedded_hal::timer;
use riscv_pac::{gptimer0::{self, duty_cycle}, Gptimer0, Gptimer1};
use volatile::{self, Volatile};

static CHECK_UPDATE_ENABEL: u32 = 0x20000;
static GPT_CONTIN_COUNT_EN: u32 = 1;
static GPT_CONTIN_COUNT:    u32 = 0;

/// Bit Specifications of each GP_TIMER's register
pub const GPT_EN:           u16 = 1 << 0;
pub const GPT_OUTPUT_EN:    u16 = 1 << 4;
pub const COUNT_RESET:      u16 = 1 << 5;
pub const CONTIN_CNT_EN:    u16 = 1 << 6;
pub const PWM_FALL_INTR_EN: u16 = 1 << 7;
pub const PWM_RISE_INTR_EN: u16 = 1 << 8;
pub const CNTR_OFLOW_INTR_EN: u16 = 1 << 9;
pub const CNTR_UFLOW_INTR_EN: u16 = 1 << 10;

pub const GPT_INTR_EN: bool =  true; // Interrupt enable
pub const GPT_INTR_DIS: bool = false; // Interrupt disable

/// Helper functions for register bit manipulation
pub fn gpt_mode(x: u16) -> u16 {
    x << 2
}

pub fn capture_ip(x: u16) -> u16 {
    x << 15
}

// constants from secure_iot.h
pub const CLOCK_FREQUENCY_BASE: u32 =  700000000;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerMode {
    Pwm = 0b00,
    UpCounter = 0b01,
    DownCounter = 0b10,
    UpDownCounter = 0b11,
}

impl TryFrom<u8> for TimerMode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(TimerMode::Pwm),
            0b01 => Ok(TimerMode::UpCounter),
            0b10 => Ok(TimerMode::DownCounter),
            0b11 => Ok(TimerMode::UpDownCounter),
            _ => Err("Invalid TimerMode value"),
        }
    }
}

pub trait TimerOps {
    fn reset(&self);

    fn enable(&self);

    fn set_period(&self, period: u32);

    fn set_dutycycle(&self, duty_cycle: u32, period: u32);

    fn set_prescalar(&self, prescalar: u16);

    fn update_enable(&self);

    fn read_counter_value(&self) -> u32;

    fn read_reptdcount(&self) -> u32;

    fn read_captured_val(&self) -> u32;

    fn write_ctrl_reg(&self, value:u16);

    // fn delay_millisecond(&self, delay: u32);

    // fn delay_millisecond_H(&self, delay: u32);

    // fn delay_microsecond(&self, delay: u32);

    // fn delay_microsecond_H(&self, delay: u32);
}

pub struct TimerConfig{
    pub period: u32,
    pub prescalar : u16,
    pub mode: TimerMode,
    pub interrupt_en: bool,
    pub duty_cycle: u32,
    pub cnt_en: u32,
    pub capture_val: u16,
    pub output_en: u16,
}

pub struct GPTimer<T: TimerOps> {
    timer: T,
}

impl<T: TimerOps> GPTimer<T> {
    pub fn new(timer: T) -> Self {
        GPTimer { timer }
    }

    pub fn initialize(&self, config: TimerConfig) {
        self.timer.set_period(config.period);
        self.timer.set_prescalar(config.prescalar);
        if config.cnt_en == GPT_CONTIN_COUNT_EN{
            self.timer.write_ctrl_reg(CONTIN_CNT_EN);
        }
        if config.output_en == GPT_OUTPUT_EN{
            self.timer.write_ctrl_reg(GPT_OUTPUT_EN);
        }
        match config.mode{
            TimerMode::Pwm => {
                self.timer.write_ctrl_reg(PWM_FALL_INTR_EN | PWM_RISE_INTR_EN);
                if config.duty_cycle < 100 && config.duty_cycle > 0{
                    self.timer.set_dutycycle(config.duty_cycle, config.period);
                }else{
                    // log error: invalid duty cycle
                    todo!()
                }
            }
            TimerMode::DownCounter => {
                let mut value: u16 = 0;
                if config.interrupt_en == GPT_INTR_EN{
                    value |= CNTR_UFLOW_INTR_EN;
                    self.timer.write_ctrl_reg(value);
                    
                }else{
                    value &= !CNTR_UFLOW_INTR_EN;
                    self.timer.write_ctrl_reg(value);
                }
            }
            TimerMode::UpCounter => {
                let mut value: u16 = 0;
                if config.interrupt_en == GPT_INTR_EN{
                    value |= CNTR_OFLOW_INTR_EN;
                    self.timer.write_ctrl_reg(value);
                    
                }else{
                    value &= !CNTR_OFLOW_INTR_EN;
                    self.timer.write_ctrl_reg(value);
                }
                
            },
            TimerMode::UpDownCounter => {
                let mut value: u16 = 0;
                if config.interrupt_en == GPT_INTR_EN{
                    value |= CNTR_OFLOW_INTR_EN | CNTR_UFLOW_INTR_EN;
                    self.timer.write_ctrl_reg(value);
                    
                }else{
                    value &= !(CNTR_OFLOW_INTR_EN | CNTR_UFLOW_INTR_EN);
                    self.timer.write_ctrl_reg(value);
                }
            },
            _ => {
                // Error : invalid mode passed
            }
        }

        self.timer.write_ctrl_reg(GPT_EN | gpt_mode(config.mode as u16) | capture_ip(config.capture_val) | COUNT_RESET);

    }

    pub fn reset(&self){
        self.timer.write_ctrl_reg(1<<5);
    }

    pub fn delay_millisecond(&self, mut config: TimerConfig, delay: u32){
        let prescalar: u16 = (CLOCK_FREQUENCY_BASE / 1000000) as u16 ;
        let period = delay * 1000;
        config.period = period;
        config.prescalar = prescalar;

        self.initialize(config);

        let mut counter_val: Volatile<u32> = Volatile::new(self.timer.read_captured_val());

        loop {
            if counter_val.read() == 0{
                break;
            }else{
                counter_val.write(self.timer.read_captured_val());
            }
        }


    }
}

impl TimerOps for Gptimer0 {
    fn reset(&self) {
        self.ctrl().reset();
    }

    fn write_ctrl_reg(&self, value: u16){
        self.ctrl().write(|w| unsafe { w.bits(value) });
    }

    fn enable(&self) {
        self.ctrl().write(|w| w.ctrl_en().set_bit());
    }

    fn set_period(&self, period: u32) {
        self.period().write(|w| unsafe { w.bits(period) });
    }

    fn set_dutycycle(&self, duty_cycle: u32, period: u32) {
        let mut dut_cycle = 0;
        dut_cycle = (duty_cycle * period) / 100;
        self.duty_cycle().write(|w| unsafe { w.bits(dut_cycle) });
    }

    fn set_prescalar(&self, prescalar: u16) {
        self.clock_ctrl().write(|w| unsafe { w.clk_prescalar().bits(prescalar) });
    }

    fn read_counter_value(&self) -> u32 {
        self.count().read().bits()
    }

    fn read_captured_val(&self) -> u32 {
        self.capture_inp().read().bits()
    }

    fn read_reptdcount(&self) -> u32 {
        self.rptd_count().read().bits()
    }

    fn update_enable(&self) {
        todo!()
    }
}