#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WUD_CTRL {
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
#[doc = "Possible values of the field `pad_select`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD_SELECTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD_SELECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD_SELECTR {
        match value {
            i => PAD_SELECTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pad_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAD_MODER {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PAD_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAD_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAD_MODER {
        match value {
            i => PAD_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `clear_all`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEAR_ALLR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CLEAR_ALLR {
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
            CLEAR_ALLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLEAR_ALLR {
        match value {
            i => CLEAR_ALLR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ctrl_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CTRL_ENABLER {
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
            CTRL_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTRL_ENABLER {
        match value {
            i => CTRL_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `pad_select`"]
pub enum PAD_SELECTW {}
impl PAD_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PAD_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD_SELECTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pad_mode`"]
pub enum PAD_MODEW {}
impl PAD_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PAD_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PAD_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAD_MODEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `clear_all`"]
pub enum CLEAR_ALLW {}
impl CLEAR_ALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CLEAR_ALLW<'a> {
    w: &'a mut W,
}
impl<'a> _CLEAR_ALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLEAR_ALLW) -> &'a mut W {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ctrl_enable`"]
pub enum CTRL_ENABLEW {}
impl CTRL_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CTRL_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRL_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTRL_ENABLEW) -> &'a mut W {
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
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:5 - Wake-Up Pad Select"]
    #[inline]
    pub fn pad_select(&self) -> PAD_SELECTR {
        PAD_SELECTR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Wake-Up Pad Signal Mode"]
    #[inline]
    pub fn pad_mode(&self) -> PAD_MODER {
        PAD_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Clear All WUD Pad States"]
    #[inline]
    pub fn clear_all(&self) -> CLEAR_ALLR {
        CLEAR_ALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable WUD Control Modification"]
    #[inline]
    pub fn ctrl_enable(&self) -> CTRL_ENABLER {
        CTRL_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:5 - Wake-Up Pad Select"]
    #[inline]
    pub fn pad_select(&mut self) -> _PAD_SELECTW {
        _PAD_SELECTW { w: self }
    }
    #[doc = "Bits 8:9 - Wake-Up Pad Signal Mode"]
    #[inline]
    pub fn pad_mode(&mut self) -> _PAD_MODEW {
        _PAD_MODEW { w: self }
    }
    #[doc = "Bit 12 - Clear All WUD Pad States"]
    #[inline]
    pub fn clear_all(&mut self) -> _CLEAR_ALLW {
        _CLEAR_ALLW { w: self }
    }
    #[doc = "Bit 16 - Enable WUD Control Modification"]
    #[inline]
    pub fn ctrl_enable(&mut self) -> _CTRL_ENABLEW {
        _CTRL_ENABLEW { w: self }
    }
}
