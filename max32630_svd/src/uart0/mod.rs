#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - UART Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - UART Baud Control Register"]
    pub baud: BAUD,
    #[doc = "0x08 - UART TX Fifo Control Register"]
    pub tx_fifo_ctrl: TX_FIFO_CTRL,
    #[doc = "0x0c - UART RX Fifo Control Register"]
    pub rx_fifo_ctrl: RX_FIFO_CTRL,
    #[doc = "0x10 - UART Multidrop Control Register"]
    pub md_ctrl: MD_CTRL,
    #[doc = "0x14 - UART Interrupt Flags"]
    pub intfl: INTFL,
    #[doc = "0x18 - UART Interrupt Enable/Disable Controls"]
    pub inten: INTEN,
}
#[doc = "UART Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Control Register"]
pub mod ctrl;
#[doc = "UART Baud Control Register"]
pub struct BAUD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Baud Control Register"]
pub mod baud;
#[doc = "UART TX Fifo Control Register"]
pub struct TX_FIFO_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART TX Fifo Control Register"]
pub mod tx_fifo_ctrl;
#[doc = "UART RX Fifo Control Register"]
pub struct RX_FIFO_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART RX Fifo Control Register"]
pub mod rx_fifo_ctrl;
#[doc = "UART Multidrop Control Register"]
pub struct MD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Multidrop Control Register"]
pub mod md_ctrl;
#[doc = "UART Interrupt Flags"]
pub struct INTFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Interrupt Flags"]
pub mod intfl;
#[doc = "UART Interrupt Enable/Disable Controls"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART Interrupt Enable/Disable Controls"]
pub mod inten;
