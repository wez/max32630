#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLK_GATE_CTRL2 {
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
#[doc = "Possible values of the field `i2cs_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CS_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl I2CS_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            I2CS_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> I2CS_CLK_GATERR {
        match value {
            i => I2CS_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `spi0_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPI0_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPI0_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPI0_CLK_GATERR {
        match value {
            i => SPI0_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `spi1_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPI1_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPI1_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPI1_CLK_GATERR {
        match value {
            i => SPI1_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `spi2_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI2_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPI2_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPI2_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPI2_CLK_GATERR {
        match value {
            i => SPI2_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `spi_bridge_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI_BRIDGE_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPI_BRIDGE_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPI_BRIDGE_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPI_BRIDGE_CLK_GATERR {
        match value {
            i => SPI_BRIDGE_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `owm_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OWM_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OWM_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OWM_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OWM_CLK_GATERR {
        match value {
            i => OWM_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADC_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC_CLK_GATERR {
        match value {
            i => ADC_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `spis_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIS_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPIS_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPIS_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPIS_CLK_GATERR {
        match value {
            i => SPIS_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `i2cs_clk_gater`"]
pub enum I2CS_CLK_GATERW {}
impl I2CS_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _I2CS_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CS_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2CS_CLK_GATERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `spi0_clk_gater`"]
pub enum SPI0_CLK_GATERW {}
impl SPI0_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SPI0_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI0_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI0_CLK_GATERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `spi1_clk_gater`"]
pub enum SPI1_CLK_GATERW {}
impl SPI1_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SPI1_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI1_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI1_CLK_GATERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `spi2_clk_gater`"]
pub enum SPI2_CLK_GATERW {}
impl SPI2_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SPI2_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI2_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI2_CLK_GATERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `spi_bridge_clk_gater`"]
pub enum SPI_BRIDGE_CLK_GATERW {}
impl SPI_BRIDGE_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SPI_BRIDGE_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI_BRIDGE_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI_BRIDGE_CLK_GATERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `owm_clk_gater`"]
pub enum OWM_CLK_GATERW {}
impl OWM_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OWM_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _OWM_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OWM_CLK_GATERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `adc_clk_gater`"]
pub enum ADC_CLK_GATERW {}
impl ADC_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_CLK_GATERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `spis_clk_gater`"]
pub enum SPIS_CLK_GATERW {}
impl SPIS_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SPIS_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIS_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIS_CLK_GATERW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Clock Gating Control for I2C Slave"]
    #[inline]
    pub fn i2cs_clk_gater(&self) -> I2CS_CLK_GATERR {
        I2CS_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Clock Gating Control for SPI Master 0"]
    #[inline]
    pub fn spi0_clk_gater(&self) -> SPI0_CLK_GATERR {
        SPI0_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Clock Gating Control for SPI Master 1"]
    #[inline]
    pub fn spi1_clk_gater(&self) -> SPI1_CLK_GATERR {
        SPI1_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Clock Gating Control for SPI Master 2"]
    #[inline]
    pub fn spi2_clk_gater(&self) -> SPI2_CLK_GATERR {
        SPI2_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Clock Gating Control for SPI Bridge"]
    #[inline]
    pub fn spi_bridge_clk_gater(&self) -> SPI_BRIDGE_CLK_GATERR {
        SPI_BRIDGE_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Clock Gating Control for 1-Wire Master (OWM)"]
    #[inline]
    pub fn owm_clk_gater(&self) -> OWM_CLK_GATERR {
        OWM_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Clock Gating Control for ADC"]
    #[inline]
    pub fn adc_clk_gater(&self) -> ADC_CLK_GATERR {
        ADC_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Clock Gating Control for SPI Slave"]
    #[inline]
    pub fn spis_clk_gater(&self) -> SPIS_CLK_GATERR {
        SPIS_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:1 - Clock Gating Control for I2C Slave"]
    #[inline]
    pub fn i2cs_clk_gater(&mut self) -> _I2CS_CLK_GATERW {
        _I2CS_CLK_GATERW { w: self }
    }
    #[doc = "Bits 2:3 - Clock Gating Control for SPI Master 0"]
    #[inline]
    pub fn spi0_clk_gater(&mut self) -> _SPI0_CLK_GATERW {
        _SPI0_CLK_GATERW { w: self }
    }
    #[doc = "Bits 4:5 - Clock Gating Control for SPI Master 1"]
    #[inline]
    pub fn spi1_clk_gater(&mut self) -> _SPI1_CLK_GATERW {
        _SPI1_CLK_GATERW { w: self }
    }
    #[doc = "Bits 6:7 - Clock Gating Control for SPI Master 2"]
    #[inline]
    pub fn spi2_clk_gater(&mut self) -> _SPI2_CLK_GATERW {
        _SPI2_CLK_GATERW { w: self }
    }
    #[doc = "Bits 8:9 - Clock Gating Control for SPI Bridge"]
    #[inline]
    pub fn spi_bridge_clk_gater(&mut self) -> _SPI_BRIDGE_CLK_GATERW {
        _SPI_BRIDGE_CLK_GATERW { w: self }
    }
    #[doc = "Bits 10:11 - Clock Gating Control for 1-Wire Master (OWM)"]
    #[inline]
    pub fn owm_clk_gater(&mut self) -> _OWM_CLK_GATERW {
        _OWM_CLK_GATERW { w: self }
    }
    #[doc = "Bits 12:13 - Clock Gating Control for ADC"]
    #[inline]
    pub fn adc_clk_gater(&mut self) -> _ADC_CLK_GATERW {
        _ADC_CLK_GATERW { w: self }
    }
    #[doc = "Bits 14:15 - Clock Gating Control for SPI Slave"]
    #[inline]
    pub fn spis_clk_gater(&mut self) -> _SPIS_CLK_GATERW {
        _SPIS_CLK_GATERW { w: self }
    }
}
