#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `cpu_adc_start`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU_ADC_STARTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CPU_ADC_STARTR {
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
            CPU_ADC_STARTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPU_ADC_STARTR {
        match value {
            i => CPU_ADC_STARTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_pu`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PUR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_PUR {
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
            ADC_PUR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_PUR {
        match value {
            i => ADC_PUR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf_pu`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_PUR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_PUR {
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
            BUF_PUR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_PUR {
        match value {
            i => BUF_PUR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_refbuf_pu`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_REFBUF_PUR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_REFBUF_PUR {
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
            ADC_REFBUF_PUR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_REFBUF_PUR {
        match value {
            i => ADC_REFBUF_PUR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_chgpump_pu`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_CHGPUMP_PUR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_CHGPUMP_PUR {
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
            ADC_CHGPUMP_PUR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_CHGPUMP_PUR {
        match value {
            i => ADC_CHGPUMP_PUR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf_chop_dis`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_CHOP_DISR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_CHOP_DISR {
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
            BUF_CHOP_DISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_CHOP_DISR {
        match value {
            i => BUF_CHOP_DISR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf_pump_dis`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_PUMP_DISR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_PUMP_DISR {
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
            BUF_PUMP_DISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_PUMP_DISR {
        match value {
            i => BUF_PUMP_DISR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf_bypass`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_BYPASSR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_BYPASSR {
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
            BUF_BYPASSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_BYPASSR {
        match value {
            i => BUF_BYPASSR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_refscl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_REFSCLR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_REFSCLR {
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
            ADC_REFSCLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_REFSCLR {
        match value {
            i => ADC_REFSCLR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_scale`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_SCALER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_SCALER {
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
            ADC_SCALER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_SCALER {
        match value {
            i => ADC_SCALER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_refsel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_REFSELR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_REFSELR {
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
            ADC_REFSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_REFSELR {
        match value {
            i => ADC_REFSELR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_clk_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_CLK_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_CLK_ENR {
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
            ADC_CLK_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_CLK_ENR {
        match value {
            i => ADC_CLK_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_chsel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_CHSELR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADC_CHSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_CHSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC_CHSELR {
        match value {
            i => ADC_CHSELR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_xref`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_XREFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_XREFR {
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
            ADC_XREFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_XREFR {
        match value {
            i => ADC_XREFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_dataalign`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DATAALIGNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_DATAALIGNR {
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
            ADC_DATAALIGNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_DATAALIGNR {
        match value {
            i => ADC_DATAALIGNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `afe_pwr_up_dly`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFE_PWR_UP_DLYR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AFE_PWR_UP_DLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AFE_PWR_UP_DLYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AFE_PWR_UP_DLYR {
        match value {
            i => AFE_PWR_UP_DLYR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `cpu_adc_start`"]
pub enum CPU_ADC_STARTW {}
impl CPU_ADC_STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CPU_ADC_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _CPU_ADC_STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPU_ADC_STARTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `adc_pu`"]
pub enum ADC_PUW {}
impl ADC_PUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_PUW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_PUW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `buf_pu`"]
pub enum BUF_PUW {}
impl BUF_PUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_PUW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_PUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_PUW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `adc_refbuf_pu`"]
pub enum ADC_REFBUF_PUW {}
impl ADC_REFBUF_PUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_REFBUF_PUW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_REFBUF_PUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_REFBUF_PUW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `adc_chgpump_pu`"]
pub enum ADC_CHGPUMP_PUW {}
impl ADC_CHGPUMP_PUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_CHGPUMP_PUW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_CHGPUMP_PUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_CHGPUMP_PUW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `buf_chop_dis`"]
pub enum BUF_CHOP_DISW {}
impl BUF_CHOP_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_CHOP_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_CHOP_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_CHOP_DISW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `buf_pump_dis`"]
pub enum BUF_PUMP_DISW {}
impl BUF_PUMP_DISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_PUMP_DISW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_PUMP_DISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_PUMP_DISW) -> &'a mut W {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `buf_bypass`"]
pub enum BUF_BYPASSW {}
impl BUF_BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_BYPASSW) -> &'a mut W {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_refscl`"]
pub enum ADC_REFSCLW {}
impl ADC_REFSCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_REFSCLW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_REFSCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_REFSCLW) -> &'a mut W {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_scale`"]
pub enum ADC_SCALEW {}
impl ADC_SCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_SCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_SCALEW) -> &'a mut W {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_refsel`"]
pub enum ADC_REFSELW {}
impl ADC_REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_REFSELW) -> &'a mut W {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_clk_en`"]
pub enum ADC_CLK_ENW {}
impl ADC_CLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_CLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_CLK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_CLK_ENW) -> &'a mut W {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_chsel`"]
pub enum ADC_CHSELW {}
impl ADC_CHSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_CHSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_CHSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_CHSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_xref`"]
pub enum ADC_XREFW {}
impl ADC_XREFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_XREFW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_XREFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_XREFW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `adc_dataalign`"]
pub enum ADC_DATAALIGNW {}
impl ADC_DATAALIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_DATAALIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DATAALIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_DATAALIGNW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `afe_pwr_up_dly`"]
pub enum AFE_PWR_UP_DLYW {}
impl AFE_PWR_UP_DLYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _AFE_PWR_UP_DLYW<'a> {
    w: &'a mut W,
}
impl<'a> _AFE_PWR_UP_DLYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFE_PWR_UP_DLYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Start ADC Conversion"]
    #[inline]
    pub fn cpu_adc_start(&self) -> CPU_ADC_STARTR {
        CPU_ADC_STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - ADC Power Up"]
    #[inline]
    pub fn adc_pu(&self) -> ADC_PUR {
        ADC_PUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - ADC Input Buffer Power Up"]
    #[inline]
    pub fn buf_pu(&self) -> BUF_PUR {
        BUF_PUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ADC Reference Buffer Power Up"]
    #[inline]
    pub fn adc_refbuf_pu(&self) -> ADC_REFBUF_PUR {
        ADC_REFBUF_PUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - ADC Charge Pump Power Up"]
    #[inline]
    pub fn adc_chgpump_pu(&self) -> ADC_CHGPUMP_PUR {
        ADC_CHGPUMP_PUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - ADC Input Buffer Chop Disable (INTERNAL ONLY)"]
    #[inline]
    pub fn buf_chop_dis(&self) -> BUF_CHOP_DISR {
        BUF_CHOP_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Disable Use of Charge Pump Output by Input Buffer (INTERNAL)"]
    #[inline]
    pub fn buf_pump_dis(&self) -> BUF_PUMP_DISR {
        BUF_PUMP_DISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Bypass Input Buffer"]
    #[inline]
    pub fn buf_bypass(&self) -> BUF_BYPASSR {
        BUF_BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - ADC Reference Scale"]
    #[inline]
    pub fn adc_refscl(&self) -> ADC_REFSCLR {
        ADC_REFSCLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - ADC Scale"]
    #[inline]
    pub fn adc_scale(&self) -> ADC_SCALER {
        ADC_SCALER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - ADC Reference (VRef) Select (INTERNAL ONLY)"]
    #[inline]
    pub fn adc_refsel(&self) -> ADC_REFSELR {
        ADC_REFSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - ADC Clock Enable"]
    #[inline]
    pub fn adc_clk_en(&self) -> ADC_CLK_ENR {
        ADC_CLK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:15 - ADC Channel Select"]
    #[inline]
    pub fn adc_chsel(&self) -> ADC_CHSELR {
        ADC_CHSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Enable Use of ADC External Reference"]
    #[inline]
    pub fn adc_xref(&self) -> ADC_XREFR {
        ADC_XREFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - ADC Data Alignment Select"]
    #[inline]
    pub fn adc_dataalign(&self) -> ADC_DATAALIGNR {
        ADC_DATAALIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:31 - Delay from ADC Powerup Until ADC Ready Asserted"]
    #[inline]
    pub fn afe_pwr_up_dly(&self) -> AFE_PWR_UP_DLYR {
        AFE_PWR_UP_DLYR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Start ADC Conversion"]
    #[inline]
    pub fn cpu_adc_start(&mut self) -> _CPU_ADC_STARTW {
        _CPU_ADC_STARTW { w: self }
    }
    #[doc = "Bit 1 - ADC Power Up"]
    #[inline]
    pub fn adc_pu(&mut self) -> _ADC_PUW {
        _ADC_PUW { w: self }
    }
    #[doc = "Bit 2 - ADC Input Buffer Power Up"]
    #[inline]
    pub fn buf_pu(&mut self) -> _BUF_PUW {
        _BUF_PUW { w: self }
    }
    #[doc = "Bit 3 - ADC Reference Buffer Power Up"]
    #[inline]
    pub fn adc_refbuf_pu(&mut self) -> _ADC_REFBUF_PUW {
        _ADC_REFBUF_PUW { w: self }
    }
    #[doc = "Bit 4 - ADC Charge Pump Power Up"]
    #[inline]
    pub fn adc_chgpump_pu(&mut self) -> _ADC_CHGPUMP_PUW {
        _ADC_CHGPUMP_PUW { w: self }
    }
    #[doc = "Bit 5 - ADC Input Buffer Chop Disable (INTERNAL ONLY)"]
    #[inline]
    pub fn buf_chop_dis(&mut self) -> _BUF_CHOP_DISW {
        _BUF_CHOP_DISW { w: self }
    }
    #[doc = "Bit 6 - Disable Use of Charge Pump Output by Input Buffer (INTERNAL)"]
    #[inline]
    pub fn buf_pump_dis(&mut self) -> _BUF_PUMP_DISW {
        _BUF_PUMP_DISW { w: self }
    }
    #[doc = "Bit 7 - Bypass Input Buffer"]
    #[inline]
    pub fn buf_bypass(&mut self) -> _BUF_BYPASSW {
        _BUF_BYPASSW { w: self }
    }
    #[doc = "Bit 8 - ADC Reference Scale"]
    #[inline]
    pub fn adc_refscl(&mut self) -> _ADC_REFSCLW {
        _ADC_REFSCLW { w: self }
    }
    #[doc = "Bit 9 - ADC Scale"]
    #[inline]
    pub fn adc_scale(&mut self) -> _ADC_SCALEW {
        _ADC_SCALEW { w: self }
    }
    #[doc = "Bit 10 - ADC Reference (VRef) Select (INTERNAL ONLY)"]
    #[inline]
    pub fn adc_refsel(&mut self) -> _ADC_REFSELW {
        _ADC_REFSELW { w: self }
    }
    #[doc = "Bit 11 - ADC Clock Enable"]
    #[inline]
    pub fn adc_clk_en(&mut self) -> _ADC_CLK_ENW {
        _ADC_CLK_ENW { w: self }
    }
    #[doc = "Bits 12:15 - ADC Channel Select"]
    #[inline]
    pub fn adc_chsel(&mut self) -> _ADC_CHSELW {
        _ADC_CHSELW { w: self }
    }
    #[doc = "Bit 16 - Enable Use of ADC External Reference"]
    #[inline]
    pub fn adc_xref(&mut self) -> _ADC_XREFW {
        _ADC_XREFW { w: self }
    }
    #[doc = "Bit 17 - ADC Data Alignment Select"]
    #[inline]
    pub fn adc_dataalign(&mut self) -> _ADC_DATAALIGNW {
        _ADC_DATAALIGNW { w: self }
    }
    #[doc = "Bits 24:31 - Delay from ADC Powerup Until ADC Ready Asserted"]
    #[inline]
    pub fn afe_pwr_up_dly(&mut self) -> _AFE_PWR_UP_DLYW {
        _AFE_PWR_UP_DLYW { w: self }
    }
}
