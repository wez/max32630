#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RO_CAL0 {
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
#[doc = "Possible values of the field `ro_cal_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO_CAL_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RO_CAL_ENR {
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
            RO_CAL_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RO_CAL_ENR {
        match value {
            i => RO_CAL_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ro_cal_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO_CAL_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RO_CAL_RUNR {
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
            RO_CAL_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RO_CAL_RUNR {
        match value {
            i => RO_CAL_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ro_cal_load`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO_CAL_LOADR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RO_CAL_LOADR {
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
            RO_CAL_LOADR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RO_CAL_LOADR {
        match value {
            i => RO_CAL_LOADR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ro_cal_atomic`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO_CAL_ATOMICR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RO_CAL_ATOMICR {
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
            RO_CAL_ATOMICR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RO_CAL_ATOMICR {
        match value {
            i => RO_CAL_ATOMICR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `dummy`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUMMYR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DUMMYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DUMMYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DUMMYR {
        match value {
            i => DUMMYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `trm_mu`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRM_MUR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl TRM_MUR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            TRM_MUR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> TRM_MUR {
        match value {
            i => TRM_MUR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ro_trm`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO_TRMR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl RO_TRMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            RO_TRMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> RO_TRMR {
        match value {
            i => RO_TRMR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `ro_cal_en`"]
pub enum RO_CAL_ENW {}
impl RO_CAL_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RO_CAL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RO_CAL_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RO_CAL_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ro_cal_run`"]
pub enum RO_CAL_RUNW {}
impl RO_CAL_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RO_CAL_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _RO_CAL_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RO_CAL_RUNW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ro_cal_load`"]
pub enum RO_CAL_LOADW {}
impl RO_CAL_LOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RO_CAL_LOADW<'a> {
    w: &'a mut W,
}
impl<'a> _RO_CAL_LOADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RO_CAL_LOADW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ro_cal_atomic`"]
pub enum RO_CAL_ATOMICW {}
impl RO_CAL_ATOMICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RO_CAL_ATOMICW<'a> {
    w: &'a mut W,
}
impl<'a> _RO_CAL_ATOMICW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RO_CAL_ATOMICW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `dummy`"]
pub enum DUMMYW {}
impl DUMMYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DUMMYW<'a> {
    w: &'a mut W,
}
impl<'a> _DUMMYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DUMMYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `trm_mu`"]
pub enum TRM_MUW {}
impl TRM_MUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRM_MUW<'a> {
    w: &'a mut W,
}
impl<'a> _TRM_MUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRM_MUW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ro_trm`"]
pub enum RO_TRMW {}
impl RO_TRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RO_TRMW<'a> {
    w: &'a mut W,
}
impl<'a> _RO_TRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RO_TRMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - RO Calibration Enable"]
    #[inline]
    pub fn ro_cal_en(&self) -> RO_CAL_ENR {
        RO_CAL_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RO Calibration Run"]
    #[inline]
    pub fn ro_cal_run(&self) -> RO_CAL_RUNR {
        RO_CAL_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RO Calibration Load Initial Value"]
    #[inline]
    pub fn ro_cal_load(&self) -> RO_CAL_LOADR {
        RO_CAL_LOADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RO Calibration Run Atomic"]
    #[inline]
    pub fn ro_cal_atomic(&self) -> RO_CAL_ATOMICR {
        RO_CAL_ATOMICR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:7 - Dummy Write Field"]
    #[inline]
    pub fn dummy(&self) -> DUMMYR {
        DUMMYR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:19 - RO Trim Adaptation Gain"]
    #[inline]
    pub fn trm_mu(&self) -> TRM_MUR {
        TRM_MUR::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 23:31 - RO Trim Calibration Result"]
    #[inline]
    pub fn ro_trm(&self) -> RO_TRMR {
        RO_TRMR::_from({
            const MASK: u16 = 511;
            const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - RO Calibration Enable"]
    #[inline]
    pub fn ro_cal_en(&mut self) -> _RO_CAL_ENW {
        _RO_CAL_ENW { w: self }
    }
    #[doc = "Bit 1 - RO Calibration Run"]
    #[inline]
    pub fn ro_cal_run(&mut self) -> _RO_CAL_RUNW {
        _RO_CAL_RUNW { w: self }
    }
    #[doc = "Bit 2 - RO Calibration Load Initial Value"]
    #[inline]
    pub fn ro_cal_load(&mut self) -> _RO_CAL_LOADW {
        _RO_CAL_LOADW { w: self }
    }
    #[doc = "Bit 4 - RO Calibration Run Atomic"]
    #[inline]
    pub fn ro_cal_atomic(&mut self) -> _RO_CAL_ATOMICW {
        _RO_CAL_ATOMICW { w: self }
    }
    #[doc = "Bits 5:7 - Dummy Write Field"]
    #[inline]
    pub fn dummy(&mut self) -> _DUMMYW {
        _DUMMYW { w: self }
    }
    #[doc = "Bits 8:19 - RO Trim Adaptation Gain"]
    #[inline]
    pub fn trm_mu(&mut self) -> _TRM_MUW {
        _TRM_MUW { w: self }
    }
    #[doc = "Bits 23:31 - RO Trim Calibration Result"]
    #[inline]
    pub fn ro_trm(&mut self) -> _RO_TRMW {
        _RO_TRMW { w: self }
    }
}
