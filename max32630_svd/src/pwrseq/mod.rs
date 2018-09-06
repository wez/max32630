#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Sequencer Control Register 0"]
    pub reg0: REG0,
    #[doc = "0x04 - Power Sequencer Control Register 1"]
    pub reg1: REG1,
    #[doc = "0x08 - Power Sequencer Control Register 2"]
    pub reg2: REG2,
    #[doc = "0x0c - Power Sequencer Control Register 3"]
    pub reg3: REG3,
    #[doc = "0x10 - Power Sequencer Control Register 4 (Internal Test Only)"]
    pub reg4: REG4,
    #[doc = "0x14 - Power Sequencer Control Register 5 (Trim 0)"]
    pub reg5: REG5,
    #[doc = "0x18 - Power Sequencer Control Register 6 (Trim 1)"]
    pub reg6: REG6,
    #[doc = "0x1c - Power Sequencer Control Register 7"]
    pub reg7: REG7,
    #[doc = "0x20 - Power Sequencer Flags"]
    pub flags: FLAGS,
    #[doc = "0x24 - Power Sequencer Flags Mask Register"]
    pub msk_flags: MSK_FLAGS,
}
#[doc = "Power Sequencer Control Register 0"]
pub struct REG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Sequencer Control Register 0"]
pub mod reg0;
#[doc = "Power Sequencer Control Register 1"]
pub struct REG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Sequencer Control Register 1"]
pub mod reg1;
#[doc = "Power Sequencer Control Register 2"]
pub struct REG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Sequencer Control Register 2"]
pub mod reg2;
#[doc = "Power Sequencer Control Register 3"]
pub struct REG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Sequencer Control Register 3"]
pub mod reg3;
#[doc = "Power Sequencer Control Register 4 (Internal Test Only)"]
pub struct REG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Sequencer Control Register 4 (Internal Test Only)"]
pub mod reg4;
#[doc = "Power Sequencer Control Register 5 (Trim 0)"]
pub struct REG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Sequencer Control Register 5 (Trim 0)"]
pub mod reg5;
#[doc = "Power Sequencer Control Register 6 (Trim 1)"]
pub struct REG6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Sequencer Control Register 6 (Trim 1)"]
pub mod reg6;
#[doc = "Power Sequencer Control Register 7"]
pub struct REG7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Sequencer Control Register 7"]
pub mod reg7;
#[doc = "Power Sequencer Flags"]
pub struct FLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Sequencer Flags"]
pub mod flags;
#[doc = "Power Sequencer Flags Mask Register"]
pub struct MSK_FLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Sequencer Flags Mask Register"]
pub mod msk_flags;
