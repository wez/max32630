#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE_DATA {
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
#[doc = "Possible values of the field `mode_data_bits`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_DATA_BITSR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl MODE_DATA_BITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            MODE_DATA_BITSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> MODE_DATA_BITSR {
        match value {
            i => MODE_DATA_BITSR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `mode_data_oe`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_DATA_OER {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl MODE_DATA_OER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            MODE_DATA_OER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> MODE_DATA_OER {
        match value {
            i => MODE_DATA_OER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `mode_data_bits`"]
pub enum MODE_DATA_BITSW {}
impl MODE_DATA_BITSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MODE_DATA_BITSW<'a> {
    w: &'a mut W,
}
impl<'a> _MODE_DATA_BITSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE_DATA_BITSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `mode_data_oe`"]
pub enum MODE_DATA_OEW {}
impl MODE_DATA_OEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MODE_DATA_OEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODE_DATA_OEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODE_DATA_OEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:15 - Mode Data"]
    #[inline]
    pub fn mode_data_bits(&self) -> MODE_DATA_BITSR {
        MODE_DATA_BITSR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:31 - Mode Output Enable"]
    #[inline]
    pub fn mode_data_oe(&self) -> MODE_DATA_OER {
        MODE_DATA_OER::_from({
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
    #[doc = "Bits 0:15 - Mode Data"]
    #[inline]
    pub fn mode_data_bits(&mut self) -> _MODE_DATA_BITSW {
        _MODE_DATA_BITSW { w: self }
    }
    #[doc = "Bits 16:31 - Mode Output Enable"]
    #[inline]
    pub fn mode_data_oe(&mut self) -> _MODE_DATA_OEW {
        _MODE_DATA_OEW { w: self }
    }
}
