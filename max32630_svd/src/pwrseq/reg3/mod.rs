#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG3 {
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
#[doc = "Possible values of the field `pwr_rosel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_ROSELR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_ROSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_ROSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_ROSELR {
        match value {
            i => PWR_ROSELR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_fltrrosel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_FLTRROSELR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_FLTRROSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_FLTRROSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_FLTRROSELR {
        match value {
            i => PWR_FLTRROSELR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_svm_clk_mux`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_SVM_CLK_MUXR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_SVM_CLK_MUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_SVM_CLK_MUXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_SVM_CLK_MUXR {
        match value {
            i => PWR_SVM_CLK_MUXR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_ro_clk_mux`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_RO_CLK_MUXR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_RO_CLK_MUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_RO_CLK_MUXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_RO_CLK_MUXR {
        match value {
            i => PWR_RO_CLK_MUXR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_failsel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_FAILSELR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PWR_FAILSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PWR_FAILSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PWR_FAILSELR {
        match value {
            i => PWR_FAILSELR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `pwr_rosel`"]
pub enum PWR_ROSELW {}
impl PWR_ROSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_ROSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_ROSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_ROSELW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_fltrrosel`"]
pub enum PWR_FLTRROSELW {}
impl PWR_FLTRROSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_FLTRROSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_FLTRROSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_FLTRROSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_svm_clk_mux`"]
pub enum PWR_SVM_CLK_MUXW {}
impl PWR_SVM_CLK_MUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_SVM_CLK_MUXW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_SVM_CLK_MUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_SVM_CLK_MUXW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_ro_clk_mux`"]
pub enum PWR_RO_CLK_MUXW {}
impl PWR_RO_CLK_MUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_RO_CLK_MUXW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_RO_CLK_MUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_RO_CLK_MUXW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_failsel`"]
pub enum PWR_FAILSELW {}
impl PWR_FAILSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_FAILSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_FAILSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_FAILSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:2 - Relaxation Oscillator Stable Timeout"]
    #[inline]
    pub fn pwr_rosel(&self) -> PWR_ROSELR {
        PWR_ROSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Window of time power must be valid before entering Run mode."]
    #[inline]
    pub fn pwr_fltrrosel(&self) -> PWR_FLTRROSELR {
        PWR_FLTRROSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - SVM Clock Mux"]
    #[inline]
    pub fn pwr_svm_clk_mux(&self) -> PWR_SVM_CLK_MUXR {
        PWR_SVM_CLK_MUXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Relaxation Clock Mux"]
    #[inline]
    pub fn pwr_ro_clk_mux(&self) -> PWR_RO_CLK_MUXR {
        PWR_RO_CLK_MUXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:12 - Timeout before rebooting during PowerFail/BootFail events."]
    #[inline]
    pub fn pwr_failsel(&self) -> PWR_FAILSELR {
        PWR_FAILSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:2 - Relaxation Oscillator Stable Timeout"]
    #[inline]
    pub fn pwr_rosel(&mut self) -> _PWR_ROSELW {
        _PWR_ROSELW { w: self }
    }
    #[doc = "Bits 3:5 - Window of time power must be valid before entering Run mode."]
    #[inline]
    pub fn pwr_fltrrosel(&mut self) -> _PWR_FLTRROSELW {
        _PWR_FLTRROSELW { w: self }
    }
    #[doc = "Bits 6:7 - SVM Clock Mux"]
    #[inline]
    pub fn pwr_svm_clk_mux(&mut self) -> _PWR_SVM_CLK_MUXW {
        _PWR_SVM_CLK_MUXW { w: self }
    }
    #[doc = "Bits 8:9 - Relaxation Clock Mux"]
    #[inline]
    pub fn pwr_ro_clk_mux(&mut self) -> _PWR_RO_CLK_MUXW {
        _PWR_RO_CLK_MUXW { w: self }
    }
    #[doc = "Bits 10:12 - Timeout before rebooting during PowerFail/BootFail events."]
    #[inline]
    pub fn pwr_failsel(&mut self) -> _PWR_FAILSELW {
        _PWR_FAILSELW { w: self }
    }
}
