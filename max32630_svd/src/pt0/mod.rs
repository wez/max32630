#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pulse Train Configuration"]
    pub rate_length: RATE_LENGTH,
    #[doc = "0x04 - Pulse Train Output Pattern"]
    pub train: TRAIN,
    #[doc = "0x08 - Pulse Train Loop Count"]
    pub loop_: LOOP,
}
#[doc = "Pulse Train Configuration"]
pub struct RATE_LENGTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Train Configuration"]
pub mod rate_length;
#[doc = "Pulse Train Output Pattern"]
pub struct TRAIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Train Output Pattern"]
pub mod train;
#[doc = "Pulse Train Loop Count"]
pub struct LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pulse Train Loop Count"]
pub mod loop_;
