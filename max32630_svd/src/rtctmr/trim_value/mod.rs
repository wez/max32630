#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRIM_VALUE {
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
#[doc = "Possible values of the field `trim_value`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_VALUER {
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl TRIM_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            TRIM_VALUER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> TRIM_VALUER {
        match value {
            i => TRIM_VALUER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `trim_control`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_CONTROLR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TRIM_CONTROLR {
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
            TRIM_CONTROLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIM_CONTROLR {
        match value {
            i => TRIM_CONTROLR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `trim_value`"]
pub enum TRIM_VALUEW {}
impl TRIM_VALUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_VALUEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIM_VALUEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 262143;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `trim_control`"]
pub enum TRIM_CONTROLW {}
impl TRIM_CONTROLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_CONTROLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIM_CONTROLW) -> &'a mut W {
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:17 - Trim PPM Value"]
    #[inline]
    pub fn trim_value(&self) -> TRIM_VALUER {
        TRIM_VALUER::_from({
            const MASK: u32 = 262143;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
    #[doc = "Bit 18 - Trim Direction"]
    #[inline]
    pub fn trim_control(&self) -> TRIM_CONTROLR {
        TRIM_CONTROLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
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
    #[doc = "Bits 0:17 - Trim PPM Value"]
    #[inline]
    pub fn trim_value(&mut self) -> _TRIM_VALUEW {
        _TRIM_VALUEW { w: self }
    }
    #[doc = "Bit 18 - Trim Direction"]
    #[inline]
    pub fn trim_control(&mut self) -> _TRIM_CONTROLW {
        _TRIM_CONTROLW { w: self }
    }
}
