#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - TPU Secure Key Storage Register 0 (Cleared on Tamper Detect)"]
    pub sks0: SKS0,
    #[doc = "0x14 - TPU Secure Key Storage Register 1 (Cleared on Tamper Detect)"]
    pub sks1: SKS1,
    #[doc = "0x18 - TPU Secure Key Storage Register 2 (Cleared on Tamper Detect)"]
    pub sks2: SKS2,
    #[doc = "0x1c - TPU Secure Key Storage Register 3 (Cleared on Tamper Detect)"]
    pub sks3: SKS3,
}
#[doc = "TPU Secure Key Storage Register 0 (Cleared on Tamper Detect)"]
pub struct SKS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TPU Secure Key Storage Register 0 (Cleared on Tamper Detect)"]
pub mod sks0;
#[doc = "TPU Secure Key Storage Register 1 (Cleared on Tamper Detect)"]
pub struct SKS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TPU Secure Key Storage Register 1 (Cleared on Tamper Detect)"]
pub mod sks1;
#[doc = "TPU Secure Key Storage Register 2 (Cleared on Tamper Detect)"]
pub struct SKS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TPU Secure Key Storage Register 2 (Cleared on Tamper Detect)"]
pub mod sks2;
#[doc = "TPU Secure Key Storage Register 3 (Cleared on Tamper Detect)"]
pub struct SKS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TPU Secure Key Storage Register 3 (Cleared on Tamper Detect)"]
pub mod sks3;
