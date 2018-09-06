#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Enable/Disable Controls for All Pulse Trains"]
    pub enable: ENABLE,
    #[doc = "0x04 - Global Resync (All Pulse Trains) Control"]
    pub resync: RESYNC,
    #[doc = "0x08 - Pulse Train Interrupt Flags"]
    pub intfl: INTFL,
    #[doc = "0x0c - Pulse Train Interrupt Enable/Disable"]
    pub inten: INTEN,
}
#[doc = "Global Enable/Disable Controls for All Pulse Trains"]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Enable/Disable Controls for All Pulse Trains"]
pub mod enable;
#[doc = "Global Resync (All Pulse Trains) Control"]
pub struct RESYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Global Resync (All Pulse Trains) Control"]
pub mod resync;
#[doc = "Pulse Train Interrupt Flags"]
pub struct INTFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Train Interrupt Flags"]
pub mod intfl;
#[doc = "Pulse Train Interrupt Enable/Disable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Train Interrupt Enable/Disable"]
pub mod inten;
