#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCKDIV {
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
#[doc = "Possible values of the field `fckdiv`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCKDIVR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FCKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FCKDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FCKDIVR {
        match value {
            i => FCKDIVR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `auto_fckdiv_result`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_FCKDIV_RESULTR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl AUTO_FCKDIV_RESULTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            AUTO_FCKDIV_RESULTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> AUTO_FCKDIV_RESULTR {
        match value {
            i => AUTO_FCKDIV_RESULTR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `fckdiv`"]
pub enum FCKDIVW {}
impl FCKDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FCKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _FCKDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCKDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:6 - Flash Clock Pulse Divisor"]
    #[inline]
    pub fn fckdiv(&self) -> FCKDIVR {
        FCKDIVR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:31 - Auto FCKDIV Calculation Result"]
    #[inline]
    pub fn auto_fckdiv_result(&self) -> AUTO_FCKDIV_RESULTR {
        AUTO_FCKDIV_RESULTR::_from({
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
    #[doc = "Bits 0:6 - Flash Clock Pulse Divisor"]
    #[inline]
    pub fn fckdiv(&mut self) -> _FCKDIVW {
        _FCKDIVW { w: self }
    }
}
