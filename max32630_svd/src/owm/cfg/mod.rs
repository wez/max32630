#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `long_line_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LONG_LINE_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl LONG_LINE_MODER {
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
            LONG_LINE_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LONG_LINE_MODER {
        match value {
            i => LONG_LINE_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `force_pres_det`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_PRES_DETR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FORCE_PRES_DETR {
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
            FORCE_PRES_DETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FORCE_PRES_DETR {
        match value {
            i => FORCE_PRES_DETR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bit_bang_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT_BANG_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BIT_BANG_ENR {
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
            BIT_BANG_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIT_BANG_ENR {
        match value {
            i => BIT_BANG_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ext_pullup_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_PULLUP_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EXT_PULLUP_MODER {
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
            EXT_PULLUP_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXT_PULLUP_MODER {
        match value {
            i => EXT_PULLUP_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ext_pullup_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXT_PULLUP_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EXT_PULLUP_ENABLER {
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
            EXT_PULLUP_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXT_PULLUP_ENABLER {
        match value {
            i => EXT_PULLUP_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `single_bit_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINGLE_BIT_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SINGLE_BIT_MODER {
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
            SINGLE_BIT_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SINGLE_BIT_MODER {
        match value {
            i => SINGLE_BIT_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `overdrive`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERDRIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OVERDRIVER {
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
            OVERDRIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVERDRIVER {
        match value {
            i => OVERDRIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `int_pullup_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_PULLUP_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl INT_PULLUP_ENABLER {
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
            INT_PULLUP_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_PULLUP_ENABLER {
        match value {
            i => INT_PULLUP_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `long_line_mode`"]
pub enum LONG_LINE_MODEW {}
impl LONG_LINE_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _LONG_LINE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LONG_LINE_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LONG_LINE_MODEW) -> &'a mut W {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `force_pres_det`"]
pub enum FORCE_PRES_DETW {}
impl FORCE_PRES_DETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FORCE_PRES_DETW<'a> {
    w: &'a mut W,
}
impl<'a> _FORCE_PRES_DETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FORCE_PRES_DETW) -> &'a mut W {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `bit_bang_en`"]
pub enum BIT_BANG_ENW {}
impl BIT_BANG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BIT_BANG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BIT_BANG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIT_BANG_ENW) -> &'a mut W {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ext_pullup_mode`"]
pub enum EXT_PULLUP_MODEW {}
impl EXT_PULLUP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EXT_PULLUP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXT_PULLUP_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXT_PULLUP_MODEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ext_pullup_enable`"]
pub enum EXT_PULLUP_ENABLEW {}
impl EXT_PULLUP_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EXT_PULLUP_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXT_PULLUP_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXT_PULLUP_ENABLEW) -> &'a mut W {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `single_bit_mode`"]
pub enum SINGLE_BIT_MODEW {}
impl SINGLE_BIT_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SINGLE_BIT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SINGLE_BIT_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SINGLE_BIT_MODEW) -> &'a mut W {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `overdrive`"]
pub enum OVERDRIVEW {}
impl OVERDRIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OVERDRIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERDRIVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVERDRIVEW) -> &'a mut W {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `int_pullup_enable`"]
pub enum INT_PULLUP_ENABLEW {}
impl INT_PULLUP_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _INT_PULLUP_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_PULLUP_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT_PULLUP_ENABLEW) -> &'a mut W {
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Long Line Mode"]
    #[inline]
    pub fn long_line_mode(&self) -> LONG_LINE_MODER {
        LONG_LINE_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Force Line During Presence Detect"]
    #[inline]
    pub fn force_pres_det(&self) -> FORCE_PRES_DETR {
        FORCE_PRES_DETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Bit Bang Enable"]
    #[inline]
    pub fn bit_bang_en(&self) -> BIT_BANG_ENR {
        BIT_BANG_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Provide an extra output to control an external pullup."]
    #[inline]
    pub fn ext_pullup_mode(&self) -> EXT_PULLUP_MODER {
        EXT_PULLUP_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable External Pullup"]
    #[inline]
    pub fn ext_pullup_enable(&self) -> EXT_PULLUP_ENABLER {
        EXT_PULLUP_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode"]
    #[inline]
    pub fn single_bit_mode(&self) -> SINGLE_BIT_MODER {
        SINGLE_BIT_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline]
    pub fn overdrive(&self) -> OVERDRIVER {
        OVERDRIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable internal pullup."]
    #[inline]
    pub fn int_pullup_enable(&self) -> INT_PULLUP_ENABLER {
        INT_PULLUP_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Long Line Mode"]
    #[inline]
    pub fn long_line_mode(&mut self) -> _LONG_LINE_MODEW {
        _LONG_LINE_MODEW { w: self }
    }
    #[doc = "Bit 1 - Force Line During Presence Detect"]
    #[inline]
    pub fn force_pres_det(&mut self) -> _FORCE_PRES_DETW {
        _FORCE_PRES_DETW { w: self }
    }
    #[doc = "Bit 2 - Bit Bang Enable"]
    #[inline]
    pub fn bit_bang_en(&mut self) -> _BIT_BANG_ENW {
        _BIT_BANG_ENW { w: self }
    }
    #[doc = "Bit 3 - Provide an extra output to control an external pullup."]
    #[inline]
    pub fn ext_pullup_mode(&mut self) -> _EXT_PULLUP_MODEW {
        _EXT_PULLUP_MODEW { w: self }
    }
    #[doc = "Bit 4 - Enable External Pullup"]
    #[inline]
    pub fn ext_pullup_enable(&mut self) -> _EXT_PULLUP_ENABLEW {
        _EXT_PULLUP_ENABLEW { w: self }
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode"]
    #[inline]
    pub fn single_bit_mode(&mut self) -> _SINGLE_BIT_MODEW {
        _SINGLE_BIT_MODEW { w: self }
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline]
    pub fn overdrive(&mut self) -> _OVERDRIVEW {
        _OVERDRIVEW { w: self }
    }
    #[doc = "Bit 7 - Enable internal pullup."]
    #[inline]
    pub fn int_pullup_enable(&mut self) -> _INT_PULLUP_ENABLEW {
        _INT_PULLUP_ENABLEW { w: self }
    }
}
