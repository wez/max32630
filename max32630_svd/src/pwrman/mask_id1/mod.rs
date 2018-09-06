#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MASK_ID1 {
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
#[doc = "Possible values of the field `mask_id`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_IDR {
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl MASK_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            MASK_IDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> MASK_IDR {
        match value {
            i => MASK_IDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `mask_id_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_ID_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MASK_ID_ENABLER {
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
            MASK_ID_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASK_ID_ENABLER {
        match value {
            i => MASK_ID_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `mask_id_enable`"]
pub enum MASK_ID_ENABLEW {}
impl MASK_ID_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MASK_ID_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MASK_ID_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASK_ID_ENABLEW) -> &'a mut W {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:30 - Mask ID[58:28]"]
    #[inline]
    pub fn mask_id(&self) -> MASK_IDR {
        MASK_IDR::_from({
            const MASK: u32 = 2147483647;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
    #[doc = "Bit 31 - Enable Mask ID"]
    #[inline]
    pub fn mask_id_enable(&self) -> MASK_ID_ENABLER {
        MASK_ID_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 31 - Enable Mask ID"]
    #[inline]
    pub fn mask_id_enable(&mut self) -> _MASK_ID_ENABLEW {
        _MASK_ID_ENABLEW { w: self }
    }
}
