#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOOP {
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
#[doc = "Possible values of the field `counter_0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUNTER_0R {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl COUNTER_0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            COUNTER_0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> COUNTER_0R {
        match value {
            i => COUNTER_0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `counter_1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COUNTER_1R {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl COUNTER_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            COUNTER_1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> COUNTER_1R {
        match value {
            i => COUNTER_1R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `counter_0`"]
pub enum COUNTER_0W {}
impl COUNTER_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _COUNTER_0W<'a> {
    w: &'a mut W,
}
impl<'a> _COUNTER_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COUNTER_0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `counter_1`"]
pub enum COUNTER_1W {}
impl COUNTER_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _COUNTER_1W<'a> {
    w: &'a mut W,
}
impl<'a> _COUNTER_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COUNTER_1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:15 - CH1 Loop Counter 1"]
    #[inline]
    pub fn counter_0(&self) -> COUNTER_0R {
        COUNTER_0R::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:31 - CH1 Loop Counter 0"]
    #[inline]
    pub fn counter_1(&self) -> COUNTER_1R {
        COUNTER_1R::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:15 - CH1 Loop Counter 1"]
    #[inline]
    pub fn counter_0(&mut self) -> _COUNTER_0W {
        _COUNTER_0W { w: self }
    }
    #[doc = "Bits 16:31 - CH1 Loop Counter 0"]
    #[inline]
    pub fn counter_1(&mut self) -> _COUNTER_1W {
        _COUNTER_1W { w: self }
    }
}
