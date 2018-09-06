#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SECURITY {
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
#[doc = "Possible values of the field `debug_disable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUG_DISABLER {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DEBUG_DISABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DEBUG_DISABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DEBUG_DISABLER {
        match value {
            i => DEBUG_DISABLER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `mass_erase_lock`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASS_ERASE_LOCKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MASS_ERASE_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MASS_ERASE_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MASS_ERASE_LOCKR {
        match value {
            i => MASS_ERASE_LOCKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `disable_ahb_wr`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_AHB_WRR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DISABLE_AHB_WRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DISABLE_AHB_WRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DISABLE_AHB_WRR {
        match value {
            i => DISABLE_AHB_WRR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `flc_settings_lock`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLC_SETTINGS_LOCKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLC_SETTINGS_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLC_SETTINGS_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLC_SETTINGS_LOCKR {
        match value {
            i => FLC_SETTINGS_LOCKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `security_lock`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECURITY_LOCKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SECURITY_LOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SECURITY_LOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SECURITY_LOCKR {
        match value {
            i => SECURITY_LOCKR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `debug_disable`"]
pub enum DEBUG_DISABLEW {}
impl DEBUG_DISABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DEBUG_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEBUG_DISABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DEBUG_DISABLEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `mass_erase_lock`"]
pub enum MASS_ERASE_LOCKW {}
impl MASS_ERASE_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MASS_ERASE_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _MASS_ERASE_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASS_ERASE_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `disable_ahb_wr`"]
pub enum DISABLE_AHB_WRW {}
impl DISABLE_AHB_WRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_AHB_WRW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_AHB_WRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLE_AHB_WRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `flc_settings_lock`"]
pub enum FLC_SETTINGS_LOCKW {}
impl FLC_SETTINGS_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLC_SETTINGS_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _FLC_SETTINGS_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLC_SETTINGS_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `security_lock`"]
pub enum SECURITY_LOCKW {}
impl SECURITY_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SECURITY_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _SECURITY_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SECURITY_LOCKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:7 - Debug Lockout"]
    #[inline]
    pub fn debug_disable(&self) -> DEBUG_DISABLER {
        DEBUG_DISABLER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Mass Erase Lockout"]
    #[inline]
    pub fn mass_erase_lock(&self) -> MASS_ERASE_LOCKR {
        MASS_ERASE_LOCKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Disable AHB Flash Write Operations"]
    #[inline]
    pub fn disable_ahb_wr(&self) -> DISABLE_AHB_WRR {
        DISABLE_AHB_WRR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - FLC Settings Lock"]
    #[inline]
    pub fn flc_settings_lock(&self) -> FLC_SETTINGS_LOCKR {
        FLC_SETTINGS_LOCKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Security Lock"]
    #[inline]
    pub fn security_lock(&self) -> SECURITY_LOCKR {
        SECURITY_LOCKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:7 - Debug Lockout"]
    #[inline]
    pub fn debug_disable(&mut self) -> _DEBUG_DISABLEW {
        _DEBUG_DISABLEW { w: self }
    }
    #[doc = "Bits 8:11 - Mass Erase Lockout"]
    #[inline]
    pub fn mass_erase_lock(&mut self) -> _MASS_ERASE_LOCKW {
        _MASS_ERASE_LOCKW { w: self }
    }
    #[doc = "Bits 16:19 - Disable AHB Flash Write Operations"]
    #[inline]
    pub fn disable_ahb_wr(&mut self) -> _DISABLE_AHB_WRW {
        _DISABLE_AHB_WRW { w: self }
    }
    #[doc = "Bits 24:27 - FLC Settings Lock"]
    #[inline]
    pub fn flc_settings_lock(&mut self) -> _FLC_SETTINGS_LOCKW {
        _FLC_SETTINGS_LOCKW { w: self }
    }
    #[doc = "Bits 28:31 - Security Lock"]
    #[inline]
    pub fn security_lock(&mut self) -> _SECURITY_LOCKW {
        _SECURITY_LOCKW { w: self }
    }
}
