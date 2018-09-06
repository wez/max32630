#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLK_CTRL {
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
#[doc = "Possible values of the field `system_source_select`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTEM_SOURCE_SELECTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SYSTEM_SOURCE_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSTEM_SOURCE_SELECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SYSTEM_SOURCE_SELECTR {
        match value {
            i => SYSTEM_SOURCE_SELECTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `usb_clock_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_CLOCK_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl USB_CLOCK_ENABLER {
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
            USB_CLOCK_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB_CLOCK_ENABLER {
        match value {
            i => USB_CLOCK_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `usb_clock_select`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_CLOCK_SELECTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl USB_CLOCK_SELECTR {
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
            USB_CLOCK_SELECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB_CLOCK_SELECTR {
        match value {
            i => USB_CLOCK_SELECTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `crypto_clock_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPTO_CLOCK_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CRYPTO_CLOCK_ENABLER {
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
            CRYPTO_CLOCK_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRYPTO_CLOCK_ENABLER {
        match value {
            i => CRYPTO_CLOCK_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rtos_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTOS_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTOS_MODER {
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
            RTOS_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTOS_MODER {
        match value {
            i => RTOS_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `cpu_dynamic_clock`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU_DYNAMIC_CLOCKR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CPU_DYNAMIC_CLOCKR {
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
            CPU_DYNAMIC_CLOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPU_DYNAMIC_CLOCKR {
        match value {
            i => CPU_DYNAMIC_CLOCKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `wdt0_clock_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT0_CLOCK_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WDT0_CLOCK_ENABLER {
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
            WDT0_CLOCK_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDT0_CLOCK_ENABLER {
        match value {
            i => WDT0_CLOCK_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `wdt0_clock_select`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT0_CLOCK_SELECTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WDT0_CLOCK_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WDT0_CLOCK_SELECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WDT0_CLOCK_SELECTR {
        match value {
            i => WDT0_CLOCK_SELECTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `wdt1_clock_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT1_CLOCK_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WDT1_CLOCK_ENABLER {
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
            WDT1_CLOCK_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDT1_CLOCK_ENABLER {
        match value {
            i => WDT1_CLOCK_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `wdt1_clock_select`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT1_CLOCK_SELECTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WDT1_CLOCK_SELECTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WDT1_CLOCK_SELECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WDT1_CLOCK_SELECTR {
        match value {
            i => WDT1_CLOCK_SELECTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `adc_clock_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_CLOCK_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ADC_CLOCK_ENABLER {
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
            ADC_CLOCK_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_CLOCK_ENABLER {
        match value {
            i => ADC_CLOCK_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `system_source_select`"]
pub enum SYSTEM_SOURCE_SELECTW {}
impl SYSTEM_SOURCE_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SYSTEM_SOURCE_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSTEM_SOURCE_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSTEM_SOURCE_SELECTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `usb_clock_enable`"]
pub enum USB_CLOCK_ENABLEW {}
impl USB_CLOCK_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _USB_CLOCK_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CLOCK_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB_CLOCK_ENABLEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `usb_clock_select`"]
pub enum USB_CLOCK_SELECTW {}
impl USB_CLOCK_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _USB_CLOCK_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_CLOCK_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB_CLOCK_SELECTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `crypto_clock_enable`"]
pub enum CRYPTO_CLOCK_ENABLEW {}
impl CRYPTO_CLOCK_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CRYPTO_CLOCK_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRYPTO_CLOCK_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRYPTO_CLOCK_ENABLEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `rtos_mode`"]
pub enum RTOS_MODEW {}
impl RTOS_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RTOS_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _RTOS_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTOS_MODEW) -> &'a mut W {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `cpu_dynamic_clock`"]
pub enum CPU_DYNAMIC_CLOCKW {}
impl CPU_DYNAMIC_CLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CPU_DYNAMIC_CLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _CPU_DYNAMIC_CLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPU_DYNAMIC_CLOCKW) -> &'a mut W {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `wdt0_clock_enable`"]
pub enum WDT0_CLOCK_ENABLEW {}
impl WDT0_CLOCK_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WDT0_CLOCK_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _WDT0_CLOCK_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDT0_CLOCK_ENABLEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `wdt0_clock_select`"]
pub enum WDT0_CLOCK_SELECTW {}
impl WDT0_CLOCK_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WDT0_CLOCK_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDT0_CLOCK_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDT0_CLOCK_SELECTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `wdt1_clock_enable`"]
pub enum WDT1_CLOCK_ENABLEW {}
impl WDT1_CLOCK_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WDT1_CLOCK_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _WDT1_CLOCK_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDT1_CLOCK_ENABLEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `wdt1_clock_select`"]
pub enum WDT1_CLOCK_SELECTW {}
impl WDT1_CLOCK_SELECTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WDT1_CLOCK_SELECTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDT1_CLOCK_SELECTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDT1_CLOCK_SELECTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `adc_clock_enable`"]
pub enum ADC_CLOCK_ENABLEW {}
impl ADC_CLOCK_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ADC_CLOCK_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_CLOCK_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_CLOCK_ENABLEW) -> &'a mut W {
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
    #[doc = "Bits 0:1 - System Clock Source Select"]
    #[inline]
    pub fn system_source_select(&self) -> SYSTEM_SOURCE_SELECTR {
        SYSTEM_SOURCE_SELECTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - USB Clock Enable"]
    #[inline]
    pub fn usb_clock_enable(&self) -> USB_CLOCK_ENABLER {
        USB_CLOCK_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - USB Clock Select"]
    #[inline]
    pub fn usb_clock_select(&self) -> USB_CLOCK_SELECTR {
        USB_CLOCK_SELECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Crypto Clock Enable"]
    #[inline]
    pub fn crypto_clock_enable(&self) -> CRYPTO_CLOCK_ENABLER {
        CRYPTO_CLOCK_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable RTOS Mode for SysTick Timers"]
    #[inline]
    pub fn rtos_mode(&self) -> RTOS_MODER {
        RTOS_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable CPU Dynamic Clock Gating"]
    #[inline]
    pub fn cpu_dynamic_clock(&self) -> CPU_DYNAMIC_CLOCKR {
        CPU_DYNAMIC_CLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Watchdog 0 Clock Enable"]
    #[inline]
    pub fn wdt0_clock_enable(&self) -> WDT0_CLOCK_ENABLER {
        WDT0_CLOCK_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 17:18 - Watchdog 0 Clock Source Select"]
    #[inline]
    pub fn wdt0_clock_select(&self) -> WDT0_CLOCK_SELECTR {
        WDT0_CLOCK_SELECTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Watchdog 1 Clock Enable"]
    #[inline]
    pub fn wdt1_clock_enable(&self) -> WDT1_CLOCK_ENABLER {
        WDT1_CLOCK_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 21:22 - Watchdog 1 Clock Source Select"]
    #[inline]
    pub fn wdt1_clock_select(&self) -> WDT1_CLOCK_SELECTR {
        WDT1_CLOCK_SELECTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - ADC Clock Enable"]
    #[inline]
    pub fn adc_clock_enable(&self) -> ADC_CLOCK_ENABLER {
        ADC_CLOCK_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:1 - System Clock Source Select"]
    #[inline]
    pub fn system_source_select(&mut self) -> _SYSTEM_SOURCE_SELECTW {
        _SYSTEM_SOURCE_SELECTW { w: self }
    }
    #[doc = "Bit 4 - USB Clock Enable"]
    #[inline]
    pub fn usb_clock_enable(&mut self) -> _USB_CLOCK_ENABLEW {
        _USB_CLOCK_ENABLEW { w: self }
    }
    #[doc = "Bit 5 - USB Clock Select"]
    #[inline]
    pub fn usb_clock_select(&mut self) -> _USB_CLOCK_SELECTW {
        _USB_CLOCK_SELECTW { w: self }
    }
    #[doc = "Bit 8 - Crypto Clock Enable"]
    #[inline]
    pub fn crypto_clock_enable(&mut self) -> _CRYPTO_CLOCK_ENABLEW {
        _CRYPTO_CLOCK_ENABLEW { w: self }
    }
    #[doc = "Bit 12 - Enable RTOS Mode for SysTick Timers"]
    #[inline]
    pub fn rtos_mode(&mut self) -> _RTOS_MODEW {
        _RTOS_MODEW { w: self }
    }
    #[doc = "Bit 13 - Enable CPU Dynamic Clock Gating"]
    #[inline]
    pub fn cpu_dynamic_clock(&mut self) -> _CPU_DYNAMIC_CLOCKW {
        _CPU_DYNAMIC_CLOCKW { w: self }
    }
    #[doc = "Bit 16 - Watchdog 0 Clock Enable"]
    #[inline]
    pub fn wdt0_clock_enable(&mut self) -> _WDT0_CLOCK_ENABLEW {
        _WDT0_CLOCK_ENABLEW { w: self }
    }
    #[doc = "Bits 17:18 - Watchdog 0 Clock Source Select"]
    #[inline]
    pub fn wdt0_clock_select(&mut self) -> _WDT0_CLOCK_SELECTW {
        _WDT0_CLOCK_SELECTW { w: self }
    }
    #[doc = "Bit 20 - Watchdog 1 Clock Enable"]
    #[inline]
    pub fn wdt1_clock_enable(&mut self) -> _WDT1_CLOCK_ENABLEW {
        _WDT1_CLOCK_ENABLEW { w: self }
    }
    #[doc = "Bits 21:22 - Watchdog 1 Clock Source Select"]
    #[inline]
    pub fn wdt1_clock_select(&mut self) -> _WDT1_CLOCK_SELECTW {
        _WDT1_CLOCK_SELECTW { w: self }
    }
    #[doc = "Bit 24 - ADC Clock Enable"]
    #[inline]
    pub fn adc_clock_enable(&mut self) -> _ADC_CLOCK_ENABLEW {
        _ADC_CLOCK_ENABLEW { w: self }
    }
}
