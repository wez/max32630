#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Reset Control and Status"]
    pub pwr_rst_ctrl: PWR_RST_CTRL,
    #[doc = "0x04 - Interrupt Flags"]
    pub intfl: INTFL,
    #[doc = "0x08 - Interrupt Enable/Disable Controls"]
    pub inten: INTEN,
    #[doc = "0x0c - SVM Event Status Flags (read-only)"]
    pub svm_events: SVM_EVENTS,
    #[doc = "0x10 - Wake-Up Detect Control"]
    pub wud_ctrl: WUD_CTRL,
    #[doc = "0x14 - WUD Pulse To Mode Bit 0"]
    pub wud_pulse0: WUD_PULSE0,
    #[doc = "0x18 - WUD Pulse To Mode Bit 1"]
    pub wud_pulse1: WUD_PULSE1,
    #[doc = "0x1c - Wake-up Detect Status for P0/P1/P2/P3"]
    pub wud_seen0: WUD_SEEN0,
    #[doc = "0x20 - Wake-up Detect Status for P4/P5/P6/P7"]
    pub wud_seen1: WUD_SEEN1,
    _reserved9: [u8; 16usize],
    #[doc = "0x34 - SRAM Margin Adjustment"]
    pub margin_ctrl: MARGIN_CTRL,
    #[doc = "0x38 - Die Type ID Register"]
    pub die_type: DIE_TYPE,
    #[doc = "0x3c - Base Part Number"]
    pub base_part_num: BASE_PART_NUM,
    #[doc = "0x40 - Mask ID Register 0"]
    pub mask_id0: MASK_ID0,
    #[doc = "0x44 - Mask ID Register 1"]
    pub mask_id1: MASK_ID1,
    #[doc = "0x48 - Peripheral Reset Control Register"]
    pub peripheral_reset: PERIPHERAL_RESET,
}
#[doc = "Power Reset Control and Status"]
pub struct PWR_RST_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Reset Control and Status"]
pub mod pwr_rst_ctrl;
#[doc = "Interrupt Flags"]
pub struct INTFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Flags"]
pub mod intfl;
#[doc = "Interrupt Enable/Disable Controls"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable/Disable Controls"]
pub mod inten;
#[doc = "SVM Event Status Flags (read-only)"]
pub struct SVM_EVENTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SVM Event Status Flags (read-only)"]
pub mod svm_events;
#[doc = "Wake-Up Detect Control"]
pub struct WUD_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-Up Detect Control"]
pub mod wud_ctrl;
#[doc = "WUD Pulse To Mode Bit 0"]
pub struct WUD_PULSE0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WUD Pulse To Mode Bit 0"]
pub mod wud_pulse0;
#[doc = "WUD Pulse To Mode Bit 1"]
pub struct WUD_PULSE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WUD Pulse To Mode Bit 1"]
pub mod wud_pulse1;
#[doc = "Wake-up Detect Status for P0/P1/P2/P3"]
pub struct WUD_SEEN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-up Detect Status for P0/P1/P2/P3"]
pub mod wud_seen0;
#[doc = "Wake-up Detect Status for P4/P5/P6/P7"]
pub struct WUD_SEEN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake-up Detect Status for P4/P5/P6/P7"]
pub mod wud_seen1;
#[doc = "SRAM Margin Adjustment"]
pub struct MARGIN_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SRAM Margin Adjustment"]
pub mod margin_ctrl;
#[doc = "Die Type ID Register"]
pub struct DIE_TYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Die Type ID Register"]
pub mod die_type;
#[doc = "Base Part Number"]
pub struct BASE_PART_NUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Base Part Number"]
pub mod base_part_num;
#[doc = "Mask ID Register 0"]
pub struct MASK_ID0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask ID Register 0"]
pub mod mask_id0;
#[doc = "Mask ID Register 1"]
pub struct MASK_ID1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask ID Register 1"]
pub mod mask_id1;
#[doc = "Peripheral Reset Control Register"]
pub struct PERIPHERAL_RESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Reset Control Register"]
pub mod peripheral_reset;
