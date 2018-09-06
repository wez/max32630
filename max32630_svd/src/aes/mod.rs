#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES Control and Status"]
    pub ctrl: CTRL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - A write to this location triggers an erase of all AES memory locations."]
    pub erase_all: ERASE_ALL,
}
#[doc = "AES Control and Status"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AES Control and Status"]
pub mod ctrl;
#[doc = "A write to this location triggers an erase of all AES memory locations."]
pub struct ERASE_ALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "A write to this location triggers an erase of all AES memory locations."]
pub mod erase_all;
