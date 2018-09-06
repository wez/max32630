#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Nano Oscillator Counter Read Register"]
    pub nano_cntr: NANO_CNTR,
    #[doc = "0x04 - RTC Clock Control Settings"]
    pub clk_ctrl: CLK_CTRL,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - RTC Oscillator Control"]
    pub osc_ctrl: OSC_CTRL,
}
#[doc = "Nano Oscillator Counter Read Register"]
pub struct NANO_CNTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Nano Oscillator Counter Read Register"]
pub mod nano_cntr;
#[doc = "RTC Clock Control Settings"]
pub struct CLK_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Clock Control Settings"]
pub mod clk_ctrl;
#[doc = "RTC Oscillator Control"]
pub struct OSC_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Oscillator Control"]
pub mod osc_ctrl;
