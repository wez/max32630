#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WUD_ACK1 {
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
#[doc = "Possible values of the field `wud_ack_p4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUD_ACK_P4R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WUD_ACK_P4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUD_ACK_P4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUD_ACK_P4R {
        match value {
            i => WUD_ACK_P4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `wud_ack_p5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUD_ACK_P5R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WUD_ACK_P5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUD_ACK_P5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUD_ACK_P5R {
        match value {
            i => WUD_ACK_P5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `wud_ack_p6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUD_ACK_P6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WUD_ACK_P6R {
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
            WUD_ACK_P6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUD_ACK_P6R {
        match value {
            i => WUD_ACK_P6R::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - WUD Mode Acknowledge: P4[7:0]"]
    #[inline]
    pub fn wud_ack_p4(&self) -> WUD_ACK_P4R {
        WUD_ACK_P4R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - WUD Mode Acknowledge: P5[7:0]"]
    #[inline]
    pub fn wud_ack_p5(&self) -> WUD_ACK_P5R {
        WUD_ACK_P5R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - WUD Mode Acknowledge: P6[7:0]"]
    #[inline]
    pub fn wud_ack_p6(&self) -> WUD_ACK_P6R {
        WUD_ACK_P6R::_from({
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
}
