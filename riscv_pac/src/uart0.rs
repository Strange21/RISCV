#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    baud_reg: BaudReg,
    _reserved1: [u8; 0x02],
    tx_reg: TxReg,
    rx_reg: RxReg,
    status_reg: StatusReg,
    _reserved4: [u8; 0x02],
    delay_reg: DelayReg,
    _reserved5: [u8; 0x02],
    ctrl: Ctrl,
    _reserved6: [u8; 0x02],
    intr_en: IntrEn,
    _reserved7: [u8; 0x02],
    rx_threshold: RxThreshold,
}
impl RegisterBlock {
    #[doc = "0x00 - Baud register"]
    #[inline(always)]
    pub const fn baud_reg(&self) -> &BaudReg {
        &self.baud_reg
    }
    #[doc = "0x04 - TX data register"]
    #[inline(always)]
    pub const fn tx_reg(&self) -> &TxReg {
        &self.tx_reg
    }
    #[doc = "0x08 - RX data register"]
    #[inline(always)]
    pub const fn rx_reg(&self) -> &RxReg {
        &self.rx_reg
    }
    #[doc = "0x0c - UART Status register"]
    #[inline(always)]
    pub const fn status_reg(&self) -> &StatusReg {
        &self.status_reg
    }
    #[doc = "0x10 - Stores the delay to have before Tranmission"]
    #[inline(always)]
    pub const fn delay_reg(&self) -> &DelayReg {
        &self.delay_reg
    }
    #[doc = "0x14 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x18 - Interrupts enable register"]
    #[inline(always)]
    pub const fn intr_en(&self) -> &IntrEn {
        &self.intr_en
    }
    #[doc = "0x1c - The threshold value to indicate the RX FIFO almost full interrupt"]
    #[inline(always)]
    pub const fn rx_threshold(&self) -> &RxThreshold {
        &self.rx_threshold
    }
}
#[doc = "BAUD_REG (rw) register accessor: Baud register\n\nYou can [`read`](crate::Reg::read) this register and get [`baud_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud_reg`]
module"]
#[doc(alias = "BAUD_REG")]
pub type BaudReg = crate::Reg<baud_reg::BaudRegSpec>;
#[doc = "Baud register"]
pub mod baud_reg;
#[doc = "TX_REG (rw) register accessor: TX data register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_reg`]
module"]
#[doc(alias = "TX_REG")]
pub type TxReg = crate::Reg<tx_reg::TxRegSpec>;
#[doc = "TX data register"]
pub mod tx_reg;
#[doc = "RX_REG (rw) register accessor: RX data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_reg`]
module"]
#[doc(alias = "RX_REG")]
pub type RxReg = crate::Reg<rx_reg::RxRegSpec>;
#[doc = "RX data register"]
pub mod rx_reg;
#[doc = "STATUS_REG (rw) register accessor: UART Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_reg`]
module"]
#[doc(alias = "STATUS_REG")]
pub type StatusReg = crate::Reg<status_reg::StatusRegSpec>;
#[doc = "UART Status register"]
pub mod status_reg;
#[doc = "DELAY_REG (rw) register accessor: Stores the delay to have before Tranmission\n\nYou can [`read`](crate::Reg::read) this register and get [`delay_reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay_reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delay_reg`]
module"]
#[doc(alias = "DELAY_REG")]
pub type DelayReg = crate::Reg<delay_reg::DelayRegSpec>;
#[doc = "Stores the delay to have before Tranmission"]
pub mod delay_reg;
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "INTR_EN (rw) register accessor: Interrupts enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en`]
module"]
#[doc(alias = "INTR_EN")]
pub type IntrEn = crate::Reg<intr_en::IntrEnSpec>;
#[doc = "Interrupts enable register"]
pub mod intr_en;
#[doc = "RX_THRESHOLD (rw) register accessor: The threshold value to indicate the RX FIFO almost full interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_threshold`]
module"]
#[doc(alias = "RX_THRESHOLD")]
pub type RxThreshold = crate::Reg<rx_threshold::RxThresholdSpec>;
#[doc = "The threshold value to indicate the RX FIFO almost full interrupt"]
pub mod rx_threshold;
