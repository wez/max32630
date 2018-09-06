#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Clock Configuration"]
    pub clk_config: CLK_CONFIG,
    #[doc = "0x04 - System Clock Controls"]
    pub clk_ctrl: CLK_CTRL,
    #[doc = "0x08 - Interrupt Flags"]
    pub intfl: INTFL,
    #[doc = "0x0c - Interrupt Enable/Disable Controls"]
    pub inten: INTEN,
    #[doc = "0x10 - Trim Calculation Controls"]
    pub trim_calc: TRIM_CALC,
    #[doc = "0x14 - I2C Timer Control"]
    pub i2c_timer_ctrl: I2C_TIMER_CTRL,
    #[doc = "0x18 - CM4 Start Clock on Interrupt Enable 0"]
    pub cm4_start_clk_en0: CM4_START_CLK_EN0,
    #[doc = "0x1c - CM4 Start Clock on Interrupt Enable 1"]
    pub cm4_start_clk_en1: CM4_START_CLK_EN1,
    #[doc = "0x20 - CM4 Start Clock on Interrupt Enable 2"]
    pub cm4_start_clk_en2: CM4_START_CLK_EN2,
    _reserved9: [u8; 28usize],
    #[doc = "0x40 - Control Settings for CLK0 - Cortex M4 Clock"]
    pub sys_clk_ctrl_0_cm4: SYS_CLK_CTRL_0_CM4,
    #[doc = "0x44 - Control Settings for CLK1 - Synchronizer Clock"]
    pub sys_clk_ctrl_1_sync: SYS_CLK_CTRL_1_SYNC,
    #[doc = "0x48 - Control Settings for CLK2 - SPI XIP Clock"]
    pub sys_clk_ctrl_2_spix: SYS_CLK_CTRL_2_SPIX,
    #[doc = "0x4c - Control Settings for CLK3 - PRNG Clock"]
    pub sys_clk_ctrl_3_prng: SYS_CLK_CTRL_3_PRNG,
    #[doc = "0x50 - Control Settings for CLK4 - Watchdog Timer 0"]
    pub sys_clk_ctrl_4_wdt0: SYS_CLK_CTRL_4_WDT0,
    #[doc = "0x54 - Control Settings for CLK5 - Watchdog Timer 1"]
    pub sys_clk_ctrl_5_wdt1: SYS_CLK_CTRL_5_WDT1,
    #[doc = "0x58 - Control Settings for CLK6 - Clock for GPIO Ports"]
    pub sys_clk_ctrl_6_gpio: SYS_CLK_CTRL_6_GPIO,
    #[doc = "0x5c - Control Settings for CLK7 - Source Clock for All Pulse Trains"]
    pub sys_clk_ctrl_7_pt: SYS_CLK_CTRL_7_PT,
    #[doc = "0x60 - Control Settings for CLK8 - Source Clock for All UARTs"]
    pub sys_clk_ctrl_8_uart: SYS_CLK_CTRL_8_UART,
    #[doc = "0x64 - Control Settings for CLK9 - Source Clock for All I2C Masters"]
    pub sys_clk_ctrl_9_i2cm: SYS_CLK_CTRL_9_I2CM,
    #[doc = "0x68 - Control Settings for CLK10 - Source Clock for I2C Slave"]
    pub sys_clk_ctrl_10_i2cs: SYS_CLK_CTRL_10_I2CS,
    #[doc = "0x6c - Control Settings for CLK11 - SPI Master 0"]
    pub sys_clk_ctrl_11_spi0: SYS_CLK_CTRL_11_SPI0,
    #[doc = "0x70 - Control Settings for CLK12 - SPI Master 1"]
    pub sys_clk_ctrl_12_spi1: SYS_CLK_CTRL_12_SPI1,
    #[doc = "0x74 - Control Settings for CLK13 - SPI Master 2"]
    pub sys_clk_ctrl_13_spi2: SYS_CLK_CTRL_13_SPI2,
    #[doc = "0x78 - Control Settings for CLK14 - SPI Bridge Clock"]
    pub sys_clk_ctrl_14_spib: SYS_CLK_CTRL_14_SPIB,
    #[doc = "0x7c - Control Settings for CLK15 - 1-Wire Master Clock"]
    pub sys_clk_ctrl_15_owm: SYS_CLK_CTRL_15_OWM,
    #[doc = "0x80 - Control Settings for CLK16 - SPI Slave Clock"]
    pub sys_clk_ctrl_16_spis: SYS_CLK_CTRL_16_SPIS,
    _reserved26: [u8; 124usize],
    #[doc = "0x100 - Control Settings for Crypto Clock 0 - AES"]
    pub crypt_clk_ctrl_0_aes: CRYPT_CLK_CTRL_0_AES,
    #[doc = "0x104 - Control Settings for Crypto Clock 1 - MAA"]
    pub crypt_clk_ctrl_1_maa: CRYPT_CLK_CTRL_1_MAA,
    #[doc = "0x108 - Control Settings for Crypto Clock 2 - PRNG"]
    pub crypt_clk_ctrl_2_prng: CRYPT_CLK_CTRL_2_PRNG,
    _reserved29: [u8; 52usize],
    #[doc = "0x140 - Dynamic Clock Gating Control Register 0"]
    pub clk_gate_ctrl0: CLK_GATE_CTRL0,
    #[doc = "0x144 - Dynamic Clock Gating Control Register 1"]
    pub clk_gate_ctrl1: CLK_GATE_CTRL1,
    #[doc = "0x148 - Dynamic Clock Gating Control Register 2"]
    pub clk_gate_ctrl2: CLK_GATE_CTRL2,
}
#[doc = "System Clock Configuration"]
pub struct CLK_CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Clock Configuration"]
pub mod clk_config;
#[doc = "System Clock Controls"]
pub struct CLK_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Clock Controls"]
pub mod clk_ctrl;
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
#[doc = "Trim Calculation Controls"]
pub struct TRIM_CALC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Trim Calculation Controls"]
pub mod trim_calc;
#[doc = "I2C Timer Control"]
pub struct I2C_TIMER_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Timer Control"]
pub mod i2c_timer_ctrl;
#[doc = "CM4 Start Clock on Interrupt Enable 0"]
pub struct CM4_START_CLK_EN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM4 Start Clock on Interrupt Enable 0"]
pub mod cm4_start_clk_en0;
#[doc = "CM4 Start Clock on Interrupt Enable 1"]
pub struct CM4_START_CLK_EN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM4 Start Clock on Interrupt Enable 1"]
pub mod cm4_start_clk_en1;
#[doc = "CM4 Start Clock on Interrupt Enable 2"]
pub struct CM4_START_CLK_EN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM4 Start Clock on Interrupt Enable 2"]
pub mod cm4_start_clk_en2;
#[doc = "Control Settings for CLK0 - Cortex M4 Clock"]
pub struct SYS_CLK_CTRL_0_CM4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK0 - Cortex M4 Clock"]
pub mod sys_clk_ctrl_0_cm4;
#[doc = "Control Settings for CLK1 - Synchronizer Clock"]
pub struct SYS_CLK_CTRL_1_SYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK1 - Synchronizer Clock"]
pub mod sys_clk_ctrl_1_sync;
#[doc = "Control Settings for CLK2 - SPI XIP Clock"]
pub struct SYS_CLK_CTRL_2_SPIX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK2 - SPI XIP Clock"]
pub mod sys_clk_ctrl_2_spix;
#[doc = "Control Settings for CLK3 - PRNG Clock"]
pub struct SYS_CLK_CTRL_3_PRNG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK3 - PRNG Clock"]
pub mod sys_clk_ctrl_3_prng;
#[doc = "Control Settings for CLK4 - Watchdog Timer 0"]
pub struct SYS_CLK_CTRL_4_WDT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK4 - Watchdog Timer 0"]
pub mod sys_clk_ctrl_4_wdt0;
#[doc = "Control Settings for CLK5 - Watchdog Timer 1"]
pub struct SYS_CLK_CTRL_5_WDT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK5 - Watchdog Timer 1"]
pub mod sys_clk_ctrl_5_wdt1;
#[doc = "Control Settings for CLK6 - Clock for GPIO Ports"]
pub struct SYS_CLK_CTRL_6_GPIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK6 - Clock for GPIO Ports"]
pub mod sys_clk_ctrl_6_gpio;
#[doc = "Control Settings for CLK7 - Source Clock for All Pulse Trains"]
pub struct SYS_CLK_CTRL_7_PT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK7 - Source Clock for All Pulse Trains"]
pub mod sys_clk_ctrl_7_pt;
#[doc = "Control Settings for CLK8 - Source Clock for All UARTs"]
pub struct SYS_CLK_CTRL_8_UART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK8 - Source Clock for All UARTs"]
pub mod sys_clk_ctrl_8_uart;
#[doc = "Control Settings for CLK9 - Source Clock for All I2C Masters"]
pub struct SYS_CLK_CTRL_9_I2CM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK9 - Source Clock for All I2C Masters"]
pub mod sys_clk_ctrl_9_i2cm;
#[doc = "Control Settings for CLK10 - Source Clock for I2C Slave"]
pub struct SYS_CLK_CTRL_10_I2CS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK10 - Source Clock for I2C Slave"]
pub mod sys_clk_ctrl_10_i2cs;
#[doc = "Control Settings for CLK11 - SPI Master 0"]
pub struct SYS_CLK_CTRL_11_SPI0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK11 - SPI Master 0"]
pub mod sys_clk_ctrl_11_spi0;
#[doc = "Control Settings for CLK12 - SPI Master 1"]
pub struct SYS_CLK_CTRL_12_SPI1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK12 - SPI Master 1"]
pub mod sys_clk_ctrl_12_spi1;
#[doc = "Control Settings for CLK13 - SPI Master 2"]
pub struct SYS_CLK_CTRL_13_SPI2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK13 - SPI Master 2"]
pub mod sys_clk_ctrl_13_spi2;
#[doc = "Control Settings for CLK14 - SPI Bridge Clock"]
pub struct SYS_CLK_CTRL_14_SPIB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK14 - SPI Bridge Clock"]
pub mod sys_clk_ctrl_14_spib;
#[doc = "Control Settings for CLK15 - 1-Wire Master Clock"]
pub struct SYS_CLK_CTRL_15_OWM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK15 - 1-Wire Master Clock"]
pub mod sys_clk_ctrl_15_owm;
#[doc = "Control Settings for CLK16 - SPI Slave Clock"]
pub struct SYS_CLK_CTRL_16_SPIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for CLK16 - SPI Slave Clock"]
pub mod sys_clk_ctrl_16_spis;
#[doc = "Control Settings for Crypto Clock 0 - AES"]
pub struct CRYPT_CLK_CTRL_0_AES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for Crypto Clock 0 - AES"]
pub mod crypt_clk_ctrl_0_aes;
#[doc = "Control Settings for Crypto Clock 1 - MAA"]
pub struct CRYPT_CLK_CTRL_1_MAA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for Crypto Clock 1 - MAA"]
pub mod crypt_clk_ctrl_1_maa;
#[doc = "Control Settings for Crypto Clock 2 - PRNG"]
pub struct CRYPT_CLK_CTRL_2_PRNG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Settings for Crypto Clock 2 - PRNG"]
pub mod crypt_clk_ctrl_2_prng;
#[doc = "Dynamic Clock Gating Control Register 0"]
pub struct CLK_GATE_CTRL0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dynamic Clock Gating Control Register 0"]
pub mod clk_gate_ctrl0;
#[doc = "Dynamic Clock Gating Control Register 1"]
pub struct CLK_GATE_CTRL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dynamic Clock Gating Control Register 1"]
pub mod clk_gate_ctrl1;
#[doc = "Dynamic Clock Gating Control Register 2"]
pub struct CLK_GATE_CTRL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dynamic Clock Gating Control Register 2"]
pub mod clk_gate_ctrl2;
