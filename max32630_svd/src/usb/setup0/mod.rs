#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SETUP0 {
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
#[doc = "Possible values of the field `byte0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE0R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYTE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYTE0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYTE0R {
        match value {
            i => BYTE0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `byte1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE1R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYTE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYTE1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYTE1R {
        match value {
            i => BYTE1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `byte2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE2R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYTE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYTE2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYTE2R {
        match value {
            i => BYTE2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `byte3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE3R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYTE3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYTE3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYTE3R {
        match value {
            i => BYTE3R::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - SETUP Packet Byte 0"]
    #[inline]
    pub fn byte0(&self) -> BYTE0R {
        BYTE0R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - SETUP Packet Byte 1"]
    #[inline]
    pub fn byte1(&self) -> BYTE1R {
        BYTE1R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - SETUP Packet Byte 2"]
    #[inline]
    pub fn byte2(&self) -> BYTE2R {
        BYTE2R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - SETUP Packet Byte 3"]
    #[inline]
    pub fn byte3(&self) -> BYTE3R {
        BYTE3R::_from({
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
}
