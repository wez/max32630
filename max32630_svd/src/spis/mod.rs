#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Slave General Control Register"]
    pub gen_ctrl: GEN_CTRL,
    #[doc = "0x04 - SPI Master FIFO Control Register"]
    pub fifo_ctrl: FIFO_CTRL,
    #[doc = "0x08 - SPI Slave FIFO Status Information"]
    pub fifo_stat: FIFO_STAT,
    #[doc = "0x0c - SPI Slave Interrupt Flags"]
    pub intfl: INTFL,
    #[doc = "0x10 - SPI Slave Interrupt Enable/Disable Settings"]
    pub inten: INTEN,
}
#[doc = "SPI Slave General Control Register"]
pub struct GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Slave General Control Register"]
pub mod gen_ctrl;
#[doc = "SPI Master FIFO Control Register"]
pub struct FIFO_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master FIFO Control Register"]
pub mod fifo_ctrl;
#[doc = "SPI Slave FIFO Status Information"]
pub struct FIFO_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Slave FIFO Status Information"]
pub mod fifo_stat;
#[doc = "SPI Slave Interrupt Flags"]
pub struct INTFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Slave Interrupt Flags"]
pub mod intfl;
#[doc = "SPI Slave Interrupt Enable/Disable Settings"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Slave Interrupt Enable/Disable Settings"]
pub mod inten;
