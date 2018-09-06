#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SETUP1 {
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
#[doc = "Possible values of the field `byte4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE4R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYTE4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYTE4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYTE4R {
        match value {
            i => BYTE4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `byte5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE5R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYTE5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYTE5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYTE5R {
        match value {
            i => BYTE5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `byte6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE6R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYTE6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYTE6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYTE6R {
        match value {
            i => BYTE6R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `byte7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYTE7R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BYTE7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BYTE7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BYTE7R {
        match value {
            i => BYTE7R::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - SETUP Packet Byte 4"]
    #[inline]
    pub fn byte4(&self) -> BYTE4R {
        BYTE4R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - SETUP Packet Byte 5"]
    #[inline]
    pub fn byte5(&self) -> BYTE5R {
        BYTE5R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - SETUP Packet Byte 6"]
    #[inline]
    pub fn byte6(&self) -> BYTE6R {
        BYTE6R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - SETUP Packet Byte 7"]
    #[inline]
    pub fn byte7(&self) -> BYTE7R {
        BYTE7R::_from({
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
