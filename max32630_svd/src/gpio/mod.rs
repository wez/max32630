#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port P0 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p0: RST_MODE_P0,
    #[doc = "0x04 - Port P1 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p1: RST_MODE_P1,
    #[doc = "0x08 - Port P2 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p2: RST_MODE_P2,
    #[doc = "0x0c - Port P3 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p3: RST_MODE_P3,
    #[doc = "0x10 - Port P4 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p4: RST_MODE_P4,
    #[doc = "0x14 - Port P5 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p5: RST_MODE_P5,
    #[doc = "0x18 - Port P6 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p6: RST_MODE_P6,
    #[doc = "0x1c - Port P7 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p7: RST_MODE_P7,
    #[doc = "0x20 - Port P8 Default (Power-On Reset) Output Drive Mode"]
    pub rst_mode_p8: RST_MODE_P8,
    _reserved9: [u8; 28usize],
    #[doc = "0x40 - Port P0 Free for GPIO Operation Flags"]
    pub free_p0: FREE_P0,
    #[doc = "0x44 - Port P1 Free for GPIO Operation Flags"]
    pub free_p1: FREE_P1,
    #[doc = "0x48 - Port P2 Free for GPIO Operation Flags"]
    pub free_p2: FREE_P2,
    #[doc = "0x4c - Port P3 Free for GPIO Operation Flags"]
    pub free_p3: FREE_P3,
    #[doc = "0x50 - Port P4 Free for GPIO Operation Flags"]
    pub free_p4: FREE_P4,
    #[doc = "0x54 - Port P5 Free for GPIO Operation Flags"]
    pub free_p5: FREE_P5,
    #[doc = "0x58 - Port P6 Free for GPIO Operation Flags"]
    pub free_p6: FREE_P6,
    #[doc = "0x5c - Port P7 Free for GPIO Operation Flags"]
    pub free_p7: FREE_P7,
    #[doc = "0x60 - Port P8 Free for GPIO Operation Flags"]
    pub free_p8: FREE_P8,
    _reserved18: [u8; 28usize],
    #[doc = "0x80 - Port P0 GPIO Output Drive Mode"]
    pub out_mode_p0: OUT_MODE_P0,
    #[doc = "0x84 - Port P1 GPIO Output Drive Mode"]
    pub out_mode_p1: OUT_MODE_P1,
    #[doc = "0x88 - Port P2 GPIO Output Drive Mode"]
    pub out_mode_p2: OUT_MODE_P2,
    #[doc = "0x8c - Port P3 GPIO Output Drive Mode"]
    pub out_mode_p3: OUT_MODE_P3,
    #[doc = "0x90 - Port P4 GPIO Output Drive Mode"]
    pub out_mode_p4: OUT_MODE_P4,
    #[doc = "0x94 - Port P5 GPIO Output Drive Mode"]
    pub out_mode_p5: OUT_MODE_P5,
    #[doc = "0x98 - Port P6 GPIO Output Drive Mode"]
    pub out_mode_p6: OUT_MODE_P6,
    #[doc = "0x9c - Port P7 GPIO Output Drive Mode"]
    pub out_mode_p7: OUT_MODE_P7,
    #[doc = "0xa0 - Port P8 GPIO Output Drive Mode"]
    pub out_mode_p8: OUT_MODE_P8,
    _reserved27: [u8; 28usize],
    #[doc = "0xc0 - Port P0 GPIO Output Value"]
    pub out_val_p0: OUT_VAL_P0,
    #[doc = "0xc4 - Port P1 GPIO Output Value"]
    pub out_val_p1: OUT_VAL_P1,
    #[doc = "0xc8 - Port P2 GPIO Output Value"]
    pub out_val_p2: OUT_VAL_P2,
    #[doc = "0xcc - Port P3 GPIO Output Value"]
    pub out_val_p3: OUT_VAL_P3,
    #[doc = "0xd0 - Port P4 GPIO Output Value"]
    pub out_val_p4: OUT_VAL_P4,
    #[doc = "0xd4 - Port P5 GPIO Output Value"]
    pub out_val_p5: OUT_VAL_P5,
    #[doc = "0xd8 - Port P6 GPIO Output Value"]
    pub out_val_p6: OUT_VAL_P6,
    #[doc = "0xdc - Port P7 GPIO Output Value"]
    pub out_val_p7: OUT_VAL_P7,
    #[doc = "0xe0 - Port P8 GPIO Output Value"]
    pub out_val_p8: OUT_VAL_P8,
    _reserved36: [u8; 28usize],
    #[doc = "0x100 - Port P0 GPIO Function Select"]
    pub func_sel_p0: FUNC_SEL_P0,
    #[doc = "0x104 - Port P1 GPIO Function Select"]
    pub func_sel_p1: FUNC_SEL_P1,
    #[doc = "0x108 - Port P2 GPIO Function Select"]
    pub func_sel_p2: FUNC_SEL_P2,
    #[doc = "0x10c - Port P3 GPIO Function Select"]
    pub func_sel_p3: FUNC_SEL_P3,
    #[doc = "0x110 - Port P4 GPIO Function Select"]
    pub func_sel_p4: FUNC_SEL_P4,
    #[doc = "0x114 - Port P5 GPIO Function Select"]
    pub func_sel_p5: FUNC_SEL_P5,
    #[doc = "0x118 - Port P6 GPIO Function Select"]
    pub func_sel_p6: FUNC_SEL_P6,
    #[doc = "0x11c - Port P7 GPIO Function Select"]
    pub func_sel_p7: FUNC_SEL_P7,
    #[doc = "0x120 - Port P8 GPIO Function Select"]
    pub func_sel_p8: FUNC_SEL_P8,
    _reserved45: [u8; 28usize],
    #[doc = "0x140 - Port P0 GPIO Input Monitoring Mode"]
    pub in_mode_p0: IN_MODE_P0,
    #[doc = "0x144 - Port P1 GPIO Input Monitoring Mode"]
    pub in_mode_p1: IN_MODE_P1,
    #[doc = "0x148 - Port P2 GPIO Input Monitoring Mode"]
    pub in_mode_p2: IN_MODE_P2,
    #[doc = "0x14c - Port P3 GPIO Input Monitoring Mode"]
    pub in_mode_p3: IN_MODE_P3,
    #[doc = "0x150 - Port P4 GPIO Input Monitoring Mode"]
    pub in_mode_p4: IN_MODE_P4,
    #[doc = "0x154 - Port P5 GPIO Input Monitoring Mode"]
    pub in_mode_p5: IN_MODE_P5,
    #[doc = "0x158 - Port P6 GPIO Input Monitoring Mode"]
    pub in_mode_p6: IN_MODE_P6,
    #[doc = "0x15c - Port P7 GPIO Input Monitoring Mode"]
    pub in_mode_p7: IN_MODE_P7,
    #[doc = "0x160 - Port P8 GPIO Input Monitoring Mode"]
    pub in_mode_p8: IN_MODE_P8,
    _reserved54: [u8; 28usize],
    #[doc = "0x180 - Port P0 GPIO Input Value"]
    pub in_val_p0: IN_VAL_P0,
    #[doc = "0x184 - Port P1 GPIO Input Value"]
    pub in_val_p1: IN_VAL_P1,
    #[doc = "0x188 - Port P2 GPIO Input Value"]
    pub in_val_p2: IN_VAL_P2,
    #[doc = "0x18c - Port P3 GPIO Input Value"]
    pub in_val_p3: IN_VAL_P3,
    #[doc = "0x190 - Port P4 GPIO Input Value"]
    pub in_val_p4: IN_VAL_P4,
    #[doc = "0x194 - Port P5 GPIO Input Value"]
    pub in_val_p5: IN_VAL_P5,
    #[doc = "0x198 - Port P6 GPIO Input Value"]
    pub in_val_p6: IN_VAL_P6,
    #[doc = "0x19c - Port P7 GPIO Input Value"]
    pub in_val_p7: IN_VAL_P7,
    #[doc = "0x1a0 - Port P8 GPIO Input Value"]
    pub in_val_p8: IN_VAL_P8,
    _reserved63: [u8; 28usize],
    #[doc = "0x1c0 - Port P0 Interrupt Detection Mode"]
    pub int_mode_p0: INT_MODE_P0,
    #[doc = "0x1c4 - Port P1 Interrupt Detection Mode"]
    pub int_mode_p1: INT_MODE_P1,
    #[doc = "0x1c8 - Port P2 Interrupt Detection Mode"]
    pub int_mode_p2: INT_MODE_P2,
    #[doc = "0x1cc - Port P3 Interrupt Detection Mode"]
    pub int_mode_p3: INT_MODE_P3,
    #[doc = "0x1d0 - Port P4 Interrupt Detection Mode"]
    pub int_mode_p4: INT_MODE_P4,
    #[doc = "0x1d4 - Port P5 Interrupt Detection Mode"]
    pub int_mode_p5: INT_MODE_P5,
    #[doc = "0x1d8 - Port P6 Interrupt Detection Mode"]
    pub int_mode_p6: INT_MODE_P6,
    #[doc = "0x1dc - Port P7 Interrupt Detection Mode"]
    pub int_mode_p7: INT_MODE_P7,
    #[doc = "0x1e0 - Port P8 Interrupt Detection Mode"]
    pub int_mode_p8: INT_MODE_P8,
    _reserved72: [u8; 28usize],
    #[doc = "0x200 - Port P0 Interrupt Flags"]
    pub intfl_p0: INTFL_P0,
    #[doc = "0x204 - Port P1 Interrupt Flags"]
    pub intfl_p1: INTFL_P1,
    #[doc = "0x208 - Port P2 Interrupt Flags"]
    pub intfl_p2: INTFL_P2,
    #[doc = "0x20c - Port P3 Interrupt Flags"]
    pub intfl_p3: INTFL_P3,
    #[doc = "0x210 - Port P4 Interrupt Flags"]
    pub intfl_p4: INTFL_P4,
    #[doc = "0x214 - Port P5 Interrupt Flags"]
    pub intfl_p5: INTFL_P5,
    #[doc = "0x218 - Port P6 Interrupt Flags"]
    pub intfl_p6: INTFL_P6,
    #[doc = "0x21c - Port P7 Interrupt Flags"]
    pub intfl_p7: INTFL_P7,
    #[doc = "0x220 - Port P8 Interrupt Flags"]
    pub intfl_p8: INTFL_P8,
    _reserved81: [u8; 28usize],
    #[doc = "0x240 - Port P0 Interrupt Enables"]
    pub inten_p0: INTEN_P0,
    #[doc = "0x244 - Port P1 Interrupt Enables"]
    pub inten_p1: INTEN_P1,
    #[doc = "0x248 - Port P2 Interrupt Enables"]
    pub inten_p2: INTEN_P2,
    #[doc = "0x24c - Port P3 Interrupt Enables"]
    pub inten_p3: INTEN_P3,
    #[doc = "0x250 - Port P4 Interrupt Enables"]
    pub inten_p4: INTEN_P4,
    #[doc = "0x254 - Port P5 Interrupt Enables"]
    pub inten_p5: INTEN_P5,
    #[doc = "0x258 - Port P6 Interrupt Enables"]
    pub inten_p6: INTEN_P6,
    #[doc = "0x25c - Port P7 Interrupt Enables"]
    pub inten_p7: INTEN_P7,
    #[doc = "0x260 - Port P8 Interrupt Enables"]
    pub inten_p8: INTEN_P8,
}
#[doc = "Port P0 Default (Power-On Reset) Output Drive Mode"]
pub struct RST_MODE_P0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P0 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p0;
#[doc = "Port P1 Default (Power-On Reset) Output Drive Mode"]
pub struct RST_MODE_P1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P1 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p1;
#[doc = "Port P2 Default (Power-On Reset) Output Drive Mode"]
pub struct RST_MODE_P2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P2 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p2;
#[doc = "Port P3 Default (Power-On Reset) Output Drive Mode"]
pub struct RST_MODE_P3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P3 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p3;
#[doc = "Port P4 Default (Power-On Reset) Output Drive Mode"]
pub struct RST_MODE_P4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P4 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p4;
#[doc = "Port P5 Default (Power-On Reset) Output Drive Mode"]
pub struct RST_MODE_P5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P5 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p5;
#[doc = "Port P6 Default (Power-On Reset) Output Drive Mode"]
pub struct RST_MODE_P6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P6 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p6;
#[doc = "Port P7 Default (Power-On Reset) Output Drive Mode"]
pub struct RST_MODE_P7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P7 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p7;
#[doc = "Port P8 Default (Power-On Reset) Output Drive Mode"]
pub struct RST_MODE_P8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P8 Default (Power-On Reset) Output Drive Mode"]
pub mod rst_mode_p8;
#[doc = "Port P0 Free for GPIO Operation Flags"]
pub struct FREE_P0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P0 Free for GPIO Operation Flags"]
pub mod free_p0;
#[doc = "Port P1 Free for GPIO Operation Flags"]
pub struct FREE_P1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P1 Free for GPIO Operation Flags"]
pub mod free_p1;
#[doc = "Port P2 Free for GPIO Operation Flags"]
pub struct FREE_P2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P2 Free for GPIO Operation Flags"]
pub mod free_p2;
#[doc = "Port P3 Free for GPIO Operation Flags"]
pub struct FREE_P3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P3 Free for GPIO Operation Flags"]
pub mod free_p3;
#[doc = "Port P4 Free for GPIO Operation Flags"]
pub struct FREE_P4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P4 Free for GPIO Operation Flags"]
pub mod free_p4;
#[doc = "Port P5 Free for GPIO Operation Flags"]
pub struct FREE_P5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P5 Free for GPIO Operation Flags"]
pub mod free_p5;
#[doc = "Port P6 Free for GPIO Operation Flags"]
pub struct FREE_P6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P6 Free for GPIO Operation Flags"]
pub mod free_p6;
#[doc = "Port P7 Free for GPIO Operation Flags"]
pub struct FREE_P7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P7 Free for GPIO Operation Flags"]
pub mod free_p7;
#[doc = "Port P8 Free for GPIO Operation Flags"]
pub struct FREE_P8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P8 Free for GPIO Operation Flags"]
pub mod free_p8;
#[doc = "Port P0 GPIO Output Drive Mode"]
pub struct OUT_MODE_P0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P0 GPIO Output Drive Mode"]
pub mod out_mode_p0;
#[doc = "Port P1 GPIO Output Drive Mode"]
pub struct OUT_MODE_P1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P1 GPIO Output Drive Mode"]
pub mod out_mode_p1;
#[doc = "Port P2 GPIO Output Drive Mode"]
pub struct OUT_MODE_P2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P2 GPIO Output Drive Mode"]
pub mod out_mode_p2;
#[doc = "Port P3 GPIO Output Drive Mode"]
pub struct OUT_MODE_P3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P3 GPIO Output Drive Mode"]
pub mod out_mode_p3;
#[doc = "Port P4 GPIO Output Drive Mode"]
pub struct OUT_MODE_P4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P4 GPIO Output Drive Mode"]
pub mod out_mode_p4;
#[doc = "Port P5 GPIO Output Drive Mode"]
pub struct OUT_MODE_P5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P5 GPIO Output Drive Mode"]
pub mod out_mode_p5;
#[doc = "Port P6 GPIO Output Drive Mode"]
pub struct OUT_MODE_P6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P6 GPIO Output Drive Mode"]
pub mod out_mode_p6;
#[doc = "Port P7 GPIO Output Drive Mode"]
pub struct OUT_MODE_P7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P7 GPIO Output Drive Mode"]
pub mod out_mode_p7;
#[doc = "Port P8 GPIO Output Drive Mode"]
pub struct OUT_MODE_P8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P8 GPIO Output Drive Mode"]
pub mod out_mode_p8;
#[doc = "Port P0 GPIO Output Value"]
pub struct OUT_VAL_P0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P0 GPIO Output Value"]
pub mod out_val_p0;
#[doc = "Port P1 GPIO Output Value"]
pub struct OUT_VAL_P1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P1 GPIO Output Value"]
pub mod out_val_p1;
#[doc = "Port P2 GPIO Output Value"]
pub struct OUT_VAL_P2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P2 GPIO Output Value"]
pub mod out_val_p2;
#[doc = "Port P3 GPIO Output Value"]
pub struct OUT_VAL_P3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P3 GPIO Output Value"]
pub mod out_val_p3;
#[doc = "Port P4 GPIO Output Value"]
pub struct OUT_VAL_P4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P4 GPIO Output Value"]
pub mod out_val_p4;
#[doc = "Port P5 GPIO Output Value"]
pub struct OUT_VAL_P5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P5 GPIO Output Value"]
pub mod out_val_p5;
#[doc = "Port P6 GPIO Output Value"]
pub struct OUT_VAL_P6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P6 GPIO Output Value"]
pub mod out_val_p6;
#[doc = "Port P7 GPIO Output Value"]
pub struct OUT_VAL_P7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P7 GPIO Output Value"]
pub mod out_val_p7;
#[doc = "Port P8 GPIO Output Value"]
pub struct OUT_VAL_P8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P8 GPIO Output Value"]
pub mod out_val_p8;
#[doc = "Port P0 GPIO Function Select"]
pub struct FUNC_SEL_P0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P0 GPIO Function Select"]
pub mod func_sel_p0;
#[doc = "Port P1 GPIO Function Select"]
pub struct FUNC_SEL_P1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P1 GPIO Function Select"]
pub mod func_sel_p1;
#[doc = "Port P2 GPIO Function Select"]
pub struct FUNC_SEL_P2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P2 GPIO Function Select"]
pub mod func_sel_p2;
#[doc = "Port P3 GPIO Function Select"]
pub struct FUNC_SEL_P3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P3 GPIO Function Select"]
pub mod func_sel_p3;
#[doc = "Port P4 GPIO Function Select"]
pub struct FUNC_SEL_P4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P4 GPIO Function Select"]
pub mod func_sel_p4;
#[doc = "Port P5 GPIO Function Select"]
pub struct FUNC_SEL_P5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P5 GPIO Function Select"]
pub mod func_sel_p5;
#[doc = "Port P6 GPIO Function Select"]
pub struct FUNC_SEL_P6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P6 GPIO Function Select"]
pub mod func_sel_p6;
#[doc = "Port P7 GPIO Function Select"]
pub struct FUNC_SEL_P7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P7 GPIO Function Select"]
pub mod func_sel_p7;
#[doc = "Port P8 GPIO Function Select"]
pub struct FUNC_SEL_P8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P8 GPIO Function Select"]
pub mod func_sel_p8;
#[doc = "Port P0 GPIO Input Monitoring Mode"]
pub struct IN_MODE_P0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P0 GPIO Input Monitoring Mode"]
pub mod in_mode_p0;
#[doc = "Port P1 GPIO Input Monitoring Mode"]
pub struct IN_MODE_P1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P1 GPIO Input Monitoring Mode"]
pub mod in_mode_p1;
#[doc = "Port P2 GPIO Input Monitoring Mode"]
pub struct IN_MODE_P2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P2 GPIO Input Monitoring Mode"]
pub mod in_mode_p2;
#[doc = "Port P3 GPIO Input Monitoring Mode"]
pub struct IN_MODE_P3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P3 GPIO Input Monitoring Mode"]
pub mod in_mode_p3;
#[doc = "Port P4 GPIO Input Monitoring Mode"]
pub struct IN_MODE_P4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P4 GPIO Input Monitoring Mode"]
pub mod in_mode_p4;
#[doc = "Port P5 GPIO Input Monitoring Mode"]
pub struct IN_MODE_P5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P5 GPIO Input Monitoring Mode"]
pub mod in_mode_p5;
#[doc = "Port P6 GPIO Input Monitoring Mode"]
pub struct IN_MODE_P6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P6 GPIO Input Monitoring Mode"]
pub mod in_mode_p6;
#[doc = "Port P7 GPIO Input Monitoring Mode"]
pub struct IN_MODE_P7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P7 GPIO Input Monitoring Mode"]
pub mod in_mode_p7;
#[doc = "Port P8 GPIO Input Monitoring Mode"]
pub struct IN_MODE_P8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P8 GPIO Input Monitoring Mode"]
pub mod in_mode_p8;
#[doc = "Port P0 GPIO Input Value"]
pub struct IN_VAL_P0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P0 GPIO Input Value"]
pub mod in_val_p0;
#[doc = "Port P1 GPIO Input Value"]
pub struct IN_VAL_P1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P1 GPIO Input Value"]
pub mod in_val_p1;
#[doc = "Port P2 GPIO Input Value"]
pub struct IN_VAL_P2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P2 GPIO Input Value"]
pub mod in_val_p2;
#[doc = "Port P3 GPIO Input Value"]
pub struct IN_VAL_P3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P3 GPIO Input Value"]
pub mod in_val_p3;
#[doc = "Port P4 GPIO Input Value"]
pub struct IN_VAL_P4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P4 GPIO Input Value"]
pub mod in_val_p4;
#[doc = "Port P5 GPIO Input Value"]
pub struct IN_VAL_P5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P5 GPIO Input Value"]
pub mod in_val_p5;
#[doc = "Port P6 GPIO Input Value"]
pub struct IN_VAL_P6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P6 GPIO Input Value"]
pub mod in_val_p6;
#[doc = "Port P7 GPIO Input Value"]
pub struct IN_VAL_P7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P7 GPIO Input Value"]
pub mod in_val_p7;
#[doc = "Port P8 GPIO Input Value"]
pub struct IN_VAL_P8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P8 GPIO Input Value"]
pub mod in_val_p8;
#[doc = "Port P0 Interrupt Detection Mode"]
pub struct INT_MODE_P0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P0 Interrupt Detection Mode"]
pub mod int_mode_p0;
#[doc = "Port P1 Interrupt Detection Mode"]
pub struct INT_MODE_P1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P1 Interrupt Detection Mode"]
pub mod int_mode_p1;
#[doc = "Port P2 Interrupt Detection Mode"]
pub struct INT_MODE_P2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P2 Interrupt Detection Mode"]
pub mod int_mode_p2;
#[doc = "Port P3 Interrupt Detection Mode"]
pub struct INT_MODE_P3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P3 Interrupt Detection Mode"]
pub mod int_mode_p3;
#[doc = "Port P4 Interrupt Detection Mode"]
pub struct INT_MODE_P4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P4 Interrupt Detection Mode"]
pub mod int_mode_p4;
#[doc = "Port P5 Interrupt Detection Mode"]
pub struct INT_MODE_P5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P5 Interrupt Detection Mode"]
pub mod int_mode_p5;
#[doc = "Port P6 Interrupt Detection Mode"]
pub struct INT_MODE_P6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P6 Interrupt Detection Mode"]
pub mod int_mode_p6;
#[doc = "Port P7 Interrupt Detection Mode"]
pub struct INT_MODE_P7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P7 Interrupt Detection Mode"]
pub mod int_mode_p7;
#[doc = "Port P8 Interrupt Detection Mode"]
pub struct INT_MODE_P8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P8 Interrupt Detection Mode"]
pub mod int_mode_p8;
#[doc = "Port P0 Interrupt Flags"]
pub struct INTFL_P0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P0 Interrupt Flags"]
pub mod intfl_p0;
#[doc = "Port P1 Interrupt Flags"]
pub struct INTFL_P1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P1 Interrupt Flags"]
pub mod intfl_p1;
#[doc = "Port P2 Interrupt Flags"]
pub struct INTFL_P2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P2 Interrupt Flags"]
pub mod intfl_p2;
#[doc = "Port P3 Interrupt Flags"]
pub struct INTFL_P3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P3 Interrupt Flags"]
pub mod intfl_p3;
#[doc = "Port P4 Interrupt Flags"]
pub struct INTFL_P4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P4 Interrupt Flags"]
pub mod intfl_p4;
#[doc = "Port P5 Interrupt Flags"]
pub struct INTFL_P5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P5 Interrupt Flags"]
pub mod intfl_p5;
#[doc = "Port P6 Interrupt Flags"]
pub struct INTFL_P6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P6 Interrupt Flags"]
pub mod intfl_p6;
#[doc = "Port P7 Interrupt Flags"]
pub struct INTFL_P7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P7 Interrupt Flags"]
pub mod intfl_p7;
#[doc = "Port P8 Interrupt Flags"]
pub struct INTFL_P8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P8 Interrupt Flags"]
pub mod intfl_p8;
#[doc = "Port P0 Interrupt Enables"]
pub struct INTEN_P0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P0 Interrupt Enables"]
pub mod inten_p0;
#[doc = "Port P1 Interrupt Enables"]
pub struct INTEN_P1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P1 Interrupt Enables"]
pub mod inten_p1;
#[doc = "Port P2 Interrupt Enables"]
pub struct INTEN_P2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P2 Interrupt Enables"]
pub mod inten_p2;
#[doc = "Port P3 Interrupt Enables"]
pub struct INTEN_P3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P3 Interrupt Enables"]
pub mod inten_p3;
#[doc = "Port P4 Interrupt Enables"]
pub struct INTEN_P4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P4 Interrupt Enables"]
pub mod inten_p4;
#[doc = "Port P5 Interrupt Enables"]
pub struct INTEN_P5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P5 Interrupt Enables"]
pub mod inten_p5;
#[doc = "Port P6 Interrupt Enables"]
pub struct INTEN_P6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P6 Interrupt Enables"]
pub mod inten_p6;
#[doc = "Port P7 Interrupt Enables"]
pub struct INTEN_P7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P7 Interrupt Enables"]
pub mod inten_p7;
#[doc = "Port P8 Interrupt Enables"]
pub struct INTEN_P8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port P8 Interrupt Enables"]
pub mod inten_p8;
