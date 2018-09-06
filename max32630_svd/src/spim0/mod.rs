#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI Master Configuration Register"]
    pub mstr_cfg: MSTR_CFG,
    #[doc = "0x04 - Polarity Control for SS and SR Signals"]
    pub ss_sr_polarity: SS_SR_POLARITY,
    #[doc = "0x08 - SPI Master General Control Register"]
    pub gen_ctrl: GEN_CTRL,
    #[doc = "0x0c - SPI Master FIFO Control Register"]
    pub fifo_ctrl: FIFO_CTRL,
    #[doc = "0x10 - SPI Master Special Mode Controls"]
    pub spcl_ctrl: SPCL_CTRL,
    #[doc = "0x14 - SPI Master Interrupt Flags"]
    pub intfl: INTFL,
    #[doc = "0x18 - SPI Master Interrupt Enable/Disable Settings"]
    pub inten: INTEN,
}
#[doc = "SPI Master Configuration Register"]
pub struct MSTR_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master Configuration Register"]
pub mod mstr_cfg;
#[doc = "Polarity Control for SS and SR Signals"]
pub struct SS_SR_POLARITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Polarity Control for SS and SR Signals"]
pub mod ss_sr_polarity;
#[doc = "SPI Master General Control Register"]
pub struct GEN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master General Control Register"]
pub mod gen_ctrl;
#[doc = "SPI Master FIFO Control Register"]
pub struct FIFO_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master FIFO Control Register"]
pub mod fifo_ctrl;
#[doc = "SPI Master Special Mode Controls"]
pub struct SPCL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master Special Mode Controls"]
pub mod spcl_ctrl;
#[doc = "SPI Master Interrupt Flags"]
pub struct INTFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master Interrupt Flags"]
pub mod intfl;
#[doc = "SPI Master Interrupt Enable/Disable Settings"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master Interrupt Enable/Disable Settings"]
pub mod inten;
