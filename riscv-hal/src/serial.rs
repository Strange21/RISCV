use embedded_hal::serial::{Read, Write};
use nb::block;
use riscv_pac::Uart0;
use riscv_pac::uart0;

pub struct Serial {
    uart: Uart0,
}

#[derive(Debug)]
pub enum Error {
    Framing,
    Parity,
    Break,
    Overrun,
}

impl Serial {
    pub fn new(uart: Uart0) -> Self {
        Serial { uart }
    }

    pub fn init(&mut self, baud_rate: u16) {
        // Configure for 8N1 operation (8 data bits, no parity, 1 stop bit)
        self.uart.init(baud_rate, 0b00, 0b00, 0b00);
    }
}

impl Write<u8> for Serial {
    type Error = void::Void;

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        if self.uart.registers.status_reg.read().status_tx_full().bit_is_set() {
            Err(nb::Error::WouldBlock)
        } else {
            self.uart.registers.tx_reg.write(|w| unsafe { w.bits(byte as u32) });
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        if self.uart.registers.status_reg.read().status_tx_empty().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl Read<u8> for Serial {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let status = self.uart.registers.status_reg.read();
        
        if status.status_frame_err().bit_is_set() {
            return Err(nb::Error::Other(Error::Framing));
        }
        if status.status_parity_err().bit_is_set() {
            return Err(nb::Error::Other(Error::Parity));
        }
        if status.status_break_err().bit_is_set() {
            return Err(nb::Error::Other(Error::Break));
        }
        if status.status_overrun_err().bit_is_set() {
            return Err(nb::Error::Other(Error::Overrun));
        }

        if status.status_rx_not_empty().bit_is_set() {
            Ok(self.uart.registers.rx_reg.read().bits() as u8)
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
