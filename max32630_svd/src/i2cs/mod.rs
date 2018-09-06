#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Slave Clock Divisor Control"]
    pub clk_div: CLK_DIV,
    #[doc = "0x04 - I2C Slave Device ID Register"]
    pub dev_id: DEV_ID,
    #[doc = "0x08 - I2CS Interrupt Flags"]
    pub intfl: INTFL,
    #[doc = "0x0c - I2CS Interrupt Enable/Disable Controls"]
    pub inten: INTEN,
    #[doc = "0x10 - I2CS Data Byte"]
    pub data_byte: DATA_BYTE,
}
#[doc = "I2C Slave Clock Divisor Control"]
pub struct CLK_DIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Clock Divisor Control"]
pub mod clk_div;
#[doc = "I2C Slave Device ID Register"]
pub struct DEV_ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave Device ID Register"]
pub mod dev_id;
#[doc = "I2CS Interrupt Flags"]
pub struct INTFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2CS Interrupt Flags"]
pub mod intfl;
#[doc = "I2CS Interrupt Enable/Disable Controls"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2CS Interrupt Enable/Disable Controls"]
pub mod inten;
#[doc = "I2CS Data Byte"]
pub struct DATA_BYTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2CS Data Byte"]
pub mod data_byte;
