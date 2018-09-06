#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG2 {
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
#[doc = "Possible values of the field `pwr_vdd12_hyst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VDD12_HYSTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_VDD12_HYSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_VDD12_HYSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_VDD12_HYSTR {
        match value {
            i => PWR_VDD12_HYSTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_vdd18_hyst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VDD18_HYSTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_VDD18_HYSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_VDD18_HYSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_VDD18_HYSTR {
        match value {
            i => PWR_VDD18_HYSTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_vrtc_hyst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VRTC_HYSTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_VRTC_HYSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_VRTC_HYSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_VRTC_HYSTR {
        match value {
            i => PWR_VRTC_HYSTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_vddb_hyst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VDDB_HYSTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_VDDB_HYSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_VDDB_HYSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_VDDB_HYSTR {
        match value {
            i => PWR_VDDB_HYSTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_tvdd12_hyst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TVDD12_HYSTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_TVDD12_HYSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_TVDD12_HYSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_TVDD12_HYSTR {
        match value {
            i => PWR_TVDD12_HYSTR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `pwr_vdd12_hyst`"]
pub enum PWR_VDD12_HYSTW {}
impl PWR_VDD12_HYSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VDD12_HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VDD12_HYSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VDD12_HYSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_vdd18_hyst`"]
pub enum PWR_VDD18_HYSTW {}
impl PWR_VDD18_HYSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VDD18_HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VDD18_HYSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VDD18_HYSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_vrtc_hyst`"]
pub enum PWR_VRTC_HYSTW {}
impl PWR_VRTC_HYSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VRTC_HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VRTC_HYSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VRTC_HYSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_vddb_hyst`"]
pub enum PWR_VDDB_HYSTW {}
impl PWR_VDDB_HYSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VDDB_HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VDDB_HYSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VDDB_HYSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_tvdd12_hyst`"]
pub enum PWR_TVDD12_HYSTW {}
impl PWR_TVDD12_HYSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TVDD12_HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TVDD12_HYSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TVDD12_HYSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - VDD12_SW Comparator Hysteresis Setting"]
    #[inline]
    pub fn pwr_vdd12_hyst(&self) -> PWR_VDD12_HYSTR {
        PWR_VDD12_HYSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - VDD18_SW Comparator Hysteresis Setting"]
    #[inline]
    pub fn pwr_vdd18_hyst(&self) -> PWR_VDD18_HYSTR {
        PWR_VDD18_HYSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - VRTC Comparator Hysteresis Setting"]
    #[inline]
    pub fn pwr_vrtc_hyst(&self) -> PWR_VRTC_HYSTR {
        PWR_VRTC_HYSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - VDDB Comparator Hysteresis Setting"]
    #[inline]
    pub fn pwr_vddb_hyst(&self) -> PWR_VDDB_HYSTR {
        PWR_VDDB_HYSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - TVDD12 Comparator Hysteresis Setting"]
    #[inline]
    pub fn pwr_tvdd12_hyst(&self) -> PWR_TVDD12_HYSTR {
        PWR_TVDD12_HYSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - VDD12_SW Comparator Hysteresis Setting"]
    #[inline]
    pub fn pwr_vdd12_hyst(&mut self) -> _PWR_VDD12_HYSTW {
        _PWR_VDD12_HYSTW { w: self }
    }
    #[doc = "Bits 2:3 - VDD18_SW Comparator Hysteresis Setting"]
    #[inline]
    pub fn pwr_vdd18_hyst(&mut self) -> _PWR_VDD18_HYSTW {
        _PWR_VDD18_HYSTW { w: self }
    }
    #[doc = "Bits 4:5 - VRTC Comparator Hysteresis Setting"]
    #[inline]
    pub fn pwr_vrtc_hyst(&mut self) -> _PWR_VRTC_HYSTW {
        _PWR_VRTC_HYSTW { w: self }
    }
    #[doc = "Bits 6:7 - VDDB Comparator Hysteresis Setting"]
    #[inline]
    pub fn pwr_vddb_hyst(&mut self) -> _PWR_VDDB_HYSTW {
        _PWR_VDDB_HYSTW { w: self }
    }
    #[doc = "Bits 8:9 - TVDD12 Comparator Hysteresis Setting"]
    #[inline]
    pub fn pwr_tvdd12_hyst(&mut self) -> _PWR_TVDD12_HYSTW {
        _PWR_TVDD12_HYSTW { w: self }
    }
}
