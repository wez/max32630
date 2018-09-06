#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SS_SR_POLARITY {
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
#[doc = "Possible values of the field `ss_polarity`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_POLARITYR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SS_POLARITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SS_POLARITYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SS_POLARITYR {
        match value {
            i => SS_POLARITYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `fc_polarity`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FC_POLARITYR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FC_POLARITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FC_POLARITYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FC_POLARITYR {
        match value {
            i => FC_POLARITYR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `ss_polarity`"]
pub enum SS_POLARITYW {}
impl SS_POLARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SS_POLARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _SS_POLARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS_POLARITYW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `fc_polarity`"]
pub enum FC_POLARITYW {}
impl FC_POLARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FC_POLARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _FC_POLARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FC_POLARITYW) -> &'a mut W {
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
    #[doc = "Bits 0:7 - SS Signal Polarity"]
    #[inline]
    pub fn ss_polarity(&self) -> SS_POLARITYR {
        SS_POLARITYR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - SR Signal Polarity [FC Polarity]"]
    #[inline]
    pub fn fc_polarity(&self) -> FC_POLARITYR {
        FC_POLARITYR::_from({
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
    #[doc = "Bits 0:7 - SS Signal Polarity"]
    #[inline]
    pub fn ss_polarity(&mut self) -> _SS_POLARITYW {
        _SS_POLARITYW { w: self }
    }
    #[doc = "Bits 8:15 - SR Signal Polarity [FC Polarity]"]
    #[inline]
    pub fn fc_polarity(&mut self) -> _FC_POLARITYW {
        _FC_POLARITYW { w: self }
    }
}
