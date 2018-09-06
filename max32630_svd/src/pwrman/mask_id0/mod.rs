#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MASK_ID0 {
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
#[doc = "Possible values of the field `revision_id`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVISION_IDR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REVISION_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REVISION_IDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REVISION_IDR {
        match value {
            i => REVISION_IDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `mask_id`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_IDR {
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl MASK_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            MASK_IDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> MASK_IDR {
        match value {
            i => MASK_IDR::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Revision ID"]
    #[inline]
    pub fn revision_id(&self) -> REVISION_IDR {
        REVISION_IDR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:31 - Mask ID[27:0]"]
    #[inline]
    pub fn mask_id(&self) -> MASK_IDR {
        MASK_IDR::_from({
            const MASK: u32 = 268435455;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u32
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
}
