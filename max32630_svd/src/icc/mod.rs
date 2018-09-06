#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device ID Register"]
    pub id: ID,
    #[doc = "0x04 - Memory Configuration Register"]
    pub mem_cfg: MEM_CFG,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - Control and Status"]
    pub ctrl_stat: CTRL_STAT,
    _reserved3: [u8; 1532usize],
    #[doc = "0x700 - Invalidate (Clear) Cache Control"]
    pub invdt_all: INVDT_ALL,
}
#[doc = "Device ID Register"]
pub struct ID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device ID Register"]
pub mod id;
#[doc = "Memory Configuration Register"]
pub struct MEM_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Memory Configuration Register"]
pub mod mem_cfg;
#[doc = "Control and Status"]
pub struct CTRL_STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control and Status"]
pub mod ctrl_stat;
#[doc = "Invalidate (Clear) Cache Control"]
pub struct INVDT_ALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Invalidate (Clear) Cache Control"]
pub mod invdt_all;
