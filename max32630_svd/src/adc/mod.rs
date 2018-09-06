#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Control"]
    pub ctrl: CTRL,
    #[doc = "0x04 - ADC Status"]
    pub status: STATUS,
    #[doc = "0x08 - ADC Output Data"]
    pub data: DATA,
    #[doc = "0x0c - ADC Interrupt Control Register"]
    pub intr: INTR,
    #[doc = "0x10 - ADC Limit"]
    pub limit0: LIMIT0,
    #[doc = "0x14 - ADC Limit 1"]
    pub limit1: LIMIT1,
    #[doc = "0x18 - ADC Limit 2"]
    pub limit2: LIMIT2,
    #[doc = "0x1c - ADC Limit 3"]
    pub limit3: LIMIT3,
    #[doc = "0x20 - AFE Control Register"]
    pub afe_ctrl: AFE_CTRL,
    #[doc = "0x24 - RO Trim Calibration Register 0"]
    pub ro_cal0: RO_CAL0,
    #[doc = "0x28 - RO Trim Calibration Register 1"]
    pub ro_cal1: RO_CAL1,
    #[doc = "0x2c - RO Trim Calibration Register 2"]
    pub ro_cal2: RO_CAL2,
}
#[doc = "ADC Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Control"]
pub mod ctrl;
#[doc = "ADC Status"]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Status"]
pub mod status;
#[doc = "ADC Output Data"]
pub struct DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Output Data"]
pub mod data;
#[doc = "ADC Interrupt Control Register"]
pub struct INTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Interrupt Control Register"]
pub mod intr;
#[doc = "ADC Limit"]
pub struct LIMIT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Limit"]
pub mod limit0;
#[doc = "ADC Limit 1"]
pub struct LIMIT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Limit 1"]
pub mod limit1;
#[doc = "ADC Limit 2"]
pub struct LIMIT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Limit 2"]
pub mod limit2;
#[doc = "ADC Limit 3"]
pub struct LIMIT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC Limit 3"]
pub mod limit3;
#[doc = "AFE Control Register"]
pub struct AFE_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AFE Control Register"]
pub mod afe_ctrl;
#[doc = "RO Trim Calibration Register 0"]
pub struct RO_CAL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RO Trim Calibration Register 0"]
pub mod ro_cal0;
#[doc = "RO Trim Calibration Register 1"]
pub struct RO_CAL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RO Trim Calibration Register 1"]
pub mod ro_cal1;
#[doc = "RO Trim Calibration Register 2"]
pub struct RO_CAL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RO Trim Calibration Register 2"]
pub mod ro_cal2;
