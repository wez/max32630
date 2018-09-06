#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Wakeup Detect Mode Request Register 0 (P0/P1/P2/P3)"]
    pub wud_req0: WUD_REQ0,
    #[doc = "0x04 - Wakeup Detect Mode Request Register 1 (P4/P5/P6)"]
    pub wud_req1: WUD_REQ1,
    #[doc = "0x08 - Wakeup Detect Mode Acknowledge Register 0 (P0/P1/P2/P3)"]
    pub wud_ack0: WUD_ACK0,
    #[doc = "0x0c - Wakeup Detect Mode Acknowledge Register 1 (P4/P5/P6)"]
    pub wud_ack1: WUD_ACK1,
    #[doc = "0x10 - Analog Input Request Register 0 (P0/P1/P2/P3)"]
    pub ali_req0: ALI_REQ0,
    #[doc = "0x14 - Analog Input Request Register 1 (P4/P5/P6)"]
    pub ali_req1: ALI_REQ1,
    #[doc = "0x18 - Analog Input Acknowledge Register 0 (P0/P1/P2/P3)"]
    pub ali_ack0: ALI_ACK0,
    #[doc = "0x1c - Analog Input Acknowledge Register 1 (P4/P5/P6)"]
    pub ali_ack1: ALI_ACK1,
    #[doc = "0x20 - Analog I/O Connection Control Register 0"]
    pub ali_connect0: ALI_CONNECT0,
    #[doc = "0x24 - Analog I/O Connection Control Register 1"]
    pub ali_connect1: ALI_CONNECT1,
    #[doc = "0x28 - SPIX I/O Mode Request"]
    pub spix_req: SPIX_REQ,
    #[doc = "0x2c - SPIX I/O Mode Acknowledge"]
    pub spix_ack: SPIX_ACK,
    #[doc = "0x30 - UART0 I/O Mode Request"]
    pub uart0_req: UART0_REQ,
    #[doc = "0x34 - UART0 I/O Mode Acknowledge"]
    pub uart0_ack: UART0_ACK,
    #[doc = "0x38 - UART1 I/O Mode Request"]
    pub uart1_req: UART1_REQ,
    #[doc = "0x3c - UART1 I/O Mode Acknowledge"]
    pub uart1_ack: UART1_ACK,
    #[doc = "0x40 - UART2 I/O Mode Request"]
    pub uart2_req: UART2_REQ,
    #[doc = "0x44 - UART2 I/O Mode Acknowledge"]
    pub uart2_ack: UART2_ACK,
    #[doc = "0x48 - UART3 I/O Mode Request"]
    pub uart3_req: UART3_REQ,
    #[doc = "0x4c - UART3 I/O Mode Acknowledge"]
    pub uart3_ack: UART3_ACK,
    #[doc = "0x50 - I2C Master 0 I/O Request"]
    pub i2cm0_req: I2CM0_REQ,
    #[doc = "0x54 - I2C Master 0 I/O Acknowledge"]
    pub i2cm0_ack: I2CM0_ACK,
    #[doc = "0x58 - I2C Master 1 I/O Request"]
    pub i2cm1_req: I2CM1_REQ,
    #[doc = "0x5c - I2C Master 1 I/O Acknowledge"]
    pub i2cm1_ack: I2CM1_ACK,
    #[doc = "0x60 - I2C Master 2 I/O Request"]
    pub i2cm2_req: I2CM2_REQ,
    #[doc = "0x64 - I2C Master 2 I/O Acknowledge"]
    pub i2cm2_ack: I2CM2_ACK,
    #[doc = "0x68 - I2C Slave I/O Request"]
    pub i2cs_req: I2CS_REQ,
    #[doc = "0x6c - I2C Slave I/O Acknowledge"]
    pub i2cs_ack: I2CS_ACK,
    #[doc = "0x70 - SPI Master 0 I/O Mode Request"]
    pub spi0_req: SPI0_REQ,
    #[doc = "0x74 - SPI Master 0 I/O Mode Acknowledge"]
    pub spi0_ack: SPI0_ACK,
    #[doc = "0x78 - SPI Master 1 I/O Mode Request"]
    pub spi1_req: SPI1_REQ,
    #[doc = "0x7c - SPI Master 1 I/O Mode Acknowledge"]
    pub spi1_ack: SPI1_ACK,
    #[doc = "0x80 - SPI Master 2 I/O Mode Request"]
    pub spi2_req: SPI2_REQ,
    #[doc = "0x84 - SPI Master 2 I/O Mode Acknowledge"]
    pub spi2_ack: SPI2_ACK,
    #[doc = "0x88 - SPI Bridge I/O Mode Request"]
    pub spib_req: SPIB_REQ,
    #[doc = "0x8c - SPI Bridge I/O Mode Acknowledge"]
    pub spib_ack: SPIB_ACK,
    #[doc = "0x90 - 1-Wire Master I/O Mode Request"]
    pub owm_req: OWM_REQ,
    #[doc = "0x94 - 1-Wire Master I/O Mode Acknowledge"]
    pub owm_ack: OWM_ACK,
}
#[doc = "Wakeup Detect Mode Request Register 0 (P0/P1/P2/P3)"]
pub struct WUD_REQ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wakeup Detect Mode Request Register 0 (P0/P1/P2/P3)"]
pub mod wud_req0;
#[doc = "Wakeup Detect Mode Request Register 1 (P4/P5/P6)"]
pub struct WUD_REQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wakeup Detect Mode Request Register 1 (P4/P5/P6)"]
pub mod wud_req1;
#[doc = "Wakeup Detect Mode Acknowledge Register 0 (P0/P1/P2/P3)"]
pub struct WUD_ACK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wakeup Detect Mode Acknowledge Register 0 (P0/P1/P2/P3)"]
pub mod wud_ack0;
#[doc = "Wakeup Detect Mode Acknowledge Register 1 (P4/P5/P6)"]
pub struct WUD_ACK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wakeup Detect Mode Acknowledge Register 1 (P4/P5/P6)"]
pub mod wud_ack1;
#[doc = "Analog Input Request Register 0 (P0/P1/P2/P3)"]
pub struct ALI_REQ0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Input Request Register 0 (P0/P1/P2/P3)"]
pub mod ali_req0;
#[doc = "Analog Input Request Register 1 (P4/P5/P6)"]
pub struct ALI_REQ1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Input Request Register 1 (P4/P5/P6)"]
pub mod ali_req1;
#[doc = "Analog Input Acknowledge Register 0 (P0/P1/P2/P3)"]
pub struct ALI_ACK0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Input Acknowledge Register 0 (P0/P1/P2/P3)"]
pub mod ali_ack0;
#[doc = "Analog Input Acknowledge Register 1 (P4/P5/P6)"]
pub struct ALI_ACK1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Input Acknowledge Register 1 (P4/P5/P6)"]
pub mod ali_ack1;
#[doc = "Analog I/O Connection Control Register 0"]
pub struct ALI_CONNECT0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog I/O Connection Control Register 0"]
pub mod ali_connect0;
#[doc = "Analog I/O Connection Control Register 1"]
pub struct ALI_CONNECT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog I/O Connection Control Register 1"]
pub mod ali_connect1;
#[doc = "SPIX I/O Mode Request"]
pub struct SPIX_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIX I/O Mode Request"]
pub mod spix_req;
#[doc = "SPIX I/O Mode Acknowledge"]
pub struct SPIX_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIX I/O Mode Acknowledge"]
pub mod spix_ack;
#[doc = "UART0 I/O Mode Request"]
pub struct UART0_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART0 I/O Mode Request"]
pub mod uart0_req;
#[doc = "UART0 I/O Mode Acknowledge"]
pub struct UART0_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART0 I/O Mode Acknowledge"]
pub mod uart0_ack;
#[doc = "UART1 I/O Mode Request"]
pub struct UART1_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART1 I/O Mode Request"]
pub mod uart1_req;
#[doc = "UART1 I/O Mode Acknowledge"]
pub struct UART1_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART1 I/O Mode Acknowledge"]
pub mod uart1_ack;
#[doc = "UART2 I/O Mode Request"]
pub struct UART2_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART2 I/O Mode Request"]
pub mod uart2_req;
#[doc = "UART2 I/O Mode Acknowledge"]
pub struct UART2_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART2 I/O Mode Acknowledge"]
pub mod uart2_ack;
#[doc = "UART3 I/O Mode Request"]
pub struct UART3_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART3 I/O Mode Request"]
pub mod uart3_req;
#[doc = "UART3 I/O Mode Acknowledge"]
pub struct UART3_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UART3 I/O Mode Acknowledge"]
pub mod uart3_ack;
#[doc = "I2C Master 0 I/O Request"]
pub struct I2CM0_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master 0 I/O Request"]
pub mod i2cm0_req;
#[doc = "I2C Master 0 I/O Acknowledge"]
pub struct I2CM0_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master 0 I/O Acknowledge"]
pub mod i2cm0_ack;
#[doc = "I2C Master 1 I/O Request"]
pub struct I2CM1_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master 1 I/O Request"]
pub mod i2cm1_req;
#[doc = "I2C Master 1 I/O Acknowledge"]
pub struct I2CM1_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master 1 I/O Acknowledge"]
pub mod i2cm1_ack;
#[doc = "I2C Master 2 I/O Request"]
pub struct I2CM2_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master 2 I/O Request"]
pub mod i2cm2_req;
#[doc = "I2C Master 2 I/O Acknowledge"]
pub struct I2CM2_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Master 2 I/O Acknowledge"]
pub mod i2cm2_ack;
#[doc = "I2C Slave I/O Request"]
pub struct I2CS_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave I/O Request"]
pub mod i2cs_req;
#[doc = "I2C Slave I/O Acknowledge"]
pub struct I2CS_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Slave I/O Acknowledge"]
pub mod i2cs_ack;
#[doc = "SPI Master 0 I/O Mode Request"]
pub struct SPI0_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master 0 I/O Mode Request"]
pub mod spi0_req;
#[doc = "SPI Master 0 I/O Mode Acknowledge"]
pub struct SPI0_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master 0 I/O Mode Acknowledge"]
pub mod spi0_ack;
#[doc = "SPI Master 1 I/O Mode Request"]
pub struct SPI1_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master 1 I/O Mode Request"]
pub mod spi1_req;
#[doc = "SPI Master 1 I/O Mode Acknowledge"]
pub struct SPI1_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master 1 I/O Mode Acknowledge"]
pub mod spi1_ack;
#[doc = "SPI Master 2 I/O Mode Request"]
pub struct SPI2_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master 2 I/O Mode Request"]
pub mod spi2_req;
#[doc = "SPI Master 2 I/O Mode Acknowledge"]
pub struct SPI2_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Master 2 I/O Mode Acknowledge"]
pub mod spi2_ack;
#[doc = "SPI Bridge I/O Mode Request"]
pub struct SPIB_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Bridge I/O Mode Request"]
pub mod spib_req;
#[doc = "SPI Bridge I/O Mode Acknowledge"]
pub struct SPIB_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI Bridge I/O Mode Acknowledge"]
pub mod spib_ack;
#[doc = "1-Wire Master I/O Mode Request"]
pub struct OWM_REQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1-Wire Master I/O Mode Request"]
pub mod owm_req;
#[doc = "1-Wire Master I/O Mode Acknowledge"]
pub struct OWM_ACK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1-Wire Master I/O Mode Acknowledge"]
pub mod owm_ack;
