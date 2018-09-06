#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RO_CAL2 {
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
#[doc = "Possible values of the field `auto_cal_done_cnt`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_CAL_DONE_CNTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AUTO_CAL_DONE_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AUTO_CAL_DONE_CNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AUTO_CAL_DONE_CNTR {
        match value {
            i => AUTO_CAL_DONE_CNTR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `auto_cal_done_cnt`"]
pub enum AUTO_CAL_DONE_CNTW {}
impl AUTO_CAL_DONE_CNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _AUTO_CAL_DONE_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTO_CAL_DONE_CNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTO_CAL_DONE_CNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - Auto Cal Time Delay for Atomic Calibration (in milliseconds)"]
    #[inline]
    pub fn auto_cal_done_cnt(&self) -> AUTO_CAL_DONE_CNTR {
        AUTO_CAL_DONE_CNTR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:7 - Auto Cal Time Delay for Atomic Calibration (in milliseconds)"]
    #[inline]
    pub fn auto_cal_done_cnt(&mut self) -> _AUTO_CAL_DONE_CNTW {
        _AUTO_CAL_DONE_CNTW { w: self }
    }
}
