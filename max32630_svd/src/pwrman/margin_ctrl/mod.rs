#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MARGIN_CTRL {
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
#[doc = "Possible values of the field `extra_margin`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRA_MARGINR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTRA_MARGINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTRA_MARGINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTRA_MARGINR {
        match value {
            i => EXTRA_MARGINR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `extra_write_margin`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRA_WRITE_MARGINR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTRA_WRITE_MARGINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTRA_WRITE_MARGINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTRA_WRITE_MARGINR {
        match value {
            i => EXTRA_WRITE_MARGINR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `write_assist_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_ASSIST_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WRITE_ASSIST_ENR {
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
            WRITE_ASSIST_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRITE_ASSIST_ENR {
        match value {
            i => WRITE_ASSIST_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `write_assist_margin`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_ASSIST_MARGINR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WRITE_ASSIST_MARGINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WRITE_ASSIST_MARGINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WRITE_ASSIST_MARGINR {
        match value {
            i => WRITE_ASSIST_MARGINR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `extra_margin`"]
pub enum EXTRA_MARGINW {}
impl EXTRA_MARGINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EXTRA_MARGINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTRA_MARGINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTRA_MARGINW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `extra_write_margin`"]
pub enum EXTRA_WRITE_MARGINW {}
impl EXTRA_WRITE_MARGINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EXTRA_WRITE_MARGINW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTRA_WRITE_MARGINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTRA_WRITE_MARGINW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `write_assist_en`"]
pub enum WRITE_ASSIST_ENW {}
impl WRITE_ASSIST_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WRITE_ASSIST_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITE_ASSIST_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRITE_ASSIST_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `write_assist_margin`"]
pub enum WRITE_ASSIST_MARGINW {}
impl WRITE_ASSIST_MARGINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WRITE_ASSIST_MARGINW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITE_ASSIST_MARGINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRITE_ASSIST_MARGINW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Extra Margin Adjustment"]
    #[inline]
    pub fn extra_margin(&self) -> EXTRA_MARGINR {
        EXTRA_MARGINR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:4 - Extra Write Margin Adjustment"]
    #[inline]
    pub fn extra_write_margin(&self) -> EXTRA_WRITE_MARGINR {
        EXTRA_WRITE_MARGINR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Write Assist Enable"]
    #[inline]
    pub fn write_assist_en(&self) -> WRITE_ASSIST_ENR {
        WRITE_ASSIST_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 6:7 - Write Assist Margin Adjustment"]
    #[inline]
    pub fn write_assist_margin(&self) -> WRITE_ASSIST_MARGINR {
        WRITE_ASSIST_MARGINR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:2 - Extra Margin Adjustment"]
    #[inline]
    pub fn extra_margin(&mut self) -> _EXTRA_MARGINW {
        _EXTRA_MARGINW { w: self }
    }
    #[doc = "Bits 3:4 - Extra Write Margin Adjustment"]
    #[inline]
    pub fn extra_write_margin(&mut self) -> _EXTRA_WRITE_MARGINW {
        _EXTRA_WRITE_MARGINW { w: self }
    }
    #[doc = "Bit 5 - Write Assist Enable"]
    #[inline]
    pub fn write_assist_en(&mut self) -> _WRITE_ASSIST_ENW {
        _WRITE_ASSIST_ENW { w: self }
    }
    #[doc = "Bits 6:7 - Write Assist Margin Adjustment"]
    #[inline]
    pub fn write_assist_margin(&mut self) -> _WRITE_ASSIST_MARGINW {
        _WRITE_ASSIST_MARGINW { w: self }
    }
}
