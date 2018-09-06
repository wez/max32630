#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - [32 bit] Current Count Value"]
    pub count32: COUNT32,
    #[doc = "0x08 - [32 bit] Terminal Count Setting"]
    pub term_cnt32: TERM_CNT32,
    #[doc = "0x0c - [32 bit] PWM Compare Setting or Capture/Measure Value"]
    pub pwm_cap32: PWM_CAP32,
    #[doc = "0x10 - [16 bit] Current Count Value, 16-bit Timer 0"]
    pub count16_0: COUNT16_0,
    #[doc = "0x14 - [16 bit] Terminal Count Setting, 16-bit Timer 0"]
    pub term_cnt16_0: TERM_CNT16_0,
    #[doc = "0x18 - [16 bit] Current Count Value, 16-bit Timer 1"]
    pub count16_1: COUNT16_1,
    #[doc = "0x1c - [16 bit] Terminal Count Setting, 16-bit Timer 1"]
    pub term_cnt16_1: TERM_CNT16_1,
    #[doc = "0x20 - Timer Module Interrupt Flags"]
    pub intfl: INTFL,
    #[doc = "0x24 - Timer Module Interrupt Enable/Disable Settings"]
    pub inten: INTEN,
}
#[doc = "Timer Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control Register"]
pub mod ctrl;
#[doc = "[32 bit] Current Count Value"]
pub struct COUNT32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "[32 bit] Current Count Value"]
pub mod count32;
#[doc = "[32 bit] Terminal Count Setting"]
pub struct TERM_CNT32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "[32 bit] Terminal Count Setting"]
pub mod term_cnt32;
#[doc = "[32 bit] PWM Compare Setting or Capture/Measure Value"]
pub struct PWM_CAP32 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "[32 bit] PWM Compare Setting or Capture/Measure Value"]
pub mod pwm_cap32;
#[doc = "[16 bit] Current Count Value, 16-bit Timer 0"]
pub struct COUNT16_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "[16 bit] Current Count Value, 16-bit Timer 0"]
pub mod count16_0;
#[doc = "[16 bit] Terminal Count Setting, 16-bit Timer 0"]
pub struct TERM_CNT16_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "[16 bit] Terminal Count Setting, 16-bit Timer 0"]
pub mod term_cnt16_0;
#[doc = "[16 bit] Current Count Value, 16-bit Timer 1"]
pub struct COUNT16_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "[16 bit] Current Count Value, 16-bit Timer 1"]
pub mod count16_1;
#[doc = "[16 bit] Terminal Count Setting, 16-bit Timer 1"]
pub struct TERM_CNT16_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "[16 bit] Terminal Count Setting, 16-bit Timer 1"]
pub mod term_cnt16_1;
#[doc = "Timer Module Interrupt Flags"]
pub struct INTFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Module Interrupt Flags"]
pub mod intfl;
#[doc = "Timer Module Interrupt Enable/Disable Settings"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Module Interrupt Enable/Disable Settings"]
pub mod inten;
