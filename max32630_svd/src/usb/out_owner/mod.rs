#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUT_OWNER {
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
#[doc = "Possible values of the field `buf0_owner`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF0_OWNERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BUF0_OWNERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BUF0_OWNERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BUF0_OWNERR {
        match value {
            i => BUF0_OWNERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf1_owner`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF1_OWNERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BUF1_OWNERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BUF1_OWNERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BUF1_OWNERR {
        match value {
            i => BUF1_OWNERR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `buf0_owner`"]
pub enum BUF0_OWNERW {}
impl BUF0_OWNERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF0_OWNERW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF0_OWNERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF0_OWNERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `buf1_owner`"]
pub enum BUF1_OWNERW {}
impl BUF1_OWNERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF1_OWNERW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF1_OWNERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF1_OWNERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - Owner for OUT Buffer 0 for Endpoints"]
    #[inline]
    pub fn buf0_owner(&self) -> BUF0_OWNERR {
        BUF0_OWNERR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Owner for OUT Buffer 1 for Endpoints"]
    #[inline]
    pub fn buf1_owner(&self) -> BUF1_OWNERR {
        BUF1_OWNERR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:7 - Owner for OUT Buffer 0 for Endpoints"]
    #[inline]
    pub fn buf0_owner(&mut self) -> _BUF0_OWNERW {
        _BUF0_OWNERW { w: self }
    }
    #[doc = "Bits 16:23 - Owner for OUT Buffer 1 for Endpoints"]
    #[inline]
    pub fn buf1_owner(&mut self) -> _BUF1_OWNERW {
        _BUF1_OWNERW { w: self }
    }
}
