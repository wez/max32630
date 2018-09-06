#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTR {
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
#[doc = "Possible values of the field `finished_if`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINISHED_IFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FINISHED_IFR {
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
            FINISHED_IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FINISHED_IFR {
        match value {
            i => FINISHED_IFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `failed_if`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAILED_IFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FAILED_IFR {
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
            FAILED_IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAILED_IFR {
        match value {
            i => FAILED_IFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `finished_ie`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FINISHED_IER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FINISHED_IER {
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
            FINISHED_IER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FINISHED_IER {
        match value {
            i => FINISHED_IER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `failed_ie`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAILED_IER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FAILED_IER {
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
            FAILED_IER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAILED_IER {
        match value {
            i => FAILED_IER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `fail_flags`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAIL_FLAGSR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl FAIL_FLAGSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            FAIL_FLAGSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> FAIL_FLAGSR {
        match value {
            i => FAIL_FLAGSR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `finished_if`"]
pub enum FINISHED_IFW {}
impl FINISHED_IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FINISHED_IFW<'a> {
    w: &'a mut W,
}
impl<'a> _FINISHED_IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FINISHED_IFW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `failed_if`"]
pub enum FAILED_IFW {}
impl FAILED_IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FAILED_IFW<'a> {
    w: &'a mut W,
}
impl<'a> _FAILED_IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAILED_IFW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `finished_ie`"]
pub enum FINISHED_IEW {}
impl FINISHED_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FINISHED_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _FINISHED_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FINISHED_IEW) -> &'a mut W {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `failed_ie`"]
pub enum FAILED_IEW {}
impl FAILED_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FAILED_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _FAILED_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAILED_IEW) -> &'a mut W {
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Flash Write/Erase Operation Finished"]
    #[inline]
    pub fn finished_if(&self) -> FINISHED_IFR {
        FINISHED_IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Flash Operation Failed"]
    #[inline]
    pub fn failed_if(&self) -> FAILED_IFR {
        FAILED_IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Flash Write/Erase Operation Finished Interrupt Enable"]
    #[inline]
    pub fn finished_ie(&self) -> FINISHED_IER {
        FINISHED_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Flash Operation Failed Interrupt Enable"]
    #[inline]
    pub fn failed_ie(&self) -> FAILED_IER {
        FAILED_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:31 - Flash Operation Failure Details"]
    #[inline]
    pub fn fail_flags(&self) -> FAIL_FLAGSR {
        FAIL_FLAGSR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bit 0 - Flash Write/Erase Operation Finished"]
    #[inline]
    pub fn finished_if(&mut self) -> _FINISHED_IFW {
        _FINISHED_IFW { w: self }
    }
    #[doc = "Bit 1 - Flash Operation Failed"]
    #[inline]
    pub fn failed_if(&mut self) -> _FAILED_IFW {
        _FAILED_IFW { w: self }
    }
    #[doc = "Bit 8 - Flash Write/Erase Operation Finished Interrupt Enable"]
    #[inline]
    pub fn finished_ie(&mut self) -> _FINISHED_IEW {
        _FINISHED_IEW { w: self }
    }
    #[doc = "Bit 9 - Flash Operation Failed Interrupt Enable"]
    #[inline]
    pub fn failed_ie(&mut self) -> _FAILED_IEW {
        _FAILED_IEW { w: self }
    }
}
