#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starting Descriptor Address"]
    pub dscadr: DSCADR,
    #[doc = "0x04 - Channel Configuration"]
    pub cfg: CFG,
    #[doc = "0x08 - Channel Loop Counters"]
    pub loop_: LOOP,
    #[doc = "0x0c - Current Descriptor DWORD 0 (OP)"]
    pub op: OP,
    #[doc = "0x10 - Current Descriptor DWORD 1"]
    pub dsc1: DSC1,
    #[doc = "0x14 - Current Descriptor DWORD 2"]
    pub dsc2: DSC2,
    #[doc = "0x18 - Current Descriptor DWORD 3"]
    pub dsc3: DSC3,
    #[doc = "0x1c - Current Descriptor DWORD 4"]
    pub dsc4: DSC4,
}
#[doc = "Starting Descriptor Address"]
pub struct DSCADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Starting Descriptor Address"]
pub mod dscadr;
#[doc = "Channel Configuration"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Configuration"]
pub mod cfg;
#[doc = "Channel Loop Counters"]
pub struct LOOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Loop Counters"]
pub mod loop_;
#[doc = "Current Descriptor DWORD 0 (OP)"]
pub struct OP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Descriptor DWORD 0 (OP)"]
pub mod op;
#[doc = "Current Descriptor DWORD 1"]
pub struct DSC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Descriptor DWORD 1"]
pub mod dsc1;
#[doc = "Current Descriptor DWORD 2"]
pub struct DSC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Descriptor DWORD 2"]
pub mod dsc2;
#[doc = "Current Descriptor DWORD 3"]
pub struct DSC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Descriptor DWORD 3"]
pub mod dsc3;
#[doc = "Current Descriptor DWORD 4"]
pub struct DSC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Current Descriptor DWORD 4"]
pub mod dsc4;
