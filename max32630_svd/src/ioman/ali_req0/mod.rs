#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ALI_REQ0 {
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
#[doc = "Possible values of the field `ali_req_p0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALI_REQ_P0R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALI_REQ_P0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALI_REQ_P0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALI_REQ_P0R {
        match value {
            i => ALI_REQ_P0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ali_req_p1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALI_REQ_P1R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALI_REQ_P1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALI_REQ_P1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALI_REQ_P1R {
        match value {
            i => ALI_REQ_P1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ali_req_p2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALI_REQ_P2R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALI_REQ_P2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALI_REQ_P2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALI_REQ_P2R {
        match value {
            i => ALI_REQ_P2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ali_req_p3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALI_REQ_P3R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALI_REQ_P3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALI_REQ_P3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALI_REQ_P3R {
        match value {
            i => ALI_REQ_P3R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `ali_req_p0`"]
pub enum ALI_REQ_P0W {}
impl ALI_REQ_P0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ALI_REQ_P0W<'a> {
    w: &'a mut W,
}
impl<'a> _ALI_REQ_P0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALI_REQ_P0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ali_req_p1`"]
pub enum ALI_REQ_P1W {}
impl ALI_REQ_P1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ALI_REQ_P1W<'a> {
    w: &'a mut W,
}
impl<'a> _ALI_REQ_P1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALI_REQ_P1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ali_req_p2`"]
pub enum ALI_REQ_P2W {}
impl ALI_REQ_P2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ALI_REQ_P2W<'a> {
    w: &'a mut W,
}
impl<'a> _ALI_REQ_P2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALI_REQ_P2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ali_req_p3`"]
pub enum ALI_REQ_P3W {}
impl ALI_REQ_P3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ALI_REQ_P3W<'a> {
    w: &'a mut W,
}
impl<'a> _ALI_REQ_P3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALI_REQ_P3W) -> &'a mut W {
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
    #[doc = "Bits 0:7 - Analog Input Mode Request: P0[7:0]"]
    #[inline]
    pub fn ali_req_p0(&self) -> ALI_REQ_P0R {
        ALI_REQ_P0R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Analog Input Mode Request: P1[7:0]"]
    #[inline]
    pub fn ali_req_p1(&self) -> ALI_REQ_P1R {
        ALI_REQ_P1R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Analog Input Mode Request: P2[7:0]"]
    #[inline]
    pub fn ali_req_p2(&self) -> ALI_REQ_P2R {
        ALI_REQ_P2R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - Analog Input Mode Request: P3[7:0]"]
    #[inline]
    pub fn ali_req_p3(&self) -> ALI_REQ_P3R {
        ALI_REQ_P3R::_from({
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
    #[doc = "Bits 0:7 - Analog Input Mode Request: P0[7:0]"]
    #[inline]
    pub fn ali_req_p0(&mut self) -> _ALI_REQ_P0W {
        _ALI_REQ_P0W { w: self }
    }
    #[doc = "Bits 8:15 - Analog Input Mode Request: P1[7:0]"]
    #[inline]
    pub fn ali_req_p1(&mut self) -> _ALI_REQ_P1W {
        _ALI_REQ_P1W { w: self }
    }
    #[doc = "Bits 16:23 - Analog Input Mode Request: P2[7:0]"]
    #[inline]
    pub fn ali_req_p2(&mut self) -> _ALI_REQ_P2W {
        _ALI_REQ_P2W { w: self }
    }
    #[doc = "Bits 24:31 - Analog Input Mode Request: P3[7:0]"]
    #[inline]
    pub fn ali_req_p3(&mut self) -> _ALI_REQ_P3W {
        _ALI_REQ_P3W { w: self }
    }
}
