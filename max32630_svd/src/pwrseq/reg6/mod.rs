#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG6 {
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
#[doc = "Possible values of the field `pwr_trim_usb_bias`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TRIM_USB_BIASR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_TRIM_USB_BIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_TRIM_USB_BIASR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_TRIM_USB_BIASR {
        match value {
            i => PWR_TRIM_USB_BIASR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_trim_usb_pm_res`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TRIM_USB_PM_RESR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_TRIM_USB_PM_RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_TRIM_USB_PM_RESR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_TRIM_USB_PM_RESR {
        match value {
            i => PWR_TRIM_USB_PM_RESR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_trim_usb_dm_res`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TRIM_USB_DM_RESR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_TRIM_USB_DM_RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_TRIM_USB_DM_RESR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_TRIM_USB_DM_RESR {
        match value {
            i => PWR_TRIM_USB_DM_RESR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_trim_osc_vref`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TRIM_OSC_VREFR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl PWR_TRIM_OSC_VREFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            PWR_TRIM_OSC_VREFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> PWR_TRIM_OSC_VREFR {
        match value {
            i => PWR_TRIM_OSC_VREFR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_trim_crypto_osc`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TRIM_CRYPTO_OSCR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl PWR_TRIM_CRYPTO_OSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            PWR_TRIM_CRYPTO_OSCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> PWR_TRIM_CRYPTO_OSCR {
        match value {
            i => PWR_TRIM_CRYPTO_OSCR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `pwr_trim_usb_bias`"]
pub enum PWR_TRIM_USB_BIASW {}
impl PWR_TRIM_USB_BIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TRIM_USB_BIASW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TRIM_USB_BIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TRIM_USB_BIASW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_trim_usb_pm_res`"]
pub enum PWR_TRIM_USB_PM_RESW {}
impl PWR_TRIM_USB_PM_RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TRIM_USB_PM_RESW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TRIM_USB_PM_RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TRIM_USB_PM_RESW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_trim_usb_dm_res`"]
pub enum PWR_TRIM_USB_DM_RESW {}
impl PWR_TRIM_USB_DM_RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TRIM_USB_DM_RESW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TRIM_USB_DM_RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TRIM_USB_DM_RESW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_trim_osc_vref`"]
pub enum PWR_TRIM_OSC_VREFW {}
impl PWR_TRIM_OSC_VREFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TRIM_OSC_VREFW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TRIM_OSC_VREFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TRIM_OSC_VREFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_trim_crypto_osc`"]
pub enum PWR_TRIM_CRYPTO_OSCW {}
impl PWR_TRIM_CRYPTO_OSCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TRIM_CRYPTO_OSCW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TRIM_CRYPTO_OSCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TRIM_CRYPTO_OSCW) -> &'a mut W {
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
    #[doc = "Bits 0:2 - USB Bias Current trim setting"]
    #[inline]
    pub fn pwr_trim_usb_bias(&self) -> PWR_TRIM_USB_BIASR {
        PWR_TRIM_USB_BIASR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:6 - USB Data Plus Slew Rate trim setting"]
    #[inline]
    pub fn pwr_trim_usb_pm_res(&self) -> PWR_TRIM_USB_PM_RESR {
        PWR_TRIM_USB_PM_RESR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 7:10 - USB Data Minus Slew Rate trim setting"]
    #[inline]
    pub fn pwr_trim_usb_dm_res(&self) -> PWR_TRIM_USB_DM_RESR {
        PWR_TRIM_USB_DM_RESR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:19 - Relaxation Oscillator trim setting"]
    #[inline]
    pub fn pwr_trim_osc_vref(&self) -> PWR_TRIM_OSC_VREFR {
        PWR_TRIM_OSC_VREFR::_from({
            const MASK: u16 = 511;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 20:28 - Crypto Oscillator trim setting"]
    #[inline]
    pub fn pwr_trim_crypto_osc(&self) -> PWR_TRIM_CRYPTO_OSCR {
        PWR_TRIM_CRYPTO_OSCR::_from({
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
    #[doc = "Bits 0:2 - USB Bias Current trim setting"]
    #[inline]
    pub fn pwr_trim_usb_bias(&mut self) -> _PWR_TRIM_USB_BIASW {
        _PWR_TRIM_USB_BIASW { w: self }
    }
    #[doc = "Bits 3:6 - USB Data Plus Slew Rate trim setting"]
    #[inline]
    pub fn pwr_trim_usb_pm_res(&mut self) -> _PWR_TRIM_USB_PM_RESW {
        _PWR_TRIM_USB_PM_RESW { w: self }
    }
    #[doc = "Bits 7:10 - USB Data Minus Slew Rate trim setting"]
    #[inline]
    pub fn pwr_trim_usb_dm_res(&mut self) -> _PWR_TRIM_USB_DM_RESW {
        _PWR_TRIM_USB_DM_RESW { w: self }
    }
    #[doc = "Bits 11:19 - Relaxation Oscillator trim setting"]
    #[inline]
    pub fn pwr_trim_osc_vref(&mut self) -> _PWR_TRIM_OSC_VREFW {
        _PWR_TRIM_OSC_VREFW { w: self }
    }
    #[doc = "Bits 20:28 - Crypto Oscillator trim setting"]
    #[inline]
    pub fn pwr_trim_crypto_osc(&mut self) -> _PWR_TRIM_CRYPTO_OSCW {
        _PWR_TRIM_CRYPTO_OSCW { w: self }
    }
}
