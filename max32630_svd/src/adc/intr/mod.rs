#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTR {
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
#[doc = "Possible values of the field `adc_done_ie`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DONE_IER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_DONE_IER {
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
            ADC_DONE_IER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_DONE_IER {
        match value {
            i => ADC_DONE_IER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_ref_ready_ie`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_REF_READY_IER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_REF_READY_IER {
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
            ADC_REF_READY_IER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_REF_READY_IER {
        match value {
            i => ADC_REF_READY_IER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_hi_limit_ie`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_HI_LIMIT_IER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_HI_LIMIT_IER {
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
            ADC_HI_LIMIT_IER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_HI_LIMIT_IER {
        match value {
            i => ADC_HI_LIMIT_IER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_lo_limit_ie`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_LO_LIMIT_IER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_LO_LIMIT_IER {
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
            ADC_LO_LIMIT_IER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_LO_LIMIT_IER {
        match value {
            i => ADC_LO_LIMIT_IER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_overflow_ie`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_OVERFLOW_IER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_OVERFLOW_IER {
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
            ADC_OVERFLOW_IER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_OVERFLOW_IER {
        match value {
            i => ADC_OVERFLOW_IER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ro_cal_done_ie`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO_CAL_DONE_IER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RO_CAL_DONE_IER {
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
            RO_CAL_DONE_IER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RO_CAL_DONE_IER {
        match value {
            i => RO_CAL_DONE_IER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_done_if`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DONE_IFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_DONE_IFR {
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
            ADC_DONE_IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_DONE_IFR {
        match value {
            i => ADC_DONE_IFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_ref_ready_if`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_REF_READY_IFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_REF_READY_IFR {
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
            ADC_REF_READY_IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_REF_READY_IFR {
        match value {
            i => ADC_REF_READY_IFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_hi_limit_if`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_HI_LIMIT_IFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_HI_LIMIT_IFR {
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
            ADC_HI_LIMIT_IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_HI_LIMIT_IFR {
        match value {
            i => ADC_HI_LIMIT_IFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_lo_limit_if`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_LO_LIMIT_IFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_LO_LIMIT_IFR {
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
            ADC_LO_LIMIT_IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_LO_LIMIT_IFR {
        match value {
            i => ADC_LO_LIMIT_IFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_overflow_if`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_OVERFLOW_IFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_OVERFLOW_IFR {
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
            ADC_OVERFLOW_IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_OVERFLOW_IFR {
        match value {
            i => ADC_OVERFLOW_IFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ro_cal_done_if`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RO_CAL_DONE_IFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RO_CAL_DONE_IFR {
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
            RO_CAL_DONE_IFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RO_CAL_DONE_IFR {
        match value {
            i => RO_CAL_DONE_IFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_int_pending`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_INT_PENDINGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_INT_PENDINGR {
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
            ADC_INT_PENDINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_INT_PENDINGR {
        match value {
            i => ADC_INT_PENDINGR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `adc_done_ie`"]
pub enum ADC_DONE_IEW {}
impl ADC_DONE_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_DONE_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DONE_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_DONE_IEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `adc_ref_ready_ie`"]
pub enum ADC_REF_READY_IEW {}
impl ADC_REF_READY_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_REF_READY_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_REF_READY_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_REF_READY_IEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `adc_hi_limit_ie`"]
pub enum ADC_HI_LIMIT_IEW {}
impl ADC_HI_LIMIT_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_HI_LIMIT_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_HI_LIMIT_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_HI_LIMIT_IEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `adc_lo_limit_ie`"]
pub enum ADC_LO_LIMIT_IEW {}
impl ADC_LO_LIMIT_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_LO_LIMIT_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_LO_LIMIT_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_LO_LIMIT_IEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `adc_overflow_ie`"]
pub enum ADC_OVERFLOW_IEW {}
impl ADC_OVERFLOW_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_OVERFLOW_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_OVERFLOW_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_OVERFLOW_IEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ro_cal_done_ie`"]
pub enum RO_CAL_DONE_IEW {}
impl RO_CAL_DONE_IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RO_CAL_DONE_IEW<'a> {
    w: &'a mut W,
}
impl<'a> _RO_CAL_DONE_IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RO_CAL_DONE_IEW) -> &'a mut W {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_done_if`"]
pub enum ADC_DONE_IFW {}
impl ADC_DONE_IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_DONE_IFW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DONE_IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_DONE_IFW) -> &'a mut W {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_ref_ready_if`"]
pub enum ADC_REF_READY_IFW {}
impl ADC_REF_READY_IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_REF_READY_IFW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_REF_READY_IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_REF_READY_IFW) -> &'a mut W {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_hi_limit_if`"]
pub enum ADC_HI_LIMIT_IFW {}
impl ADC_HI_LIMIT_IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_HI_LIMIT_IFW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_HI_LIMIT_IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_HI_LIMIT_IFW) -> &'a mut W {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_lo_limit_if`"]
pub enum ADC_LO_LIMIT_IFW {}
impl ADC_LO_LIMIT_IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_LO_LIMIT_IFW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_LO_LIMIT_IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_LO_LIMIT_IFW) -> &'a mut W {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_overflow_if`"]
pub enum ADC_OVERFLOW_IFW {}
impl ADC_OVERFLOW_IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_OVERFLOW_IFW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_OVERFLOW_IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_OVERFLOW_IFW) -> &'a mut W {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ro_cal_done_if`"]
pub enum RO_CAL_DONE_IFW {}
impl RO_CAL_DONE_IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RO_CAL_DONE_IFW<'a> {
    w: &'a mut W,
}
impl<'a> _RO_CAL_DONE_IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RO_CAL_DONE_IFW) -> &'a mut W {
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
        const OFFSET: u8 = 21;
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
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline]
    pub fn adc_done_ie(&self) -> ADC_DONE_IER {
        ADC_DONE_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline]
    pub fn adc_ref_ready_ie(&self) -> ADC_REF_READY_IER {
        ADC_REF_READY_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline]
    pub fn adc_hi_limit_ie(&self) -> ADC_HI_LIMIT_IER {
        ADC_HI_LIMIT_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline]
    pub fn adc_lo_limit_ie(&self) -> ADC_LO_LIMIT_IER {
        ADC_LO_LIMIT_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline]
    pub fn adc_overflow_ie(&self) -> ADC_OVERFLOW_IER {
        ADC_OVERFLOW_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RO Cal Done Interrupt Enable"]
    #[inline]
    pub fn ro_cal_done_ie(&self) -> RO_CAL_DONE_IER {
        RO_CAL_DONE_IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline]
    pub fn adc_done_if(&self) -> ADC_DONE_IFR {
        ADC_DONE_IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline]
    pub fn adc_ref_ready_if(&self) -> ADC_REF_READY_IFR {
        ADC_REF_READY_IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline]
    pub fn adc_hi_limit_if(&self) -> ADC_HI_LIMIT_IFR {
        ADC_HI_LIMIT_IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline]
    pub fn adc_lo_limit_if(&self) -> ADC_LO_LIMIT_IFR {
        ADC_LO_LIMIT_IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline]
    pub fn adc_overflow_if(&self) -> ADC_OVERFLOW_IFR {
        ADC_OVERFLOW_IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - RO Cal Done Interrupt Flag"]
    #[inline]
    pub fn ro_cal_done_if(&self) -> RO_CAL_DONE_IFR {
        RO_CAL_DONE_IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - ADC Interrupt Pending Status"]
    #[inline]
    pub fn adc_int_pending(&self) -> ADC_INT_PENDINGR {
        ADC_INT_PENDINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
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
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline]
    pub fn adc_done_ie(&mut self) -> _ADC_DONE_IEW {
        _ADC_DONE_IEW { w: self }
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline]
    pub fn adc_ref_ready_ie(&mut self) -> _ADC_REF_READY_IEW {
        _ADC_REF_READY_IEW { w: self }
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline]
    pub fn adc_hi_limit_ie(&mut self) -> _ADC_HI_LIMIT_IEW {
        _ADC_HI_LIMIT_IEW { w: self }
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline]
    pub fn adc_lo_limit_ie(&mut self) -> _ADC_LO_LIMIT_IEW {
        _ADC_LO_LIMIT_IEW { w: self }
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline]
    pub fn adc_overflow_ie(&mut self) -> _ADC_OVERFLOW_IEW {
        _ADC_OVERFLOW_IEW { w: self }
    }
    #[doc = "Bit 5 - RO Cal Done Interrupt Enable"]
    #[inline]
    pub fn ro_cal_done_ie(&mut self) -> _RO_CAL_DONE_IEW {
        _RO_CAL_DONE_IEW { w: self }
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline]
    pub fn adc_done_if(&mut self) -> _ADC_DONE_IFW {
        _ADC_DONE_IFW { w: self }
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline]
    pub fn adc_ref_ready_if(&mut self) -> _ADC_REF_READY_IFW {
        _ADC_REF_READY_IFW { w: self }
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline]
    pub fn adc_hi_limit_if(&mut self) -> _ADC_HI_LIMIT_IFW {
        _ADC_HI_LIMIT_IFW { w: self }
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline]
    pub fn adc_lo_limit_if(&mut self) -> _ADC_LO_LIMIT_IFW {
        _ADC_LO_LIMIT_IFW { w: self }
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline]
    pub fn adc_overflow_if(&mut self) -> _ADC_OVERFLOW_IFW {
        _ADC_OVERFLOW_IFW { w: self }
    }
    #[doc = "Bit 21 - RO Cal Done Interrupt Flag"]
    #[inline]
    pub fn ro_cal_done_if(&mut self) -> _RO_CAL_DONE_IFW {
        _RO_CAL_DONE_IFW { w: self }
    }
}
