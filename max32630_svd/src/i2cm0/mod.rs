#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Full Speed SCL Clock Settings"]
    pub fs_clk_div: FS_CLK_DIV,
    _reserved1: [u8; 8usize],
    #[doc = "0x0c - Timeout and Auto-Stop Settings"]
    pub timeout: TIMEOUT,
    #[doc = "0x10 - I2C Master Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x14 - I2C Master Transaction Start and Status Flags"]
    pub trans: TRANS,
    #[doc = "0x18 - Interrupt Flags"]
    pub intfl: INTFL,
    #[doc = "0x1c - Interrupt Enable/Disable Controls"]
    pub inten: INTEN,
    _reserved6: [u8; 8usize],
    #[doc = "0x28 - Bit-Bang Control Register"]
    pub bb: BB,
}
#[doc = "Full Speed SCL Clock Settings"]
pub struct FS_CLK_DIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Full Speed SCL Clock Settings"]
pub mod fs_clk_div;
#[doc = "Timeout and Auto-Stop Settings"]
pub struct TIMEOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timeout and Auto-Stop Settings"]
pub mod timeout;
#[doc = "I2C Master Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Control Register"]
pub mod ctrl;
#[doc = "I2C Master Transaction Start and Status Flags"]
pub struct TRANS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master Transaction Start and Status Flags"]
pub mod trans;
#[doc = "Interrupt Flags"]
pub struct INTFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flags"]
pub mod intfl;
#[doc = "Interrupt Enable/Disable Controls"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable/Disable Controls"]
pub mod inten;
#[doc = "Bit-Bang Control Register"]
pub struct BB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bit-Bang Control Register"]
pub mod bb;
