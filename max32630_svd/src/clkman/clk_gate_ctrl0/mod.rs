#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLK_GATE_CTRL0 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `cm4_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CM4_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CM4_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CM4_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CM4_CLK_GATERR {
        match value {
            i => CM4_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ahb32_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB32_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AHB32_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AHB32_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AHB32_CLK_GATERR {
        match value {
            i => AHB32_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `icache_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICACHE_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ICACHE_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ICACHE_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ICACHE_CLK_GATERR {
        match value {
            i => ICACHE_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `flash_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLASH_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLASH_CLK_GATERR {
        match value {
            i => FLASH_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `sram_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRAM_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRAM_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRAM_CLK_GATERR {
        match value {
            i => SRAM_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `apb_bridge_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APB_BRIDGE_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl APB_BRIDGE_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            APB_BRIDGE_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> APB_BRIDGE_CLK_GATERR {
        match value {
            i => APB_BRIDGE_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `sysman_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSMAN_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYSMAN_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSMAN_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYSMAN_CLK_GATERR {
        match value {
            i => SYSMAN_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ptp_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTP_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PTP_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTP_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTP_CLK_GATERR {
        match value {
            i => PTP_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ssb_mux_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSB_MUX_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SSB_MUX_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSB_MUX_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSB_MUX_CLK_GATERR {
        match value {
            i => SSB_MUX_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pad_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD_CLK_GATERR {
        match value {
            i => PAD_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `spix_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIX_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPIX_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPIX_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPIX_CLK_GATERR {
        match value {
            i => SPIX_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pmu_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMU_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PMU_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PMU_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PMU_CLK_GATERR {
        match value {
            i => PMU_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `usb_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USB_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USB_CLK_GATERR {
        match value {
            i => USB_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `crc_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRC_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRC_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRC_CLK_GATERR {
        match value {
            i => CRC_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tpu_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPU_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TPU_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TPU_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TPU_CLK_GATERR {
        match value {
            i => TPU_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `watchdog0_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WATCHDOG0_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WATCHDOG0_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WATCHDOG0_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WATCHDOG0_CLK_GATERR {
        match value {
            i => WATCHDOG0_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `cm4_clk_gater`"]
pub enum CM4_CLK_GATERW {}
impl CM4_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CM4_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _CM4_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CM4_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ahb32_clk_gater`"]
pub enum AHB32_CLK_GATERW {}
impl AHB32_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _AHB32_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB32_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHB32_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `icache_clk_gater`"]
pub enum ICACHE_CLK_GATERW {}
impl ICACHE_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ICACHE_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _ICACHE_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICACHE_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `flash_clk_gater`"]
pub enum FLASH_CLK_GATERW {}
impl FLASH_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLASH_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `sram_clk_gater`"]
pub enum SRAM_CLK_GATERW {}
impl SRAM_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SRAM_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `apb_bridge_clk_gater`"]
pub enum APB_BRIDGE_CLK_GATERW {}
impl APB_BRIDGE_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _APB_BRIDGE_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _APB_BRIDGE_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: APB_BRIDGE_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `sysman_clk_gater`"]
pub enum SYSMAN_CLK_GATERW {}
impl SYSMAN_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SYSMAN_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSMAN_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSMAN_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ptp_clk_gater`"]
pub enum PTP_CLK_GATERW {}
impl PTP_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PTP_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _PTP_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTP_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ssb_mux_clk_gater`"]
pub enum SSB_MUX_CLK_GATERW {}
impl SSB_MUX_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SSB_MUX_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _SSB_MUX_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSB_MUX_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pad_clk_gater`"]
pub enum PAD_CLK_GATERW {}
impl PAD_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PAD_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `spix_clk_gater`"]
pub enum SPIX_CLK_GATERW {}
impl SPIX_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SPIX_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIX_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIX_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pmu_clk_gater`"]
pub enum PMU_CLK_GATERW {}
impl PMU_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PMU_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _PMU_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PMU_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `usb_clk_gater`"]
pub enum USB_CLK_GATERW {}
impl USB_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _USB_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `crc_clk_gater`"]
pub enum CRC_CLK_GATERW {}
impl CRC_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CRC_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _CRC_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRC_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `tpu_clk_gater`"]
pub enum TPU_CLK_GATERW {}
impl TPU_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TPU_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _TPU_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPU_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `watchdog0_clk_gater`"]
pub enum WATCHDOG0_CLK_GATERW {}
impl WATCHDOG0_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WATCHDOG0_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _WATCHDOG0_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WATCHDOG0_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Clock Gating Control for CM4 CPU"]
    #[inline]
    pub fn cm4_clk_gater(&self) -> CM4_CLK_GATERR {
        CM4_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Clock Gating Control for AHB32"]
    #[inline]
    pub fn ahb32_clk_gater(&self) -> AHB32_CLK_GATERR {
        AHB32_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Clock Gating Control for Instruction Cache"]
    #[inline]
    pub fn icache_clk_gater(&self) -> ICACHE_CLK_GATERR {
        ICACHE_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Clock Gating Control for Flash Memory"]
    #[inline]
    pub fn flash_clk_gater(&self) -> FLASH_CLK_GATERR {
        FLASH_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Clock Gating Control for SRAM"]
    #[inline]
    pub fn sram_clk_gater(&self) -> SRAM_CLK_GATERR {
        SRAM_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Clock Gating Control for AHB-to-APB Bridge"]
    #[inline]
    pub fn apb_bridge_clk_gater(&self) -> APB_BRIDGE_CLK_GATERR {
        APB_BRIDGE_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Clock Gating Control for CLKMAN, PWRMAN, and IOMAN"]
    #[inline]
    pub fn sysman_clk_gater(&self) -> SYSMAN_CLK_GATERR {
        SYSMAN_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Clock Gating Control for PTP Logic"]
    #[inline]
    pub fn ptp_clk_gater(&self) -> PTP_CLK_GATERR {
        PTP_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Clock Gating Control for SSB Mux"]
    #[inline]
    pub fn ssb_mux_clk_gater(&self) -> SSB_MUX_CLK_GATERR {
        SSB_MUX_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Clock Gating Control for Pad Mode Filter"]
    #[inline]
    pub fn pad_clk_gater(&self) -> PAD_CLK_GATERR {
        PAD_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Clock Gating Control for SPI XIP"]
    #[inline]
    pub fn spix_clk_gater(&self) -> SPIX_CLK_GATERR {
        SPIX_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Clock Gating Control for PMU"]
    #[inline]
    pub fn pmu_clk_gater(&self) -> PMU_CLK_GATERR {
        PMU_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Clock Gating Control for USB"]
    #[inline]
    pub fn usb_clk_gater(&self) -> USB_CLK_GATERR {
        USB_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Clock Gating Control for CRC"]
    #[inline]
    pub fn crc_clk_gater(&self) -> CRC_CLK_GATERR {
        CRC_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Clock Gating Control for TPU"]
    #[inline]
    pub fn tpu_clk_gater(&self) -> TPU_CLK_GATERR {
        TPU_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Clock Gating Control for Watchdog Timer 0"]
    #[inline]
    pub fn watchdog0_clk_gater(&self) -> WATCHDOG0_CLK_GATERR {
        WATCHDOG0_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Clock Gating Control for CM4 CPU"]
    #[inline]
    pub fn cm4_clk_gater(&mut self) -> _CM4_CLK_GATERW {
        _CM4_CLK_GATERW { w: self }
    }
    #[doc = "Bits 2:3 - Clock Gating Control for AHB32"]
    #[inline]
    pub fn ahb32_clk_gater(&mut self) -> _AHB32_CLK_GATERW {
        _AHB32_CLK_GATERW { w: self }
    }
    #[doc = "Bits 4:5 - Clock Gating Control for Instruction Cache"]
    #[inline]
    pub fn icache_clk_gater(&mut self) -> _ICACHE_CLK_GATERW {
        _ICACHE_CLK_GATERW { w: self }
    }
    #[doc = "Bits 6:7 - Clock Gating Control for Flash Memory"]
    #[inline]
    pub fn flash_clk_gater(&mut self) -> _FLASH_CLK_GATERW {
        _FLASH_CLK_GATERW { w: self }
    }
    #[doc = "Bits 8:9 - Clock Gating Control for SRAM"]
    #[inline]
    pub fn sram_clk_gater(&mut self) -> _SRAM_CLK_GATERW {
        _SRAM_CLK_GATERW { w: self }
    }
    #[doc = "Bits 10:11 - Clock Gating Control for AHB-to-APB Bridge"]
    #[inline]
    pub fn apb_bridge_clk_gater(&mut self) -> _APB_BRIDGE_CLK_GATERW {
        _APB_BRIDGE_CLK_GATERW { w: self }
    }
    #[doc = "Bits 12:13 - Clock Gating Control for CLKMAN, PWRMAN, and IOMAN"]
    #[inline]
    pub fn sysman_clk_gater(&mut self) -> _SYSMAN_CLK_GATERW {
        _SYSMAN_CLK_GATERW { w: self }
    }
    #[doc = "Bits 14:15 - Clock Gating Control for PTP Logic"]
    #[inline]
    pub fn ptp_clk_gater(&mut self) -> _PTP_CLK_GATERW {
        _PTP_CLK_GATERW { w: self }
    }
    #[doc = "Bits 16:17 - Clock Gating Control for SSB Mux"]
    #[inline]
    pub fn ssb_mux_clk_gater(&mut self) -> _SSB_MUX_CLK_GATERW {
        _SSB_MUX_CLK_GATERW { w: self }
    }
    #[doc = "Bits 18:19 - Clock Gating Control for Pad Mode Filter"]
    #[inline]
    pub fn pad_clk_gater(&mut self) -> _PAD_CLK_GATERW {
        _PAD_CLK_GATERW { w: self }
    }
    #[doc = "Bits 20:21 - Clock Gating Control for SPI XIP"]
    #[inline]
    pub fn spix_clk_gater(&mut self) -> _SPIX_CLK_GATERW {
        _SPIX_CLK_GATERW { w: self }
    }
    #[doc = "Bits 22:23 - Clock Gating Control for PMU"]
    #[inline]
    pub fn pmu_clk_gater(&mut self) -> _PMU_CLK_GATERW {
        _PMU_CLK_GATERW { w: self }
    }
    #[doc = "Bits 24:25 - Clock Gating Control for USB"]
    #[inline]
    pub fn usb_clk_gater(&mut self) -> _USB_CLK_GATERW {
        _USB_CLK_GATERW { w: self }
    }
    #[doc = "Bits 26:27 - Clock Gating Control for CRC"]
    #[inline]
    pub fn crc_clk_gater(&mut self) -> _CRC_CLK_GATERW {
        _CRC_CLK_GATERW { w: self }
    }
    #[doc = "Bits 28:29 - Clock Gating Control for TPU"]
    #[inline]
    pub fn tpu_clk_gater(&mut self) -> _TPU_CLK_GATERW {
        _TPU_CLK_GATERW { w: self }
    }
    #[doc = "Bits 30:31 - Clock Gating Control for Watchdog Timer 0"]
    #[inline]
    pub fn watchdog0_clk_gater(&mut self) -> _WATCHDOG0_CLK_GATERW {
        _WATCHDOG0_CLK_GATERW { w: self }
    }
}
