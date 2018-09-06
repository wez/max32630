#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRIM_CTRL {
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
#[doc = "Possible values of the field `trim_enable_r`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_ENABLE_RR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TRIM_ENABLE_RR {
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
            TRIM_ENABLE_RR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIM_ENABLE_RR {
        match value {
            i => TRIM_ENABLE_RR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `trim_faster_ovr_r`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_FASTER_OVR_RR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TRIM_FASTER_OVR_RR {
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
            TRIM_FASTER_OVR_RR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIM_FASTER_OVR_RR {
        match value {
            i => TRIM_FASTER_OVR_RR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `trim_slower_r`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_SLOWER_RR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TRIM_SLOWER_RR {
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
            TRIM_SLOWER_RR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIM_SLOWER_RR {
        match value {
            i => TRIM_SLOWER_RR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `trim_enable_r`"]
pub enum TRIM_ENABLE_RW {}
impl TRIM_ENABLE_RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_ENABLE_RW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_ENABLE_RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIM_ENABLE_RW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `trim_faster_ovr_r`"]
pub enum TRIM_FASTER_OVR_RW {}
impl TRIM_FASTER_OVR_RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_FASTER_OVR_RW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_FASTER_OVR_RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIM_FASTER_OVR_RW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable RTL Trim of RTC Timer"]
    #[inline]
    pub fn trim_enable_r(&self) -> TRIM_ENABLE_RR {
        TRIM_ENABLE_RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Force RTC Trim to Faster"]
    #[inline]
    pub fn trim_faster_ovr_r(&self) -> TRIM_FASTER_OVR_RR {
        TRIM_FASTER_OVR_RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RTC Trim Direction Status"]
    #[inline]
    pub fn trim_slower_r(&self) -> TRIM_SLOWER_RR {
        TRIM_SLOWER_RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Enable RTL Trim of RTC Timer"]
    #[inline]
    pub fn trim_enable_r(&mut self) -> _TRIM_ENABLE_RW {
        _TRIM_ENABLE_RW { w: self }
    }
    #[doc = "Bit 1 - Force RTC Trim to Faster"]
    #[inline]
    pub fn trim_faster_ovr_r(&mut self) -> _TRIM_FASTER_OVR_RW {
        _TRIM_FASTER_OVR_RW { w: self }
    }
}
