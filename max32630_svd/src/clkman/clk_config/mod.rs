#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLK_CONFIG {
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
#[doc = "Possible values of the field `crypto_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTO_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CRYPTO_ENABLER {
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
            CRYPTO_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRYPTO_ENABLER {
        match value {
            i => CRYPTO_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `crypto_stability_count`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTO_STABILITY_COUNTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CRYPTO_STABILITY_COUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRYPTO_STABILITY_COUNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRYPTO_STABILITY_COUNTR {
        match value {
            i => CRYPTO_STABILITY_COUNTR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `crypto_enable`"]
pub enum CRYPTO_ENABLEW {}
impl CRYPTO_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CRYPTO_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRYPTO_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRYPTO_ENABLEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `crypto_stability_count`"]
pub enum CRYPTO_STABILITY_COUNTW {}
impl CRYPTO_STABILITY_COUNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CRYPTO_STABILITY_COUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _CRYPTO_STABILITY_COUNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRYPTO_STABILITY_COUNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Cryptographic (TPU) Relaxation Oscillator Enable"]
    #[inline]
    pub fn crypto_enable(&self) -> CRYPTO_ENABLER {
        CRYPTO_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:7 - Crypto Oscillator Stability Select"]
    #[inline]
    pub fn crypto_stability_count(&self) -> CRYPTO_STABILITY_COUNTR {
        CRYPTO_STABILITY_COUNTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Cryptographic (TPU) Relaxation Oscillator Enable"]
    #[inline]
    pub fn crypto_enable(&mut self) -> _CRYPTO_ENABLEW {
        _CRYPTO_ENABLEW { w: self }
    }
    #[doc = "Bits 4:7 - Crypto Oscillator Stability Select"]
    #[inline]
    pub fn crypto_stability_count(&mut self) -> _CRYPTO_STABILITY_COUNTW {
        _CRYPTO_STABILITY_COUNTW { w: self }
    }
}
