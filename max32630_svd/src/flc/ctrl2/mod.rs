#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL2 {
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
#[doc = "Possible values of the field `flash_lve`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_LVER {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLASH_LVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_LVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLASH_LVER {
        match value {
            i => FLASH_LVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bypass_ahb_fail`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_AHB_FAILR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYPASS_AHB_FAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYPASS_AHB_FAILR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYPASS_AHB_FAILR {
        match value {
            i => BYPASS_AHB_FAILR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `flash_lve`"]
pub enum FLASH_LVEW {}
impl FLASH_LVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLASH_LVEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_LVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH_LVEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `bypass_ahb_fail`"]
pub enum BYPASS_AHB_FAILW {}
impl BYPASS_AHB_FAILW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BYPASS_AHB_FAILW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASS_AHB_FAILW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASS_AHB_FAILW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - Flash LVE Enable"]
    #[inline]
    pub fn flash_lve(&self) -> FLASH_LVER {
        FLASH_LVER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - AHB Fail Bypass"]
    #[inline]
    pub fn bypass_ahb_fail(&self) -> BYPASS_AHB_FAILR {
        BYPASS_AHB_FAILR::_from({
            const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - Flash LVE Enable"]
    #[inline]
    pub fn flash_lve(&mut self) -> _FLASH_LVEW {
        _FLASH_LVEW { w: self }
    }
    #[doc = "Bits 8:15 - AHB Fail Bypass"]
    #[inline]
    pub fn bypass_ahb_fail(&mut self) -> _BYPASS_AHB_FAILW {
        _BYPASS_AHB_FAILW { w: self }
    }
}
