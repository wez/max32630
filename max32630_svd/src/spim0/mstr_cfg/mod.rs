#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MSTR_CFG {
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
#[doc = "Possible values of the field `slave_sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_SELR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLAVE_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLAVE_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLAVE_SELR {
        match value {
            i => SLAVE_SELR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `three_wire_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREE_WIRE_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl THREE_WIRE_MODER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            THREE_WIRE_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> THREE_WIRE_MODER {
        match value {
            i => THREE_WIRE_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `spi_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI_MODER {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPI_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPI_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPI_MODER {
        match value {
            i => SPI_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `page_size`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAGE_SIZER {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAGE_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAGE_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAGE_SIZER {
        match value {
            i => PAGE_SIZER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `sck_hi_clk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCK_HI_CLKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCK_HI_CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCK_HI_CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCK_HI_CLKR {
        match value {
            i => SCK_HI_CLKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `sck_lo_clk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCK_LO_CLKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCK_LO_CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCK_LO_CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCK_LO_CLKR {
        match value {
            i => SCK_LO_CLKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `act_delay`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACT_DELAYR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ACT_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACT_DELAYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACT_DELAYR {
        match value {
            i => ACT_DELAYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `inact_delay`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INACT_DELAYR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INACT_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INACT_DELAYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INACT_DELAYR {
        match value {
            i => INACT_DELAYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `alt_sck_hi_clk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALT_SCK_HI_CLKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALT_SCK_HI_CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALT_SCK_HI_CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALT_SCK_HI_CLKR {
        match value {
            i => ALT_SCK_HI_CLKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `alt_sck_lo_clk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALT_SCK_LO_CLKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALT_SCK_LO_CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALT_SCK_LO_CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALT_SCK_LO_CLKR {
        match value {
            i => ALT_SCK_LO_CLKR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `slave_sel`"]
pub enum SLAVE_SELW {}
impl SLAVE_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SLAVE_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SLAVE_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLAVE_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `three_wire_mode`"]
pub enum THREE_WIRE_MODEW {}
impl THREE_WIRE_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _THREE_WIRE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _THREE_WIRE_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: THREE_WIRE_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `spi_mode`"]
pub enum SPI_MODEW {}
impl SPI_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SPI_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI_MODEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `page_size`"]
pub enum PAGE_SIZEW {}
impl PAGE_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PAGE_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PAGE_SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAGE_SIZEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `sck_hi_clk`"]
pub enum SCK_HI_CLKW {}
impl SCK_HI_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SCK_HI_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SCK_HI_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCK_HI_CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `sck_lo_clk`"]
pub enum SCK_LO_CLKW {}
impl SCK_LO_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SCK_LO_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SCK_LO_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCK_LO_CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `act_delay`"]
pub enum ACT_DELAYW {}
impl ACT_DELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ACT_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _ACT_DELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACT_DELAYW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `inact_delay`"]
pub enum INACT_DELAYW {}
impl INACT_DELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _INACT_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _INACT_DELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INACT_DELAYW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `alt_sck_hi_clk`"]
pub enum ALT_SCK_HI_CLKW {}
impl ALT_SCK_HI_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ALT_SCK_HI_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ALT_SCK_HI_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALT_SCK_HI_CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `alt_sck_lo_clk`"]
pub enum ALT_SCK_LO_CLKW {}
impl ALT_SCK_LO_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ALT_SCK_LO_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ALT_SCK_LO_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALT_SCK_LO_CLKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:2 - SPI Slave Select"]
    #[inline]
    pub fn slave_sel(&self) -> SLAVE_SELR {
        SLAVE_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - 3-Wire Mode"]
    #[inline]
    pub fn three_wire_mode(&self) -> THREE_WIRE_MODER {
        THREE_WIRE_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - SPI Mode"]
    #[inline]
    pub fn spi_mode(&self) -> SPI_MODER {
        SPI_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Page Size"]
    #[inline]
    pub fn page_size(&self) -> PAGE_SIZER {
        PAGE_SIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - SCK High Clocks"]
    #[inline]
    pub fn sck_hi_clk(&self) -> SCK_HI_CLKR {
        SCK_HI_CLKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - SCK Low Clocks"]
    #[inline]
    pub fn sck_lo_clk(&self) -> SCK_LO_CLKR {
        SCK_LO_CLKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - SS Active Timing"]
    #[inline]
    pub fn act_delay(&self) -> ACT_DELAYR {
        ACT_DELAYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - SS Inactive Timing"]
    #[inline]
    pub fn inact_delay(&self) -> INACT_DELAYR {
        INACT_DELAYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Alt SCK High Clocks"]
    #[inline]
    pub fn alt_sck_hi_clk(&self) -> ALT_SCK_HI_CLKR {
        ALT_SCK_HI_CLKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Alt SCK Low Clocks"]
    #[inline]
    pub fn alt_sck_lo_clk(&self) -> ALT_SCK_LO_CLKR {
        ALT_SCK_LO_CLKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:2 - SPI Slave Select"]
    #[inline]
    pub fn slave_sel(&mut self) -> _SLAVE_SELW {
        _SLAVE_SELW { w: self }
    }
    #[doc = "Bit 3 - 3-Wire Mode"]
    #[inline]
    pub fn three_wire_mode(&mut self) -> _THREE_WIRE_MODEW {
        _THREE_WIRE_MODEW { w: self }
    }
    #[doc = "Bits 4:5 - SPI Mode"]
    #[inline]
    pub fn spi_mode(&mut self) -> _SPI_MODEW {
        _SPI_MODEW { w: self }
    }
    #[doc = "Bits 6:7 - Page Size"]
    #[inline]
    pub fn page_size(&mut self) -> _PAGE_SIZEW {
        _PAGE_SIZEW { w: self }
    }
    #[doc = "Bits 8:11 - SCK High Clocks"]
    #[inline]
    pub fn sck_hi_clk(&mut self) -> _SCK_HI_CLKW {
        _SCK_HI_CLKW { w: self }
    }
    #[doc = "Bits 12:15 - SCK Low Clocks"]
    #[inline]
    pub fn sck_lo_clk(&mut self) -> _SCK_LO_CLKW {
        _SCK_LO_CLKW { w: self }
    }
    #[doc = "Bits 16:17 - SS Active Timing"]
    #[inline]
    pub fn act_delay(&mut self) -> _ACT_DELAYW {
        _ACT_DELAYW { w: self }
    }
    #[doc = "Bits 18:19 - SS Inactive Timing"]
    #[inline]
    pub fn inact_delay(&mut self) -> _INACT_DELAYW {
        _INACT_DELAYW { w: self }
    }
    #[doc = "Bits 20:23 - Alt SCK High Clocks"]
    #[inline]
    pub fn alt_sck_hi_clk(&mut self) -> _ALT_SCK_HI_CLKW {
        _ALT_SCK_HI_CLKW { w: self }
    }
    #[doc = "Bits 24:27 - Alt SCK Low Clocks"]
    #[inline]
    pub fn alt_sck_lo_clk(&mut self) -> _ALT_SCK_LO_CLKW {
        _ALT_SCK_LO_CLKW { w: self }
    }
}
