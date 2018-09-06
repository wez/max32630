#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FS_CLK_DIV {
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
#[doc = "Possible values of the field `fs_filter_clk_div`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS_FILTER_CLK_DIVR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FS_FILTER_CLK_DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FS_FILTER_CLK_DIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FS_FILTER_CLK_DIVR {
        match value {
            i => FS_FILTER_CLK_DIVR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `fs_scl_lo_cnt`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS_SCL_LO_CNTR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl FS_SCL_LO_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            FS_SCL_LO_CNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> FS_SCL_LO_CNTR {
        match value {
            i => FS_SCL_LO_CNTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `fs_scl_hi_cnt`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FS_SCL_HI_CNTR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl FS_SCL_HI_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            FS_SCL_HI_CNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> FS_SCL_HI_CNTR {
        match value {
            i => FS_SCL_HI_CNTR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `fs_filter_clk_div`"]
pub enum FS_FILTER_CLK_DIVW {}
impl FS_FILTER_CLK_DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FS_FILTER_CLK_DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _FS_FILTER_CLK_DIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FS_FILTER_CLK_DIVW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `fs_scl_lo_cnt`"]
pub enum FS_SCL_LO_CNTW {}
impl FS_SCL_LO_CNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FS_SCL_LO_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FS_SCL_LO_CNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FS_SCL_LO_CNTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `fs_scl_hi_cnt`"]
pub enum FS_SCL_HI_CNTW {}
impl FS_SCL_HI_CNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FS_SCL_HI_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _FS_SCL_HI_CNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FS_SCL_HI_CNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
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
    #[doc = "Bits 0:7 - Full Speed Filter Clock Divisor"]
    #[inline]
    pub fn fs_filter_clk_div(&self) -> FS_FILTER_CLK_DIVR {
        FS_FILTER_CLK_DIVR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:19 - Full Speed SCL Low Count"]
    #[inline]
    pub fn fs_scl_lo_cnt(&self) -> FS_SCL_LO_CNTR {
        FS_SCL_LO_CNTR::_from({
            const MASK: u16 = 4095;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 20:31 - Full Speed SCL High Count"]
    #[inline]
    pub fn fs_scl_hi_cnt(&self) -> FS_SCL_HI_CNTR {
        FS_SCL_HI_CNTR::_from({
            const MASK: u16 = 4095;
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
    #[doc = "Bits 0:7 - Full Speed Filter Clock Divisor"]
    #[inline]
    pub fn fs_filter_clk_div(&mut self) -> _FS_FILTER_CLK_DIVW {
        _FS_FILTER_CLK_DIVW { w: self }
    }
    #[doc = "Bits 8:19 - Full Speed SCL Low Count"]
    #[inline]
    pub fn fs_scl_lo_cnt(&mut self) -> _FS_SCL_LO_CNTW {
        _FS_SCL_LO_CNTW { w: self }
    }
    #[doc = "Bits 20:31 - Full Speed SCL High Count"]
    #[inline]
    pub fn fs_scl_hi_cnt(&mut self) -> _FS_SCL_HI_CNTW {
        _FS_SCL_HI_CNTW { w: self }
    }
}
