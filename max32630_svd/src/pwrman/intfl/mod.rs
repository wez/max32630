#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTFL {
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
#[doc = "Possible values of the field `v1_2_warning`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V1_2_WARNINGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl V1_2_WARNINGR {
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
            V1_2_WARNINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> V1_2_WARNINGR {
        match value {
            i => V1_2_WARNINGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `v1_8_warning`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum V1_8_WARNINGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl V1_8_WARNINGR {
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
            V1_8_WARNINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> V1_8_WARNINGR {
        match value {
            i => V1_8_WARNINGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rtc_warning`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_WARNINGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTC_WARNINGR {
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
            RTC_WARNINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_WARNINGR {
        match value {
            i => RTC_WARNINGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `vdda_warning`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDA_WARNINGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl VDDA_WARNINGR {
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
            VDDA_WARNINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VDDA_WARNINGR {
        match value {
            i => VDDA_WARNINGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `vddb_warning`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDB_WARNINGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl VDDB_WARNINGR {
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
            VDDB_WARNINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VDDB_WARNINGR {
        match value {
            i => VDDB_WARNINGR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `v1_2_warning`"]
pub enum V1_2_WARNINGW {}
impl V1_2_WARNINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _V1_2_WARNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _V1_2_WARNINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: V1_2_WARNINGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `v1_8_warning`"]
pub enum V1_8_WARNINGW {}
impl V1_8_WARNINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _V1_8_WARNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _V1_8_WARNINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: V1_8_WARNINGW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `rtc_warning`"]
pub enum RTC_WARNINGW {}
impl RTC_WARNINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RTC_WARNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _RTC_WARNINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTC_WARNINGW) -> &'a mut W {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `vdda_warning`"]
pub enum VDDA_WARNINGW {}
impl VDDA_WARNINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _VDDA_WARNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDA_WARNINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDDA_WARNINGW) -> &'a mut W {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `vddb_warning`"]
pub enum VDDB_WARNINGW {}
impl VDDB_WARNINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _VDDB_WARNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDB_WARNINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDDB_WARNINGW) -> &'a mut W {
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
    #[doc = "Bit 0 - 1.2V Warning Monitor Int Flag"]
    #[inline]
    pub fn v1_2_warning(&self) -> V1_2_WARNINGR {
        V1_2_WARNINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 1.8V Warning Monitor Int Flag"]
    #[inline]
    pub fn v1_8_warning(&self) -> V1_8_WARNINGR {
        V1_8_WARNINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RTC Warning Monitor Int Flag"]
    #[inline]
    pub fn rtc_warning(&self) -> RTC_WARNINGR {
        RTC_WARNINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - VDDA Warning Monitor Int Flag"]
    #[inline]
    pub fn vdda_warning(&self) -> VDDA_WARNINGR {
        VDDA_WARNINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - VDDB Warning Monitor Int Flag"]
    #[inline]
    pub fn vddb_warning(&self) -> VDDB_WARNINGR {
        VDDB_WARNINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - 1.2V Warning Monitor Int Flag"]
    #[inline]
    pub fn v1_2_warning(&mut self) -> _V1_2_WARNINGW {
        _V1_2_WARNINGW { w: self }
    }
    #[doc = "Bit 1 - 1.8V Warning Monitor Int Flag"]
    #[inline]
    pub fn v1_8_warning(&mut self) -> _V1_8_WARNINGW {
        _V1_8_WARNINGW { w: self }
    }
    #[doc = "Bit 2 - RTC Warning Monitor Int Flag"]
    #[inline]
    pub fn rtc_warning(&mut self) -> _RTC_WARNINGW {
        _RTC_WARNINGW { w: self }
    }
    #[doc = "Bit 3 - VDDA Warning Monitor Int Flag"]
    #[inline]
    pub fn vdda_warning(&mut self) -> _VDDA_WARNINGW {
        _VDDA_WARNINGW { w: self }
    }
    #[doc = "Bit 4 - VDDB Warning Monitor Int Flag"]
    #[inline]
    pub fn vddb_warning(&mut self) -> _VDDB_WARNINGW {
        _VDDB_WARNINGW { w: self }
    }
}
