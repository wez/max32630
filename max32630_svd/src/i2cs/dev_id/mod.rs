#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEV_ID {
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
#[doc = "Possible values of the field `slave_dev_id`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_DEV_IDR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SLAVE_DEV_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SLAVE_DEV_IDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SLAVE_DEV_IDR {
        match value {
            i => SLAVE_DEV_IDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ten_bit_id_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEN_BIT_ID_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TEN_BIT_ID_MODER {
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
            TEN_BIT_ID_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEN_BIT_ID_MODER {
        match value {
            i => TEN_BIT_ID_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `slave_reset`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_RESETR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SLAVE_RESETR {
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
            SLAVE_RESETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLAVE_RESETR {
        match value {
            i => SLAVE_RESETR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `slave_dev_id`"]
pub enum SLAVE_DEV_IDW {}
impl SLAVE_DEV_IDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SLAVE_DEV_IDW<'a> {
    w: &'a mut W,
}
impl<'a> _SLAVE_DEV_IDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLAVE_DEV_IDW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ten_bit_id_mode`"]
pub enum TEN_BIT_ID_MODEW {}
impl TEN_BIT_ID_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TEN_BIT_ID_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEN_BIT_ID_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEN_BIT_ID_MODEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `slave_reset`"]
pub enum SLAVE_RESETW {}
impl SLAVE_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SLAVE_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _SLAVE_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLAVE_RESETW) -> &'a mut W {
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
    #[doc = "Bits 0:9 - Slave Device ID"]
    #[inline]
    pub fn slave_dev_id(&self) -> SLAVE_DEV_IDR {
        SLAVE_DEV_IDR::_from({
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bit 12 - 10-bit ID Mode"]
    #[inline]
    pub fn ten_bit_id_mode(&self) -> TEN_BIT_ID_MODER {
        TEN_BIT_ID_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Slave Reset"]
    #[inline]
    pub fn slave_reset(&self) -> SLAVE_RESETR {
        SLAVE_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
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
    #[doc = "Bits 0:9 - Slave Device ID"]
    #[inline]
    pub fn slave_dev_id(&mut self) -> _SLAVE_DEV_IDW {
        _SLAVE_DEV_IDW { w: self }
    }
    #[doc = "Bit 12 - 10-bit ID Mode"]
    #[inline]
    pub fn ten_bit_id_mode(&mut self) -> _TEN_BIT_ID_MODEW {
        _TEN_BIT_ID_MODEW { w: self }
    }
    #[doc = "Bit 14 - Slave Reset"]
    #[inline]
    pub fn slave_reset(&mut self) -> _SLAVE_RESETW {
        _SLAVE_RESETW { w: self }
    }
}
