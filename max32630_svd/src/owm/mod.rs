#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 1-Wire Master Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - 1-Wire Master Clock Divisor"]
    pub clk_div_1us: CLK_DIV_1US,
    #[doc = "0x08 - 1-Wire Master Control/Status"]
    pub ctrl_stat: CTRL_STAT,
    #[doc = "0x0c - 1-Wire Master Data Buffer"]
    pub data: DATA,
    #[doc = "0x10 - 1-Wire Master Interrupt Flags"]
    pub intfl: INTFL,
    #[doc = "0x14 - 1-Wire Master Interrupt Enables"]
    pub inten: INTEN,
}
#[doc = "1-Wire Master Configuration"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1-Wire Master Configuration"]
pub mod cfg;
#[doc = "1-Wire Master Clock Divisor"]
pub struct CLK_DIV_1US {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1-Wire Master Clock Divisor"]
pub mod clk_div_1us;
#[doc = "1-Wire Master Control/Status"]
pub struct CTRL_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1-Wire Master Control/Status"]
pub mod ctrl_stat;
#[doc = "1-Wire Master Data Buffer"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1-Wire Master Data Buffer"]
pub mod data;
#[doc = "1-Wire Master Interrupt Flags"]
pub struct INTFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1-Wire Master Interrupt Flags"]
pub mod intfl;
#[doc = "1-Wire Master Interrupt Enables"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1-Wire Master Interrupt Enables"]
pub mod inten;
