#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WUD_REQ1 {
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
#[doc = "Possible values of the field `wud_req_p4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUD_REQ_P4R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WUD_REQ_P4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUD_REQ_P4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUD_REQ_P4R {
        match value {
            i => WUD_REQ_P4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `wud_req_p5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUD_REQ_P5R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WUD_REQ_P5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUD_REQ_P5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUD_REQ_P5R {
        match value {
            i => WUD_REQ_P5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `wud_req_p6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUD_REQ_P6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WUD_REQ_P6R {
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
            WUD_REQ_P6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUD_REQ_P6R {
        match value {
            i => WUD_REQ_P6R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `wud_req_p4`"]
pub enum WUD_REQ_P4W {}
impl WUD_REQ_P4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WUD_REQ_P4W<'a> {
    w: &'a mut W,
}
impl<'a> _WUD_REQ_P4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUD_REQ_P4W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `wud_req_p5`"]
pub enum WUD_REQ_P5W {}
impl WUD_REQ_P5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WUD_REQ_P5W<'a> {
    w: &'a mut W,
}
impl<'a> _WUD_REQ_P5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUD_REQ_P5W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `wud_req_p6`"]
pub enum WUD_REQ_P6W {}
impl WUD_REQ_P6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WUD_REQ_P6W<'a> {
    w: &'a mut W,
}
impl<'a> _WUD_REQ_P6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUD_REQ_P6W) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Wakeup Detect Request Mode: P4[7:0]"]
    #[inline]
    pub fn wud_req_p4(&self) -> WUD_REQ_P4R {
        WUD_REQ_P4R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Wakeup Detect Request Mode: P5[7:0]"]
    #[inline]
    pub fn wud_req_p5(&self) -> WUD_REQ_P5R {
        WUD_REQ_P5R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Wakeup Detect Request Mode: P6[0]"]
    #[inline]
    pub fn wud_req_p6(&self) -> WUD_REQ_P6R {
        WUD_REQ_P6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:7 - Wakeup Detect Request Mode: P4[7:0]"]
    #[inline]
    pub fn wud_req_p4(&mut self) -> _WUD_REQ_P4W {
        _WUD_REQ_P4W { w: self }
    }
    #[doc = "Bits 8:15 - Wakeup Detect Request Mode: P5[7:0]"]
    #[inline]
    pub fn wud_req_p5(&mut self) -> _WUD_REQ_P5W {
        _WUD_REQ_P5W { w: self }
    }
    #[doc = "Bit 16 - Wakeup Detect Request Mode: P6[0]"]
    #[inline]
    pub fn wud_req_p6(&mut self) -> _WUD_REQ_P6W {
        _WUD_REQ_P6W { w: self }
    }
}
