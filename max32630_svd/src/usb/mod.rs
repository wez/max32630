#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Control Register"]
    pub cn: CN,
    _reserved1: [u8; 508usize],
    #[doc = "0x200 - USB Device Address Register"]
    pub dev_addr: DEV_ADDR,
    #[doc = "0x204 - USB Device Control Register"]
    pub dev_cn: DEV_CN,
    #[doc = "0x208 - USB Device Interrupt"]
    pub dev_intfl: DEV_INTFL,
    #[doc = "0x20c - USB Device Interrupt Enable"]
    pub dev_inten: DEV_INTEN,
    _reserved5: [u8; 16usize],
    #[doc = "0x220 - USB Endpoint Descriptor Table Base Address"]
    pub ep_base: EP_BASE,
    #[doc = "0x224 - USB Current Endpoint Buffer Register"]
    pub cur_buf: CUR_BUF,
    #[doc = "0x228 - USB IN Endpoint Buffer Owner Register"]
    pub in_owner: IN_OWNER,
    #[doc = "0x22c - USB OUT Endpoint Buffer Owner Register"]
    pub out_owner: OUT_OWNER,
    #[doc = "0x230 - USB IN Endpoint Buffer Available Interrupt"]
    pub in_int: IN_INT,
    #[doc = "0x234 - USB OUT Endpoint Data Available Interrupt"]
    pub out_int: OUT_INT,
    #[doc = "0x238 - USB IN Endpoint NAK Interrupt"]
    pub nak_int: NAK_INT,
    #[doc = "0x23c - USB DMA Error Interrupt"]
    pub dma_err_int: DMA_ERR_INT,
    #[doc = "0x240 - USB Buffer Overflow Interrupt"]
    pub buf_ovr_int: BUF_OVR_INT,
    _reserved14: [u8; 28usize],
    #[doc = "0x260 - USB SETUP Packet Bytes 0 to 3"]
    pub setup0: SETUP0,
    #[doc = "0x264 - USB SETUP Packet Bytes 4 to 7"]
    pub setup1: SETUP1,
    _reserved16: [u8; 24usize],
    #[doc = "0x280 - USB Endpoint 0 Control Register"]
    pub ep0: EP0,
    #[doc = "0x284 - USB Endpoint 1 Control Register"]
    pub ep1: EP1,
    #[doc = "0x288 - USB Endpoint 2 Control Register"]
    pub ep2: EP2,
    #[doc = "0x28c - USB Endpoint 3 Control Register"]
    pub ep3: EP3,
    #[doc = "0x290 - USB Endpoint 4 Control Register"]
    pub ep4: EP4,
    #[doc = "0x294 - USB Endpoint 5 Control Register"]
    pub ep5: EP5,
    #[doc = "0x298 - USB Endpoint 6 Control Register"]
    pub ep6: EP6,
    #[doc = "0x29c - USB Endpoint 7 Control Register"]
    pub ep7: EP7,
}
#[doc = "USB Control Register"]
pub struct CN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Control Register"]
pub mod cn;
#[doc = "USB Device Address Register"]
pub struct DEV_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device Address Register"]
pub mod dev_addr;
#[doc = "USB Device Control Register"]
pub struct DEV_CN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device Control Register"]
pub mod dev_cn;
#[doc = "USB Device Interrupt"]
pub struct DEV_INTFL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device Interrupt"]
pub mod dev_intfl;
#[doc = "USB Device Interrupt Enable"]
pub struct DEV_INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device Interrupt Enable"]
pub mod dev_inten;
#[doc = "USB Endpoint Descriptor Table Base Address"]
pub struct EP_BASE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint Descriptor Table Base Address"]
pub mod ep_base;
#[doc = "USB Current Endpoint Buffer Register"]
pub struct CUR_BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Current Endpoint Buffer Register"]
pub mod cur_buf;
#[doc = "USB IN Endpoint Buffer Owner Register"]
pub struct IN_OWNER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB IN Endpoint Buffer Owner Register"]
pub mod in_owner;
#[doc = "USB OUT Endpoint Buffer Owner Register"]
pub struct OUT_OWNER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB OUT Endpoint Buffer Owner Register"]
pub mod out_owner;
#[doc = "USB IN Endpoint Buffer Available Interrupt"]
pub struct IN_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB IN Endpoint Buffer Available Interrupt"]
pub mod in_int;
#[doc = "USB OUT Endpoint Data Available Interrupt"]
pub struct OUT_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB OUT Endpoint Data Available Interrupt"]
pub mod out_int;
#[doc = "USB IN Endpoint NAK Interrupt"]
pub struct NAK_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB IN Endpoint NAK Interrupt"]
pub mod nak_int;
#[doc = "USB DMA Error Interrupt"]
pub struct DMA_ERR_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Error Interrupt"]
pub mod dma_err_int;
#[doc = "USB Buffer Overflow Interrupt"]
pub struct BUF_OVR_INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Buffer Overflow Interrupt"]
pub mod buf_ovr_int;
#[doc = "USB SETUP Packet Bytes 0 to 3"]
pub struct SETUP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB SETUP Packet Bytes 0 to 3"]
pub mod setup0;
#[doc = "USB SETUP Packet Bytes 4 to 7"]
pub struct SETUP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB SETUP Packet Bytes 4 to 7"]
pub mod setup1;
#[doc = "USB Endpoint 0 Control Register"]
pub struct EP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint 0 Control Register"]
pub mod ep0;
#[doc = "USB Endpoint 1 Control Register"]
pub struct EP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint 1 Control Register"]
pub mod ep1;
#[doc = "USB Endpoint 2 Control Register"]
pub struct EP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint 2 Control Register"]
pub mod ep2;
#[doc = "USB Endpoint 3 Control Register"]
pub struct EP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint 3 Control Register"]
pub mod ep3;
#[doc = "USB Endpoint 4 Control Register"]
pub struct EP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint 4 Control Register"]
pub mod ep4;
#[doc = "USB Endpoint 5 Control Register"]
pub struct EP5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint 5 Control Register"]
pub mod ep5;
#[doc = "USB Endpoint 6 Control Register"]
pub struct EP6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint 6 Control Register"]
pub mod ep6;
#[doc = "USB Endpoint 7 Control Register"]
pub struct EP7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint 7 Control Register"]
pub mod ep7;
