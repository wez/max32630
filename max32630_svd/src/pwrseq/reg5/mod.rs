#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG5 {
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
#[doc = "Possible values of the field `pwr_trim_svm_bg`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TRIM_SVM_BGR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl PWR_TRIM_SVM_BGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            PWR_TRIM_SVM_BGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> PWR_TRIM_SVM_BGR {
        match value {
            i => PWR_TRIM_SVM_BGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_trim_bias`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TRIM_BIASR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_TRIM_BIASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_TRIM_BIASR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_TRIM_BIASR {
        match value {
            i => PWR_TRIM_BIASR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_trim_retreg`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TRIM_RETREGR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_TRIM_RETREGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_TRIM_RETREGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_TRIM_RETREGR {
        match value {
            i => PWR_TRIM_RETREGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_rtc_trim`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_RTC_TRIMR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_RTC_TRIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_RTC_TRIMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_RTC_TRIMR {
        match value {
            i => PWR_RTC_TRIMR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `pwr_trim_svm_bg`"]
pub enum PWR_TRIM_SVM_BGW {}
impl PWR_TRIM_SVM_BGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TRIM_SVM_BGW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TRIM_SVM_BGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TRIM_SVM_BGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_trim_bias`"]
pub enum PWR_TRIM_BIASW {}
impl PWR_TRIM_BIASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TRIM_BIASW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TRIM_BIASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TRIM_BIASW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_trim_retreg`"]
pub enum PWR_TRIM_RETREGW {}
impl PWR_TRIM_RETREGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TRIM_RETREGW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TRIM_RETREGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TRIM_RETREGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_rtc_trim`"]
pub enum PWR_RTC_TRIMW {}
impl PWR_RTC_TRIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_RTC_TRIMW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_RTC_TRIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_RTC_TRIMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:8 - Power Manager Bandgap trim setting"]
    #[inline]
    pub fn pwr_trim_svm_bg(&self) -> PWR_TRIM_SVM_BGR {
        PWR_TRIM_SVM_BGR::_from({
            const MASK: u16 = 511;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 9:14 - Power Manager Bias Current trim setting"]
    #[inline]
    pub fn pwr_trim_bias(&self) -> PWR_TRIM_BIASR {
        PWR_TRIM_BIASR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 15:20 - Retention Regulator trim setting"]
    #[inline]
    pub fn pwr_trim_retreg(&self) -> PWR_TRIM_RETREGR {
        PWR_TRIM_RETREGR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 21:24 - Real Time Clock trim setting"]
    #[inline]
    pub fn pwr_rtc_trim(&self) -> PWR_RTC_TRIMR {
        PWR_RTC_TRIMR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 21;
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
    #[doc = "Bits 0:8 - Power Manager Bandgap trim setting"]
    #[inline]
    pub fn pwr_trim_svm_bg(&mut self) -> _PWR_TRIM_SVM_BGW {
        _PWR_TRIM_SVM_BGW { w: self }
    }
    #[doc = "Bits 9:14 - Power Manager Bias Current trim setting"]
    #[inline]
    pub fn pwr_trim_bias(&mut self) -> _PWR_TRIM_BIASW {
        _PWR_TRIM_BIASW { w: self }
    }
    #[doc = "Bits 15:20 - Retention Regulator trim setting"]
    #[inline]
    pub fn pwr_trim_retreg(&mut self) -> _PWR_TRIM_RETREGW {
        _PWR_TRIM_RETREGW { w: self }
    }
    #[doc = "Bits 21:24 - Real Time Clock trim setting"]
    #[inline]
    pub fn pwr_rtc_trim(&mut self) -> _PWR_RTC_TRIMW {
        _PWR_RTC_TRIMW { w: self }
    }
}
