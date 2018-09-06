#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Operation Address"]
    pub faddr: FADDR,
    #[doc = "0x04 - Flash Clock Pulse Divisor"]
    pub fckdiv: FCKDIV,
    #[doc = "0x08 - Flash Control Register"]
    pub ctrl: CTRL,
    _reserved3: [u8; 24usize],
    #[doc = "0x24 - Flash Controller Interrupt Flags and Enable/Disable 0"]
    pub intr: INTR,
    _reserved4: [u8; 8usize],
    #[doc = "0x30 - Flash Operation Data Register"]
    pub fdata: FDATA,
    _reserved5: [u8; 28usize],
    #[doc = "0x50 - Flash Performance Settings"]
    pub perform: PERFORM,
    #[doc = "0x54 - Flash Read Cycle Config"]
    pub tacc: TACC,
    #[doc = "0x58 - Flash Write Cycle Config"]
    pub tprog: TPROG,
    _reserved8: [u8; 36usize],
    #[doc = "0x80 - Security Status Flags"]
    pub status: STATUS,
    _reserved9: [u8; 4usize],
    #[doc = "0x88 - Flash Controller Security Settings"]
    pub security: SECURITY,
    _reserved10: [u8; 16usize],
    #[doc = "0x9c - Status Flags for DSB Operations"]
    pub bypass: BYPASS,
    _reserved11: [u8; 96usize],
    #[doc = "0x100 - Used to set DSB Access code and Auto-Lock in info block"]
    pub user_option: USER_OPTION,
    _reserved12: [u8; 60usize],
    #[doc = "0x140 - Flash Control Register 2"]
    pub ctrl2: CTRL2,
    #[doc = "0x144 - Interrupt Flags Register 1"]
    pub intfl1: INTFL1,
    #[doc = "0x148 - Interrupt Enable/Disable Register 1"]
    pub inten1: INTEN1,
    _reserved15: [u8; 36usize],
    #[doc = "0x170 - Bootloader Control Register"]
    pub bl_ctrl: BL_CTRL,
    #[doc = "0x174 - Cycle Count Tweak Register"]
    pub twk_cycl_cnt: TWK_CYCL_CNT,
    #[doc = "0x178 - PDM33 Register"]
    pub pdm33: PDM33,
    #[doc = "0x17c - Sleep Mode Register"]
    pub slm: SLM,
    _reserved19: [u8; 128usize],
    #[doc = "0x200 - Disable Flash Page Exec/Read Register 0"]
    pub disable_xr0: DISABLE_XR0,
    #[doc = "0x204 - Disable Flash Page Exec/Read Register 1"]
    pub disable_xr1: DISABLE_XR1,
    #[doc = "0x208 - Disable Flash Page Exec/Read Register 2"]
    pub disable_xr2: DISABLE_XR2,
    #[doc = "0x20c - Disable Flash Page Exec/Read Register 3"]
    pub disable_xr3: DISABLE_XR3,
    #[doc = "0x210 - Disable Flash Page Exec/Read Register 4"]
    pub disable_xr4: DISABLE_XR4,
    #[doc = "0x214 - Disable Flash Page Exec/Read Register 5"]
    pub disable_xr5: DISABLE_XR5,
    #[doc = "0x218 - Disable Flash Page Exec/Read Register 6"]
    pub disable_xr6: DISABLE_XR6,
    #[doc = "0x21c - Disable Flash Page Exec/Read Register 7"]
    pub disable_xr7: DISABLE_XR7,
    _reserved27: [u8; 224usize],
    #[doc = "0x300 - Disable Flash Page Write/Erase Register 0"]
    pub disable_we0: DISABLE_WE0,
    #[doc = "0x304 - Disable Flash Page Write/Erase Register 1"]
    pub disable_we1: DISABLE_WE1,
    #[doc = "0x308 - Disable Flash Page Write/Erase Register 2"]
    pub disable_we2: DISABLE_WE2,
    #[doc = "0x30c - Disable Flash Page Write/Erase Register 3"]
    pub disable_we3: DISABLE_WE3,
    #[doc = "0x310 - Disable Flash Page Write/Erase Register 4"]
    pub disable_we4: DISABLE_WE4,
    #[doc = "0x314 - Disable Flash Page Write/Erase Register 5"]
    pub disable_we5: DISABLE_WE5,
    #[doc = "0x318 - Disable Flash Page Write/Erase Register 6"]
    pub disable_we6: DISABLE_WE6,
    #[doc = "0x31c - Disable Flash Page Write/Erase Register 7"]
    pub disable_we7: DISABLE_WE7,
}
#[doc = "Flash Operation Address"]
pub struct FADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Operation Address"]
pub mod faddr;
#[doc = "Flash Clock Pulse Divisor"]
pub struct FCKDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Clock Pulse Divisor"]
pub mod fckdiv;
#[doc = "Flash Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Control Register"]
pub mod ctrl;
#[doc = "Flash Controller Interrupt Flags and Enable/Disable 0"]
pub struct INTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller Interrupt Flags and Enable/Disable 0"]
pub mod intr;
#[doc = "Flash Operation Data Register"]
pub struct FDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Operation Data Register"]
pub mod fdata;
#[doc = "Flash Performance Settings"]
pub struct PERFORM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Performance Settings"]
pub mod perform;
#[doc = "Flash Read Cycle Config"]
pub struct TACC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Read Cycle Config"]
pub mod tacc;
#[doc = "Flash Write Cycle Config"]
pub struct TPROG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Write Cycle Config"]
pub mod tprog;
#[doc = "Security Status Flags"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Security Status Flags"]
pub mod status;
#[doc = "Flash Controller Security Settings"]
pub struct SECURITY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Controller Security Settings"]
pub mod security;
#[doc = "Status Flags for DSB Operations"]
pub struct BYPASS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Flags for DSB Operations"]
pub mod bypass;
#[doc = "Used to set DSB Access code and Auto-Lock in info block"]
pub struct USER_OPTION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Used to set DSB Access code and Auto-Lock in info block"]
pub mod user_option;
#[doc = "Flash Control Register 2"]
pub struct CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flash Control Register 2"]
pub mod ctrl2;
#[doc = "Interrupt Flags Register 1"]
pub struct INTFL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flags Register 1"]
pub mod intfl1;
#[doc = "Interrupt Enable/Disable Register 1"]
pub struct INTEN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable/Disable Register 1"]
pub mod inten1;
#[doc = "Bootloader Control Register"]
pub struct BL_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bootloader Control Register"]
pub mod bl_ctrl;
#[doc = "Cycle Count Tweak Register"]
pub struct TWK_CYCL_CNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Cycle Count Tweak Register"]
pub mod twk_cycl_cnt;
#[doc = "PDM33 Register"]
pub struct PDM33 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PDM33 Register"]
pub mod pdm33;
#[doc = "Sleep Mode Register"]
pub struct SLM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sleep Mode Register"]
pub mod slm;
#[doc = "Disable Flash Page Exec/Read Register 0"]
pub struct DISABLE_XR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Exec/Read Register 0"]
pub mod disable_xr0;
#[doc = "Disable Flash Page Exec/Read Register 1"]
pub struct DISABLE_XR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Exec/Read Register 1"]
pub mod disable_xr1;
#[doc = "Disable Flash Page Exec/Read Register 2"]
pub struct DISABLE_XR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Exec/Read Register 2"]
pub mod disable_xr2;
#[doc = "Disable Flash Page Exec/Read Register 3"]
pub struct DISABLE_XR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Exec/Read Register 3"]
pub mod disable_xr3;
#[doc = "Disable Flash Page Exec/Read Register 4"]
pub struct DISABLE_XR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Exec/Read Register 4"]
pub mod disable_xr4;
#[doc = "Disable Flash Page Exec/Read Register 5"]
pub struct DISABLE_XR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Exec/Read Register 5"]
pub mod disable_xr5;
#[doc = "Disable Flash Page Exec/Read Register 6"]
pub struct DISABLE_XR6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Exec/Read Register 6"]
pub mod disable_xr6;
#[doc = "Disable Flash Page Exec/Read Register 7"]
pub struct DISABLE_XR7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Exec/Read Register 7"]
pub mod disable_xr7;
#[doc = "Disable Flash Page Write/Erase Register 0"]
pub struct DISABLE_WE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Write/Erase Register 0"]
pub mod disable_we0;
#[doc = "Disable Flash Page Write/Erase Register 1"]
pub struct DISABLE_WE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Write/Erase Register 1"]
pub mod disable_we1;
#[doc = "Disable Flash Page Write/Erase Register 2"]
pub struct DISABLE_WE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Write/Erase Register 2"]
pub mod disable_we2;
#[doc = "Disable Flash Page Write/Erase Register 3"]
pub struct DISABLE_WE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Write/Erase Register 3"]
pub mod disable_we3;
#[doc = "Disable Flash Page Write/Erase Register 4"]
pub struct DISABLE_WE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Write/Erase Register 4"]
pub mod disable_we4;
#[doc = "Disable Flash Page Write/Erase Register 5"]
pub struct DISABLE_WE5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Write/Erase Register 5"]
pub mod disable_we5;
#[doc = "Disable Flash Page Write/Erase Register 6"]
pub struct DISABLE_WE6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Write/Erase Register 6"]
pub mod disable_we6;
#[doc = "Disable Flash Page Write/Erase Register 7"]
pub struct DISABLE_WE7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Disable Flash Page Write/Erase Register 7"]
pub mod disable_we7;
