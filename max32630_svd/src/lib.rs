#![doc = "Peripheral access API for MAX32630 microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(intra_doc_link_resolution_failure)]
#![allow(non_camel_case_types)]
#![no_std]
#![feature(untagged_unions)]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn CLKMAN();
    fn POWERMANAGER();
    fn FLC();
    fn RTC_COMP0();
    fn RTC_COMP1();
    fn RTC_PRESCALE_COMP();
    fn RTC_OVERFLOW();
    fn PMU();
    fn USB();
    fn AES();
    fn MAA();
    fn WDT0();
    fn WDT0_PRE_WIN();
    fn WDT1();
    fn WDT1_PRE_WIN();
    fn GPIO0();
    fn GPIO1();
    fn GPIO2();
    fn GPIO3();
    fn GPIO4();
    fn GPIO5();
    fn GPIO6();
    fn TMR0();
    fn TMR16_0();
    fn TMR1();
    fn TMR16_1();
    fn TMR2();
    fn TMR16_2();
    fn TMR3();
    fn TMR16_3();
    fn TMR4();
    fn TMR16_4();
    fn TMR5();
    fn TMR16_5();
    fn UART0();
    fn UART1();
    fn UART2();
    fn UART3();
    fn PT();
    fn I2CM0();
    fn I2CM1();
    fn I2CM2();
    fn I2CS();
    fn SPIM0();
    fn SPIM1();
    fn SPIM2();
    fn OWM();
    fn ADC();
    fn SPIS();
    fn GPIO7();
    fn GPIO8();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 52] = [
    Vector { _handler: CLKMAN },
    Vector {
        _handler: POWERMANAGER,
    },
    Vector { _handler: FLC },
    Vector {
        _handler: RTC_COMP0,
    },
    Vector {
        _handler: RTC_COMP1,
    },
    Vector {
        _handler: RTC_PRESCALE_COMP,
    },
    Vector {
        _handler: RTC_OVERFLOW,
    },
    Vector { _handler: PMU },
    Vector { _handler: USB },
    Vector { _handler: AES },
    Vector { _handler: MAA },
    Vector { _handler: WDT0 },
    Vector {
        _handler: WDT0_PRE_WIN,
    },
    Vector { _handler: WDT1 },
    Vector {
        _handler: WDT1_PRE_WIN,
    },
    Vector { _handler: GPIO0 },
    Vector { _handler: GPIO1 },
    Vector { _handler: GPIO2 },
    Vector { _handler: GPIO3 },
    Vector { _handler: GPIO4 },
    Vector { _handler: GPIO5 },
    Vector { _handler: GPIO6 },
    Vector { _handler: TMR0 },
    Vector { _handler: TMR16_0 },
    Vector { _handler: TMR1 },
    Vector { _handler: TMR16_1 },
    Vector { _handler: TMR2 },
    Vector { _handler: TMR16_2 },
    Vector { _handler: TMR3 },
    Vector { _handler: TMR16_3 },
    Vector { _handler: TMR4 },
    Vector { _handler: TMR16_4 },
    Vector { _handler: TMR5 },
    Vector { _handler: TMR16_5 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: UART2 },
    Vector { _handler: UART3 },
    Vector { _handler: PT },
    Vector { _handler: I2CM0 },
    Vector { _handler: I2CM1 },
    Vector { _handler: I2CM2 },
    Vector { _handler: I2CS },
    Vector { _handler: SPIM0 },
    Vector { _handler: SPIM1 },
    Vector { _handler: SPIM2 },
    Vector { _reserved: 0 },
    Vector { _handler: OWM },
    Vector { _handler: ADC },
    Vector { _handler: SPIS },
    Vector { _handler: GPIO7 },
    Vector { _handler: GPIO8 },
];
#[doc = r" Macro to override a device specific interrupt handler"]
#[doc = r""]
#[doc = r" # Syntax"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!("]
#[doc = r"     // Name of the interrupt"]
#[doc = r"     $Name:ident,"]
#[doc = r""]
#[doc = r"     // Path to the interrupt handler (a function)"]
#[doc = r"     $handler:path,"]
#[doc = r""]
#[doc = r"     // Optional, state preserved across invocations of the handler"]
#[doc = r"     state: $State:ty = $initial_state:expr,"]
#[doc = r" );"]
#[doc = r" ```"]
#[doc = r""]
#[doc = r" Where `$Name` must match the name of one of the variants of the `Interrupt`"]
#[doc = r" enum."]
#[doc = r""]
#[doc = r" The handler must have signature `fn()` is no state was associated to it;"]
#[doc = r" otherwise its signature must be `fn(&mut $State)`."]
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! interrupt {
    ( $ Name : ident , $ handler : path , state : $ State : ty = $ initial_state : expr ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;
            let _ = $crate::Interrupt::$Name;
            let f: fn(&mut $State) = $handler;
            f(&mut STATE)
        }
    };
    ( $ Name : ident , $ handler : path ) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            let _ = $crate::Interrupt::$Name;
            let f: fn() = $handler;
            f()
        }
    };
}
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Clock Management IRQ"]
    CLKMAN,
    #[doc = "1 - Power Manager IRQ"]
    POWERMANAGER,
    #[doc = "2 - Flash Controller IRQ"]
    FLC,
    #[doc = "3 - RTC Compare 0 IRQ"]
    RTC_COMP0,
    #[doc = "4 - RTC Compare 1 IRQ"]
    RTC_COMP1,
    #[doc = "5 - RTC Prescale Compare IRQ"]
    RTC_PRESCALE_COMP,
    #[doc = "6 - RTC Overflow IRQ"]
    RTC_OVERFLOW,
    #[doc = "7 - Peripheral Manament IRQ"]
    PMU,
    #[doc = "8 - USB IRQ"]
    USB,
    #[doc = "9 - AES IRQ"]
    AES,
    #[doc = "10 - Modular Arithematic Accelerator IRQ"]
    MAA,
    #[doc = "11 - Watch Dog Timer 0 IRQ"]
    WDT0,
    #[doc = "12 - Watch Dog Timer 0 Pre-Window IRQ"]
    WDT0_PRE_WIN,
    #[doc = "13 - Watch Dog Timer 1 IRQ"]
    WDT1,
    #[doc = "14 - Watch Dog Timer 1 Pre-Window IRQ"]
    WDT1_PRE_WIN,
    #[doc = "15 - GPIO Port 0 IRQ"]
    GPIO0,
    #[doc = "16 - GPIO Port 1 IRQ"]
    GPIO1,
    #[doc = "17 - GPIO Port 2 IRQ"]
    GPIO2,
    #[doc = "18 - GPIO Port 3 IRQ"]
    GPIO3,
    #[doc = "19 - GPIO Port 4 IRQ"]
    GPIO4,
    #[doc = "20 - GPIO Port 5 IRQ"]
    GPIO5,
    #[doc = "21 - GPIO Port 6 IRQ"]
    GPIO6,
    #[doc = "22 - Timer 0 IRQ"]
    TMR0,
    #[doc = "23 - 16-bit Timer 0 IRQ"]
    TMR16_0,
    #[doc = "24 - Timer 1 IRQ"]
    TMR1,
    #[doc = "25 - 16-bit Timer 1 IRQ"]
    TMR16_1,
    #[doc = "26 - Timer 2 IRQ"]
    TMR2,
    #[doc = "27 - 16-bit Timer 2 IRQ"]
    TMR16_2,
    #[doc = "28 - Timer 3 IRQ"]
    TMR3,
    #[doc = "29 - 16-bit Timer 3 IRQ"]
    TMR16_3,
    #[doc = "30 - Timer 4 IRQ"]
    TMR4,
    #[doc = "31 - 16-bit Timer 4 IRQ"]
    TMR16_4,
    #[doc = "32 - Timer 5 IRQ"]
    TMR5,
    #[doc = "33 - 16-bit Timer 5 IRQ"]
    TMR16_5,
    #[doc = "34 - UART 0 IRQ"]
    UART0,
    #[doc = "35 - UART 1 IRQ"]
    UART1,
    #[doc = "36 - UART 2 IRQ"]
    UART2,
    #[doc = "37 - UART 3 IRQ"]
    UART3,
    #[doc = "38 - Pulse Train IRQ"]
    PT,
    #[doc = "39 - I2C Master 0 IRQ"]
    I2CM0,
    #[doc = "40 - I2C Master 1 IRQ"]
    I2CM1,
    #[doc = "41 - I2C Master 2 IRQ"]
    I2CM2,
    #[doc = "42 - I2C Slave IRQ"]
    I2CS,
    #[doc = "43 - SPI Master 0 IRQ"]
    SPIM0,
    #[doc = "44 - SPI Master 1 IRQ"]
    SPIM1,
    #[doc = "45 - SPI Master 2 IRQ"]
    SPIM2,
    #[doc = "47 - 1-Wire Master IRQ"]
    OWM,
    #[doc = "48 - ADC IRQ"]
    ADC,
    #[doc = "49 - SPI Slave IRQ"]
    SPIS,
    #[doc = "50 - GPIO Port 7 IRQ"]
    GPIO7,
    #[doc = "51 - GPIO Port 8 IRQ"]
    GPIO8,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::CLKMAN => 0,
            Interrupt::POWERMANAGER => 1,
            Interrupt::FLC => 2,
            Interrupt::RTC_COMP0 => 3,
            Interrupt::RTC_COMP1 => 4,
            Interrupt::RTC_PRESCALE_COMP => 5,
            Interrupt::RTC_OVERFLOW => 6,
            Interrupt::PMU => 7,
            Interrupt::USB => 8,
            Interrupt::AES => 9,
            Interrupt::MAA => 10,
            Interrupt::WDT0 => 11,
            Interrupt::WDT0_PRE_WIN => 12,
            Interrupt::WDT1 => 13,
            Interrupt::WDT1_PRE_WIN => 14,
            Interrupt::GPIO0 => 15,
            Interrupt::GPIO1 => 16,
            Interrupt::GPIO2 => 17,
            Interrupt::GPIO3 => 18,
            Interrupt::GPIO4 => 19,
            Interrupt::GPIO5 => 20,
            Interrupt::GPIO6 => 21,
            Interrupt::TMR0 => 22,
            Interrupt::TMR16_0 => 23,
            Interrupt::TMR1 => 24,
            Interrupt::TMR16_1 => 25,
            Interrupt::TMR2 => 26,
            Interrupt::TMR16_2 => 27,
            Interrupt::TMR3 => 28,
            Interrupt::TMR16_3 => 29,
            Interrupt::TMR4 => 30,
            Interrupt::TMR16_4 => 31,
            Interrupt::TMR5 => 32,
            Interrupt::TMR16_5 => 33,
            Interrupt::UART0 => 34,
            Interrupt::UART1 => 35,
            Interrupt::UART2 => 36,
            Interrupt::UART3 => 37,
            Interrupt::PT => 38,
            Interrupt::I2CM0 => 39,
            Interrupt::I2CM1 => 40,
            Interrupt::I2CM2 => 41,
            Interrupt::I2CS => 42,
            Interrupt::SPIM0 => 43,
            Interrupt::SPIM1 => 44,
            Interrupt::SPIM2 => 45,
            Interrupt::OWM => 47,
            Interrupt::ADC => 48,
            Interrupt::SPIS => 49,
            Interrupt::GPIO7 => 50,
            Interrupt::GPIO8 => 51,
        }
    }
}
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[doc = "System Clock Manager"]
pub struct CLKMAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLKMAN {}
impl CLKMAN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const clkman::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for CLKMAN {
    type Target = clkman::RegisterBlock;
    fn deref(&self) -> &clkman::RegisterBlock {
        unsafe { &*CLKMAN::ptr() }
    }
}
#[doc = "System Clock Manager"]
pub mod clkman;
#[doc = "System Power Manager"]
pub struct PWRMAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWRMAN {}
impl PWRMAN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwrman::RegisterBlock {
        1073743872 as *const _
    }
}
impl Deref for PWRMAN {
    type Target = pwrman::RegisterBlock;
    fn deref(&self) -> &pwrman::RegisterBlock {
        unsafe { &*PWRMAN::ptr() }
    }
}
#[doc = "System Power Manager"]
pub mod pwrman;
#[doc = "Real Time Clock"]
pub struct RTCTMR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCTMR {}
impl RTCTMR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtctmr::RegisterBlock {
        1073744384 as *const _
    }
}
impl Deref for RTCTMR {
    type Target = rtctmr::RegisterBlock;
    fn deref(&self) -> &rtctmr::RegisterBlock {
        unsafe { &*RTCTMR::ptr() }
    }
}
#[doc = "Real Time Clock"]
pub mod rtctmr;
#[doc = "RTC Configuration Register"]
pub struct RTCCFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCCFG {}
impl RTCCFG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtccfg::RegisterBlock {
        1073744496 as *const _
    }
}
impl Deref for RTCCFG {
    type Target = rtccfg::RegisterBlock;
    fn deref(&self) -> &rtccfg::RegisterBlock {
        unsafe { &*RTCCFG::ptr() }
    }
}
#[doc = "RTC Configuration Register"]
pub mod rtccfg;
#[doc = "Power Sequencer"]
pub struct PWRSEQ {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWRSEQ {}
impl PWRSEQ {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwrseq::RegisterBlock {
        1073744432 as *const _
    }
}
impl Deref for PWRSEQ {
    type Target = pwrseq::RegisterBlock;
    fn deref(&self) -> &pwrseq::RegisterBlock {
        unsafe { &*PWRSEQ::ptr() }
    }
}
#[doc = "Power Sequencer"]
pub mod pwrseq;
#[doc = "System I/O Manager"]
pub struct IOMAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOMAN {}
impl IOMAN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ioman::RegisterBlock {
        1073744896 as *const _
    }
}
impl Deref for IOMAN {
    type Target = ioman::RegisterBlock;
    fn deref(&self) -> &ioman::RegisterBlock {
        unsafe { &*IOMAN::ptr() }
    }
}
#[doc = "System I/O Manager"]
pub mod ioman;
#[doc = "Flash Controller"]
pub struct FLC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLC {}
impl FLC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flc::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for FLC {
    type Target = flc::RegisterBlock;
    fn deref(&self) -> &flc::RegisterBlock {
        unsafe { &*FLC::ptr() }
    }
}
#[doc = "Flash Controller"]
pub mod flc;
#[doc = "Instruction Cache Controller"]
pub struct ICC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICC {}
impl ICC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const icc::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for ICC {
    type Target = icc::RegisterBlock;
    fn deref(&self) -> &icc::RegisterBlock {
        unsafe { &*ICC::ptr() }
    }
}
#[doc = "Instruction Cache Controller"]
pub mod icc;
#[doc = "SPI XIP Interface"]
pub struct SPIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIX {}
impl SPIX {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spix::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPIX {
    type Target = spix::RegisterBlock;
    fn deref(&self) -> &spix::RegisterBlock {
        unsafe { &*SPIX::ptr() }
    }
}
#[doc = "SPI XIP Interface"]
pub mod spix;
#[doc = "Peripheral Management Unit"]
pub struct PMU0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU0 {}
impl PMU0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu0::RegisterBlock {
        1073762304 as *const _
    }
}
impl Deref for PMU0 {
    type Target = pmu0::RegisterBlock;
    fn deref(&self) -> &pmu0::RegisterBlock {
        unsafe { &*PMU0::ptr() }
    }
}
#[doc = "Peripheral Management Unit"]
pub mod pmu0;
#[doc = "Peripheral Management Unit"]
pub struct PMU1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU1 {}
impl PMU1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu0::RegisterBlock {
        1073762336 as *const _
    }
}
impl Deref for PMU1 {
    type Target = pmu0::RegisterBlock;
    fn deref(&self) -> &pmu0::RegisterBlock {
        unsafe { &*PMU1::ptr() }
    }
}
#[doc = "Peripheral Management Unit"]
pub struct PMU2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU2 {}
impl PMU2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu0::RegisterBlock {
        1073762368 as *const _
    }
}
impl Deref for PMU2 {
    type Target = pmu0::RegisterBlock;
    fn deref(&self) -> &pmu0::RegisterBlock {
        unsafe { &*PMU2::ptr() }
    }
}
#[doc = "Peripheral Management Unit"]
pub struct PMU3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU3 {}
impl PMU3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu0::RegisterBlock {
        1073762400 as *const _
    }
}
impl Deref for PMU3 {
    type Target = pmu0::RegisterBlock;
    fn deref(&self) -> &pmu0::RegisterBlock {
        unsafe { &*PMU3::ptr() }
    }
}
#[doc = "Peripheral Management Unit"]
pub struct PMU4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU4 {}
impl PMU4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu0::RegisterBlock {
        1073762432 as *const _
    }
}
impl Deref for PMU4 {
    type Target = pmu0::RegisterBlock;
    fn deref(&self) -> &pmu0::RegisterBlock {
        unsafe { &*PMU4::ptr() }
    }
}
#[doc = "Peripheral Management Unit"]
pub struct PMU5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMU5 {}
impl PMU5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmu0::RegisterBlock {
        1073762464 as *const _
    }
}
impl Deref for PMU5 {
    type Target = pmu0::RegisterBlock;
    fn deref(&self) -> &pmu0::RegisterBlock {
        unsafe { &*PMU5::ptr() }
    }
}
#[doc = "USB Device Controller"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        1074790400 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB Device Controller"]
pub mod usb;
#[doc = "CRC-16/CRC-32 Engine"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073766400 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "CRC-16/CRC-32 Engine"]
pub mod crc;
#[doc = "Trust Protection Unit (TPU)"]
pub struct TPU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPU {}
impl TPU {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tpu::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for TPU {
    type Target = tpu::RegisterBlock;
    fn deref(&self) -> &tpu::RegisterBlock {
        unsafe { &*TPU::ptr() }
    }
}
#[doc = "Trust Protection Unit (TPU)"]
pub mod tpu;
#[doc = "Trust Protection Unit (TPU)"]
pub struct TPU_TSR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPU_TSR {}
impl TPU_TSR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tpu_tsr::RegisterBlock {
        1073773568 as *const _
    }
}
impl Deref for TPU_TSR {
    type Target = tpu_tsr::RegisterBlock;
    fn deref(&self) -> &tpu_tsr::RegisterBlock {
        unsafe { &*TPU_TSR::ptr() }
    }
}
#[doc = "Trust Protection Unit (TPU)"]
pub mod tpu_tsr;
#[doc = "AES Cryptographic Engine"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aes::RegisterBlock {
        1073771520 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    fn deref(&self) -> &aes::RegisterBlock {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "AES Cryptographic Engine"]
pub mod aes;
#[doc = "MAA Cryptographic Engine"]
pub struct MAA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MAA {}
impl MAA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const maa::RegisterBlock {
        1073772544 as *const _
    }
}
impl Deref for MAA {
    type Target = maa::RegisterBlock;
    fn deref(&self) -> &maa::RegisterBlock {
        unsafe { &*MAA::ptr() }
    }
}
#[doc = "MAA Cryptographic Engine"]
pub mod maa;
#[doc = "Watchdog Timers"]
pub struct WDT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT0 {}
impl WDT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt0::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for WDT0 {
    type Target = wdt0::RegisterBlock;
    fn deref(&self) -> &wdt0::RegisterBlock {
        unsafe { &*WDT0::ptr() }
    }
}
#[doc = "Watchdog Timers"]
pub mod wdt0;
#[doc = "Watchdog Timers"]
pub struct WDT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT1 {}
impl WDT1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt0::RegisterBlock {
        1073778688 as *const _
    }
}
impl Deref for WDT1 {
    type Target = wdt0::RegisterBlock;
    fn deref(&self) -> &wdt0::RegisterBlock {
        unsafe { &*WDT1::ptr() }
    }
}
#[doc = "General Purpose I/O Ports (GPIO)"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General Purpose I/O Ports (GPIO)"]
pub mod gpio;
#[doc = "16/32 bit Timer/Counters"]
pub struct TMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR0 {}
impl TMR0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tmr0::RegisterBlock {
        1073786880 as *const _
    }
}
impl Deref for TMR0 {
    type Target = tmr0::RegisterBlock;
    fn deref(&self) -> &tmr0::RegisterBlock {
        unsafe { &*TMR0::ptr() }
    }
}
#[doc = "16/32 bit Timer/Counters"]
pub mod tmr0;
#[doc = "16/32 bit Timer/Counters"]
pub struct TMR1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR1 {}
impl TMR1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tmr0::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for TMR1 {
    type Target = tmr0::RegisterBlock;
    fn deref(&self) -> &tmr0::RegisterBlock {
        unsafe { &*TMR1::ptr() }
    }
}
#[doc = "16/32 bit Timer/Counters"]
pub struct TMR2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR2 {}
impl TMR2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tmr0::RegisterBlock {
        1073795072 as *const _
    }
}
impl Deref for TMR2 {
    type Target = tmr0::RegisterBlock;
    fn deref(&self) -> &tmr0::RegisterBlock {
        unsafe { &*TMR2::ptr() }
    }
}
#[doc = "16/32 bit Timer/Counters"]
pub struct TMR3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR3 {}
impl TMR3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tmr0::RegisterBlock {
        1073799168 as *const _
    }
}
impl Deref for TMR3 {
    type Target = tmr0::RegisterBlock;
    fn deref(&self) -> &tmr0::RegisterBlock {
        unsafe { &*TMR3::ptr() }
    }
}
#[doc = "16/32 bit Timer/Counters"]
pub struct TMR4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR4 {}
impl TMR4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tmr0::RegisterBlock {
        1073803264 as *const _
    }
}
impl Deref for TMR4 {
    type Target = tmr0::RegisterBlock;
    fn deref(&self) -> &tmr0::RegisterBlock {
        unsafe { &*TMR4::ptr() }
    }
}
#[doc = "16/32 bit Timer/Counters"]
pub struct TMR5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR5 {}
impl TMR5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tmr0::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for TMR5 {
    type Target = tmr0::RegisterBlock;
    fn deref(&self) -> &tmr0::RegisterBlock {
        unsafe { &*TMR5::ptr() }
    }
}
#[doc = "UART / Serial Port Interface"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073815552 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART / Serial Port Interface"]
pub mod uart0;
#[doc = "UART / Serial Port Interface"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART / Serial Port Interface"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "UART / Serial Port Interface"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073827840 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PTG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PTG {}
impl PTG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ptg::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for PTG {
    type Target = ptg::RegisterBlock;
    fn deref(&self) -> &ptg::RegisterBlock {
        unsafe { &*PTG::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub mod ptg;
#[doc = "Pulse Train Generation"]
pub struct PT0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT0 {}
impl PT0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811472 as *const _
    }
}
impl Deref for PT0 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT0::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub mod pt0;
#[doc = "Pulse Train Generation"]
pub struct PT1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT1 {}
impl PT1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811484 as *const _
    }
}
impl Deref for PT1 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT1::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT2 {}
impl PT2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811496 as *const _
    }
}
impl Deref for PT2 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT2::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT3 {}
impl PT3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811508 as *const _
    }
}
impl Deref for PT3 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT3::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT4 {}
impl PT4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811520 as *const _
    }
}
impl Deref for PT4 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT4::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT5 {}
impl PT5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811532 as *const _
    }
}
impl Deref for PT5 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT5::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT6 {}
impl PT6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811544 as *const _
    }
}
impl Deref for PT6 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT6::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT7 {}
impl PT7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811556 as *const _
    }
}
impl Deref for PT7 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT7::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT8 {}
impl PT8 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811568 as *const _
    }
}
impl Deref for PT8 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT8::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT9 {}
impl PT9 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811580 as *const _
    }
}
impl Deref for PT9 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT9::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT10 {}
impl PT10 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811592 as *const _
    }
}
impl Deref for PT10 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT10::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT11 {}
impl PT11 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811604 as *const _
    }
}
impl Deref for PT11 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT11::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT12 {}
impl PT12 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811616 as *const _
    }
}
impl Deref for PT12 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT12::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT13 {}
impl PT13 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811628 as *const _
    }
}
impl Deref for PT13 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT13::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT14 {}
impl PT14 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811640 as *const _
    }
}
impl Deref for PT14 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT14::ptr() }
    }
}
#[doc = "Pulse Train Generation"]
pub struct PT15 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PT15 {}
impl PT15 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pt0::RegisterBlock {
        1073811652 as *const _
    }
}
impl Deref for PT15 {
    type Target = pt0::RegisterBlock;
    fn deref(&self) -> &pt0::RegisterBlock {
        unsafe { &*PT15::ptr() }
    }
}
#[doc = "I2C Master 0 Interface"]
pub struct I2CM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2CM0 {}
impl I2CM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2cm0::RegisterBlock {
        1073831936 as *const _
    }
}
impl Deref for I2CM0 {
    type Target = i2cm0::RegisterBlock;
    fn deref(&self) -> &i2cm0::RegisterBlock {
        unsafe { &*I2CM0::ptr() }
    }
}
#[doc = "I2C Master 0 Interface"]
pub mod i2cm0;
#[doc = "I2C Master 0 Interface"]
pub struct I2CM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2CM1 {}
impl I2CM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2cm0::RegisterBlock {
        1073836032 as *const _
    }
}
impl Deref for I2CM1 {
    type Target = i2cm0::RegisterBlock;
    fn deref(&self) -> &i2cm0::RegisterBlock {
        unsafe { &*I2CM1::ptr() }
    }
}
#[doc = "I2C Master 0 Interface"]
pub struct I2CM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2CM2 {}
impl I2CM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2cm0::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for I2CM2 {
    type Target = i2cm0::RegisterBlock;
    fn deref(&self) -> &i2cm0::RegisterBlock {
        unsafe { &*I2CM2::ptr() }
    }
}
#[doc = "I2C Slave Interface"]
pub struct I2CS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2CS {}
impl I2CS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2cs::RegisterBlock {
        1073844224 as *const _
    }
}
impl Deref for I2CS {
    type Target = i2cs::RegisterBlock;
    fn deref(&self) -> &i2cs::RegisterBlock {
        unsafe { &*I2CS::ptr() }
    }
}
#[doc = "I2C Slave Interface"]
pub mod i2cs;
#[doc = "SPI Master Interface"]
pub struct SPIM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM0 {}
impl SPIM0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0::RegisterBlock {
        1073848320 as *const _
    }
}
impl Deref for SPIM0 {
    type Target = spim0::RegisterBlock;
    fn deref(&self) -> &spim0::RegisterBlock {
        unsafe { &*SPIM0::ptr() }
    }
}
#[doc = "SPI Master Interface"]
pub mod spim0;
#[doc = "SPI Master Interface"]
pub struct SPIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM1 {}
impl SPIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0::RegisterBlock {
        1073852416 as *const _
    }
}
impl Deref for SPIM1 {
    type Target = spim0::RegisterBlock;
    fn deref(&self) -> &spim0::RegisterBlock {
        unsafe { &*SPIM1::ptr() }
    }
}
#[doc = "SPI Master Interface"]
pub struct SPIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM2 {}
impl SPIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim0::RegisterBlock {
        1073856512 as *const _
    }
}
impl Deref for SPIM2 {
    type Target = spim0::RegisterBlock;
    fn deref(&self) -> &spim0::RegisterBlock {
        unsafe { &*SPIM2::ptr() }
    }
}
#[doc = "1-Wire Master Interface"]
pub struct OWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OWM {}
impl OWM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const owm::RegisterBlock {
        1073864704 as *const _
    }
}
impl Deref for OWM {
    type Target = owm::RegisterBlock;
    fn deref(&self) -> &owm::RegisterBlock {
        unsafe { &*OWM::ptr() }
    }
}
#[doc = "1-Wire Master Interface"]
pub mod owm;
#[doc = "10-bit Analog to Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1073868800 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "10-bit Analog to Digital Converter"]
pub mod adc;
#[doc = "SPI Slave Interface"]
pub struct SPIS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS {}
impl SPIS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for SPIS {
    type Target = spis::RegisterBlock;
    fn deref(&self) -> &spis::RegisterBlock {
        unsafe { &*SPIS::ptr() }
    }
}
#[doc = "SPI Slave Interface"]
pub mod spis;
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "CLKMAN"]
    pub CLKMAN: CLKMAN,
    #[doc = "PWRMAN"]
    pub PWRMAN: PWRMAN,
    #[doc = "RTCTMR"]
    pub RTCTMR: RTCTMR,
    #[doc = "RTCCFG"]
    pub RTCCFG: RTCCFG,
    #[doc = "PWRSEQ"]
    pub PWRSEQ: PWRSEQ,
    #[doc = "IOMAN"]
    pub IOMAN: IOMAN,
    #[doc = "FLC"]
    pub FLC: FLC,
    #[doc = "ICC"]
    pub ICC: ICC,
    #[doc = "SPIX"]
    pub SPIX: SPIX,
    #[doc = "PMU0"]
    pub PMU0: PMU0,
    #[doc = "PMU1"]
    pub PMU1: PMU1,
    #[doc = "PMU2"]
    pub PMU2: PMU2,
    #[doc = "PMU3"]
    pub PMU3: PMU3,
    #[doc = "PMU4"]
    pub PMU4: PMU4,
    #[doc = "PMU5"]
    pub PMU5: PMU5,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "TPU"]
    pub TPU: TPU,
    #[doc = "TPU_TSR"]
    pub TPU_TSR: TPU_TSR,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "MAA"]
    pub MAA: MAA,
    #[doc = "WDT0"]
    pub WDT0: WDT0,
    #[doc = "WDT1"]
    pub WDT1: WDT1,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "TMR0"]
    pub TMR0: TMR0,
    #[doc = "TMR1"]
    pub TMR1: TMR1,
    #[doc = "TMR2"]
    pub TMR2: TMR2,
    #[doc = "TMR3"]
    pub TMR3: TMR3,
    #[doc = "TMR4"]
    pub TMR4: TMR4,
    #[doc = "TMR5"]
    pub TMR5: TMR5,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "PTG"]
    pub PTG: PTG,
    #[doc = "PT0"]
    pub PT0: PT0,
    #[doc = "PT1"]
    pub PT1: PT1,
    #[doc = "PT2"]
    pub PT2: PT2,
    #[doc = "PT3"]
    pub PT3: PT3,
    #[doc = "PT4"]
    pub PT4: PT4,
    #[doc = "PT5"]
    pub PT5: PT5,
    #[doc = "PT6"]
    pub PT6: PT6,
    #[doc = "PT7"]
    pub PT7: PT7,
    #[doc = "PT8"]
    pub PT8: PT8,
    #[doc = "PT9"]
    pub PT9: PT9,
    #[doc = "PT10"]
    pub PT10: PT10,
    #[doc = "PT11"]
    pub PT11: PT11,
    #[doc = "PT12"]
    pub PT12: PT12,
    #[doc = "PT13"]
    pub PT13: PT13,
    #[doc = "PT14"]
    pub PT14: PT14,
    #[doc = "PT15"]
    pub PT15: PT15,
    #[doc = "I2CM0"]
    pub I2CM0: I2CM0,
    #[doc = "I2CM1"]
    pub I2CM1: I2CM1,
    #[doc = "I2CM2"]
    pub I2CM2: I2CM2,
    #[doc = "I2CS"]
    pub I2CS: I2CS,
    #[doc = "SPIM0"]
    pub SPIM0: SPIM0,
    #[doc = "SPIM1"]
    pub SPIM1: SPIM1,
    #[doc = "SPIM2"]
    pub SPIM2: SPIM2,
    #[doc = "OWM"]
    pub OWM: OWM,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "SPIS"]
    pub SPIS: SPIS,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            CLKMAN: CLKMAN {
                _marker: PhantomData,
            },
            PWRMAN: PWRMAN {
                _marker: PhantomData,
            },
            RTCTMR: RTCTMR {
                _marker: PhantomData,
            },
            RTCCFG: RTCCFG {
                _marker: PhantomData,
            },
            PWRSEQ: PWRSEQ {
                _marker: PhantomData,
            },
            IOMAN: IOMAN {
                _marker: PhantomData,
            },
            FLC: FLC {
                _marker: PhantomData,
            },
            ICC: ICC {
                _marker: PhantomData,
            },
            SPIX: SPIX {
                _marker: PhantomData,
            },
            PMU0: PMU0 {
                _marker: PhantomData,
            },
            PMU1: PMU1 {
                _marker: PhantomData,
            },
            PMU2: PMU2 {
                _marker: PhantomData,
            },
            PMU3: PMU3 {
                _marker: PhantomData,
            },
            PMU4: PMU4 {
                _marker: PhantomData,
            },
            PMU5: PMU5 {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            TPU: TPU {
                _marker: PhantomData,
            },
            TPU_TSR: TPU_TSR {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            MAA: MAA {
                _marker: PhantomData,
            },
            WDT0: WDT0 {
                _marker: PhantomData,
            },
            WDT1: WDT1 {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            TMR0: TMR0 {
                _marker: PhantomData,
            },
            TMR1: TMR1 {
                _marker: PhantomData,
            },
            TMR2: TMR2 {
                _marker: PhantomData,
            },
            TMR3: TMR3 {
                _marker: PhantomData,
            },
            TMR4: TMR4 {
                _marker: PhantomData,
            },
            TMR5: TMR5 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            PTG: PTG {
                _marker: PhantomData,
            },
            PT0: PT0 {
                _marker: PhantomData,
            },
            PT1: PT1 {
                _marker: PhantomData,
            },
            PT2: PT2 {
                _marker: PhantomData,
            },
            PT3: PT3 {
                _marker: PhantomData,
            },
            PT4: PT4 {
                _marker: PhantomData,
            },
            PT5: PT5 {
                _marker: PhantomData,
            },
            PT6: PT6 {
                _marker: PhantomData,
            },
            PT7: PT7 {
                _marker: PhantomData,
            },
            PT8: PT8 {
                _marker: PhantomData,
            },
            PT9: PT9 {
                _marker: PhantomData,
            },
            PT10: PT10 {
                _marker: PhantomData,
            },
            PT11: PT11 {
                _marker: PhantomData,
            },
            PT12: PT12 {
                _marker: PhantomData,
            },
            PT13: PT13 {
                _marker: PhantomData,
            },
            PT14: PT14 {
                _marker: PhantomData,
            },
            PT15: PT15 {
                _marker: PhantomData,
            },
            I2CM0: I2CM0 {
                _marker: PhantomData,
            },
            I2CM1: I2CM1 {
                _marker: PhantomData,
            },
            I2CM2: I2CM2 {
                _marker: PhantomData,
            },
            I2CS: I2CS {
                _marker: PhantomData,
            },
            SPIM0: SPIM0 {
                _marker: PhantomData,
            },
            SPIM1: SPIM1 {
                _marker: PhantomData,
            },
            SPIM2: SPIM2 {
                _marker: PhantomData,
            },
            OWM: OWM {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            SPIS: SPIS {
                _marker: PhantomData,
            },
        }
    }
}
