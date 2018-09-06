#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RX_FIFO_CTRL {
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
#[doc = "Possible values of the field `fifo_entry`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_ENTRYR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FIFO_ENTRYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FIFO_ENTRYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FIFO_ENTRYR {
        match value {
            i => FIFO_ENTRYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `fifo_af_lvl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_AF_LVLR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FIFO_AF_LVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FIFO_AF_LVLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FIFO_AF_LVLR {
        match value {
            i => FIFO_AF_LVLR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `fifo_af_lvl`"]
pub enum FIFO_AF_LVLW {}
impl FIFO_AF_LVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FIFO_AF_LVLW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFO_AF_LVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIFO_AF_LVLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bits 0:4 - RX FIFO Entries"]
    #[inline]
    pub fn fifo_entry(&self) -> FIFO_ENTRYR {
        FIFO_ENTRYR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:21 - RX FIFO AF Level"]
    #[inline]
    pub fn fifo_af_lvl(&self) -> FIFO_AF_LVLR {
        FIFO_AF_LVLR::_from({
            const MASK: u8 = 63;
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
    #[doc = "Bits 16:21 - RX FIFO AF Level"]
    #[inline]
    pub fn fifo_af_lvl(&mut self) -> _FIFO_AF_LVLW {
        _FIFO_AF_LVLW { w: self }
    }
}
