#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPIX Master Configuration"]
    pub master_cfg: MASTER_CFG,
    #[doc = "0x04 - SPIX Fetch Control"]
    pub fetch_ctrl: FETCH_CTRL,
    #[doc = "0x08 - SPIX Mode Control"]
    pub mode_ctrl: MODE_CTRL,
    #[doc = "0x0c - SPIX Mode Data"]
    pub mode_data: MODE_DATA,
}
#[doc = "SPIX Master Configuration"]
pub struct MASTER_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIX Master Configuration"]
pub mod master_cfg;
#[doc = "SPIX Fetch Control"]
pub struct FETCH_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIX Fetch Control"]
pub mod fetch_ctrl;
#[doc = "SPIX Mode Control"]
pub struct MODE_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIX Mode Control"]
pub mod mode_ctrl;
#[doc = "SPIX Mode Data"]
pub struct MODE_DATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPIX Mode Data"]
pub mod mode_data;
