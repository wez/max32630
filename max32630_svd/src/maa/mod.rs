#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAA Control, Configuration and Status"]
    pub ctrl: CTRL,
    #[doc = "0x04 - MAA Word (Operand) Size, Big/Little Endian Mode Select"]
    pub maws: MAWS,
}
#[doc = "MAA Control, Configuration and Status"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAA Control, Configuration and Status"]
pub mod ctrl;
#[doc = "MAA Word (Operand) Size, Big/Little Endian Mode Select"]
pub struct MAWS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAA Word (Operand) Size, Big/Little Endian Mode Select"]
pub mod maws;
