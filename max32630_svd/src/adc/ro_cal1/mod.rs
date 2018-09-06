#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RO_CAL1 {
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
#[doc = "Possible values of the field `trm_init`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRM_INITR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl TRM_INITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            TRM_INITR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> TRM_INITR {
        match value {
            i => TRM_INITR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `trm_min`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRM_MINR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl TRM_MINR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            TRM_MINR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> TRM_MINR {
        match value {
            i => TRM_MINR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `trm_max`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRM_MAXR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl TRM_MAXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            TRM_MAXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> TRM_MAXR {
        match value {
            i => TRM_MAXR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `trm_init`"]
pub enum TRM_INITW {}
impl TRM_INITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRM_INITW<'a> {
    w: &'a mut W,
}
impl<'a> _TRM_INITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRM_INITW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `trm_min`"]
pub enum TRM_MINW {}
impl TRM_MINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRM_MINW<'a> {
    w: &'a mut W,
}
impl<'a> _TRM_MINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRM_MINW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `trm_max`"]
pub enum TRM_MAXW {}
impl TRM_MAXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRM_MAXW<'a> {
    w: &'a mut W,
}
impl<'a> _TRM_MAXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRM_MAXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:8 - RO Trim Initial Value"]
    #[inline]
    pub fn trm_init(&self) -> TRM_INITR {
        TRM_INITR::_from({
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 10:18 - RO Trim Maximum Adaptive Limit"]
    #[inline]
    pub fn trm_min(&self) -> TRM_MINR {
        TRM_MINR::_from({
            const MASK: u16 = 511;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 20:28 - RO Trim Minimum Adaptive Limit"]
    #[inline]
    pub fn trm_max(&self) -> TRM_MAXR {
        TRM_MAXR::_from({
            const MASK: u16 = 511;
            const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:8 - RO Trim Initial Value"]
    #[inline]
    pub fn trm_init(&mut self) -> _TRM_INITW {
        _TRM_INITW { w: self }
    }
    #[doc = "Bits 10:18 - RO Trim Maximum Adaptive Limit"]
    #[inline]
    pub fn trm_min(&mut self) -> _TRM_MINW {
        _TRM_MINW { w: self }
    }
    #[doc = "Bits 20:28 - RO Trim Minimum Adaptive Limit"]
    #[inline]
    pub fn trm_max(&mut self) -> _TRM_MAXW {
        _TRM_MAXW { w: self }
    }
}
