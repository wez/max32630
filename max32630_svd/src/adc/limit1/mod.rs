#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LIMIT1 {
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
#[doc = "Possible values of the field `ch_lo_limit`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH_LO_LIMITR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl CH_LO_LIMITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            CH_LO_LIMITR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> CH_LO_LIMITR {
        match value {
            i => CH_LO_LIMITR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ch_hi_limit`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH_HI_LIMITR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl CH_HI_LIMITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            CH_HI_LIMITR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> CH_HI_LIMITR {
        match value {
            i => CH_HI_LIMITR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ch_sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH_SELR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CH_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CH_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CH_SELR {
        match value {
            i => CH_SELR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ch_lo_limit_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH_LO_LIMIT_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CH_LO_LIMIT_ENR {
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
            CH_LO_LIMIT_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH_LO_LIMIT_ENR {
        match value {
            i => CH_LO_LIMIT_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ch_hi_limit_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH_HI_LIMIT_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CH_HI_LIMIT_ENR {
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
            CH_HI_LIMIT_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CH_HI_LIMIT_ENR {
        match value {
            i => CH_HI_LIMIT_ENR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `ch_lo_limit`"]
pub enum CH_LO_LIMITW {}
impl CH_LO_LIMITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CH_LO_LIMITW<'a> {
    w: &'a mut W,
}
impl<'a> _CH_LO_LIMITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH_LO_LIMITW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ch_hi_limit`"]
pub enum CH_HI_LIMITW {}
impl CH_HI_LIMITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CH_HI_LIMITW<'a> {
    w: &'a mut W,
}
impl<'a> _CH_HI_LIMITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH_HI_LIMITW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 1023;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ch_sel`"]
pub enum CH_SELW {}
impl CH_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CH_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _CH_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH_SELW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ch_lo_limit_en`"]
pub enum CH_LO_LIMIT_ENW {}
impl CH_LO_LIMIT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CH_LO_LIMIT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH_LO_LIMIT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH_LO_LIMIT_ENW) -> &'a mut W {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ch_hi_limit_en`"]
pub enum CH_HI_LIMIT_ENW {}
impl CH_HI_LIMIT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CH_HI_LIMIT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CH_HI_LIMIT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CH_HI_LIMIT_ENW) -> &'a mut W {
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline]
    pub fn ch_lo_limit(&self) -> CH_LO_LIMITR {
        CH_LO_LIMITR::_from({
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline]
    pub fn ch_hi_limit(&self) -> CH_HI_LIMITR {
        CH_HI_LIMITR::_from({
            const MASK: u16 = 1023;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 24:27 - ADC Channel Select"]
    #[inline]
    pub fn ch_sel(&self) -> CH_SELR {
        CH_SELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Low Limit Monitoring Enable"]
    #[inline]
    pub fn ch_lo_limit_en(&self) -> CH_LO_LIMIT_ENR {
        CH_LO_LIMIT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - High Limit Monitoring Enable"]
    #[inline]
    pub fn ch_hi_limit_en(&self) -> CH_HI_LIMIT_ENR {
        CH_HI_LIMIT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline]
    pub fn ch_lo_limit(&mut self) -> _CH_LO_LIMITW {
        _CH_LO_LIMITW { w: self }
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline]
    pub fn ch_hi_limit(&mut self) -> _CH_HI_LIMITW {
        _CH_HI_LIMITW { w: self }
    }
    #[doc = "Bits 24:27 - ADC Channel Select"]
    #[inline]
    pub fn ch_sel(&mut self) -> _CH_SELW {
        _CH_SELW { w: self }
    }
    #[doc = "Bit 28 - Low Limit Monitoring Enable"]
    #[inline]
    pub fn ch_lo_limit_en(&mut self) -> _CH_LO_LIMIT_ENW {
        _CH_LO_LIMIT_ENW { w: self }
    }
    #[doc = "Bit 29 - High Limit Monitoring Enable"]
    #[inline]
    pub fn ch_hi_limit_en(&mut self) -> _CH_HI_LIMIT_ENW {
        _CH_HI_LIMIT_ENW { w: self }
    }
}
