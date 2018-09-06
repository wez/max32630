#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC-16/CRC-32 Reseed Controls"]
    pub reseed: RESEED,
    #[doc = "0x04 - Reseed Value for CRC-16 Calculations"]
    pub seed16: SEED16,
    #[doc = "0x08 - Reseed Value for CRC-32 Calculations"]
    pub seed32: SEED32,
}
#[doc = "CRC-16/CRC-32 Reseed Controls"]
pub struct RESEED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CRC-16/CRC-32 Reseed Controls"]
pub mod reseed;
#[doc = "Reseed Value for CRC-16 Calculations"]
pub struct SEED16 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reseed Value for CRC-16 Calculations"]
pub mod seed16;
#[doc = "Reseed Value for CRC-32 Calculations"]
pub struct SEED32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reseed Value for CRC-32 Calculations"]
pub mod seed32;
