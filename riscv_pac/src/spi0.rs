#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    clk_ctrl: ClkCtrl,
    tx: Tx,
    rx: Rx,
    intr_en: IntrEn,
    fifo_status: FifoStatus,
    comm_status: CommStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI communication control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - SPI clock generation control register"]
    #[inline(always)]
    pub const fn clk_ctrl(&self) -> &ClkCtrl {
        &self.clk_ctrl
    }
    #[doc = "0x08 - holds the tx data. This register is written by the AXI write request and once written the data is transferred to TX FIFO"]
    #[inline(always)]
    pub const fn tx(&self) -> &Tx {
        &self.tx
    }
    #[doc = "0x0c - holds the tx data. This register is read by the AXI read request. The data is written from the RX FIFO"]
    #[inline(always)]
    pub const fn rx(&self) -> &Rx {
        &self.rx
    }
    #[doc = "0x10 - Spi interrupt enable register"]
    #[inline(always)]
    pub const fn intr_en(&self) -> &IntrEn {
        &self.intr_en
    }
    #[doc = "0x14 - Gives the status of TX/RX FIFO"]
    #[inline(always)]
    pub const fn fifo_status(&self) -> &FifoStatus {
        &self.fifo_status
    }
    #[doc = "0x18 - Status of SPI communication"]
    #[inline(always)]
    pub const fn comm_status(&self) -> &CommStatus {
        &self.comm_status
    }
}
#[doc = "CTRL (rw) register accessor: SPI communication control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "SPI communication control register"]
pub mod ctrl;
#[doc = "CLK_CTRL (rw) register accessor: SPI clock generation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ctrl`]
module"]
#[doc(alias = "CLK_CTRL")]
pub type ClkCtrl = crate::Reg<clk_ctrl::ClkCtrlSpec>;
#[doc = "SPI clock generation control register"]
pub mod clk_ctrl;
#[doc = "TX (rw) register accessor: holds the tx data. This register is written by the AXI write request and once written the data is transferred to TX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx`]
module"]
#[doc(alias = "TX")]
pub type Tx = crate::Reg<tx::TxSpec>;
#[doc = "holds the tx data. This register is written by the AXI write request and once written the data is transferred to TX FIFO"]
pub mod tx;
#[doc = "RX (rw) register accessor: holds the tx data. This register is read by the AXI read request. The data is written from the RX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx`]
module"]
#[doc(alias = "RX")]
pub type Rx = crate::Reg<rx::RxSpec>;
#[doc = "holds the tx data. This register is read by the AXI read request. The data is written from the RX FIFO"]
pub mod rx;
#[doc = "INTR_EN (rw) register accessor: Spi interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_en`]
module"]
#[doc(alias = "INTR_EN")]
pub type IntrEn = crate::Reg<intr_en::IntrEnSpec>;
#[doc = "Spi interrupt enable register"]
pub mod intr_en;
#[doc = "FIFO_STATUS (rw) register accessor: Gives the status of TX/RX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_status`]
module"]
#[doc(alias = "FIFO_STATUS")]
pub type FifoStatus = crate::Reg<fifo_status::FifoStatusSpec>;
#[doc = "Gives the status of TX/RX FIFO"]
pub mod fifo_status;
#[doc = "COMM_STATUS (rw) register accessor: Status of SPI communication\n\nYou can [`read`](crate::Reg::read) this register and get [`comm_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comm_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comm_status`]
module"]
#[doc(alias = "COMM_STATUS")]
pub type CommStatus = crate::Reg<comm_status::CommStatusSpec>;
#[doc = "Status of SPI communication"]
pub mod comm_status;
