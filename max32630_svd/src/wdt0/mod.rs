#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT0 - Watchdog Timer Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - WDT0 - Watchdog Clear Register (Feed Dog)"]
    pub clear: CLEAR,
    #[doc = "0x08 - WDT0 - Watchdog Interrupt and Reset Flags"]
    pub flags: FLAGS,
    #[doc = "0x0c - WDT0 - Interrupt/Reset Enable/Disable Controls"]
    pub enable: ENABLE,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - WDT0 - Register Setting Lock for WDT0_CTRL"]
    pub lock_ctrl: LOCK_CTRL,
}
#[doc = "WDT0 - Watchdog Timer Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT0 - Watchdog Timer Control Register"]
pub mod ctrl;
#[doc = "WDT0 - Watchdog Clear Register (Feed Dog)"]
pub struct CLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT0 - Watchdog Clear Register (Feed Dog)"]
pub mod clear;
#[doc = "WDT0 - Watchdog Interrupt and Reset Flags"]
pub struct FLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT0 - Watchdog Interrupt and Reset Flags"]
pub mod flags;
#[doc = "WDT0 - Interrupt/Reset Enable/Disable Controls"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT0 - Interrupt/Reset Enable/Disable Controls"]
pub mod enable;
#[doc = "WDT0 - Register Setting Lock for WDT0_CTRL"]
pub struct LOCK_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDT0 - Register Setting Lock for WDT0_CTRL"]
pub mod lock_ctrl;
