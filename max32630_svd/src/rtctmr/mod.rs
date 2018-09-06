#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Timer Control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - RTC Timer Count Value"]
    pub timer: TIMER,
    #[doc = "0x08 - RTC Time of Day Alarm 0 Compare Register"]
    pub comp0: COMP0,
    #[doc = "0x0c - RTC Time of Day Alarm 1 Compare Register"]
    pub comp1: COMP1,
    #[doc = "0x10 - CPU Interrupt and RTC Domain Flags"]
    pub flags: FLAGS,
    #[doc = "0x14 - RTC Timer Alarm Snooze Value"]
    pub snz_value: SNZ_VALUE,
    #[doc = "0x18 - Interrupt Enable Controls"]
    pub inten: INTEN,
    #[doc = "0x1c - RTC Timer Prescale Setting"]
    pub prescale: PRESCALE,
    _reserved8: [u8; 4usize],
    #[doc = "0x24 - RTC Timer Prescale Compare Mask"]
    pub prescale_mask: PRESCALE_MASK,
    #[doc = "0x28 - RTC Timer Trim Controls"]
    pub trim_ctrl: TRIM_CTRL,
    #[doc = "0x2c - RTC Timer Trim Adjustment Interval"]
    pub trim_value: TRIM_VALUE,
}
#[doc = "RTC Timer Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Control"]
pub mod ctrl;
#[doc = "RTC Timer Count Value"]
pub struct TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Count Value"]
pub mod timer;
#[doc = "RTC Time of Day Alarm 0 Compare Register"]
pub struct COMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Time of Day Alarm 0 Compare Register"]
pub mod comp0;
#[doc = "RTC Time of Day Alarm 1 Compare Register"]
pub struct COMP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Time of Day Alarm 1 Compare Register"]
pub mod comp1;
#[doc = "CPU Interrupt and RTC Domain Flags"]
pub struct FLAGS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CPU Interrupt and RTC Domain Flags"]
pub mod flags;
#[doc = "RTC Timer Alarm Snooze Value"]
pub struct SNZ_VALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Alarm Snooze Value"]
pub mod snz_value;
#[doc = "Interrupt Enable Controls"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Controls"]
pub mod inten;
#[doc = "RTC Timer Prescale Setting"]
pub struct PRESCALE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Prescale Setting"]
pub mod prescale;
#[doc = "RTC Timer Prescale Compare Mask"]
pub struct PRESCALE_MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Prescale Compare Mask"]
pub mod prescale_mask;
#[doc = "RTC Timer Trim Controls"]
pub struct TRIM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Trim Controls"]
pub mod trim_ctrl;
#[doc = "RTC Timer Trim Adjustment Interval"]
pub struct TRIM_VALUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Timer Trim Adjustment Interval"]
pub mod trim_value;
