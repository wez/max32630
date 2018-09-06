#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WUD_ACK0 {
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
#[doc = "Possible values of the field `wud_ack_p0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUD_ACK_P0R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WUD_ACK_P0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUD_ACK_P0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUD_ACK_P0R {
        match value {
            i => WUD_ACK_P0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `wud_ack_p1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUD_ACK_P1R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WUD_ACK_P1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUD_ACK_P1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUD_ACK_P1R {
        match value {
            i => WUD_ACK_P1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `wud_ack_p2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUD_ACK_P2R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WUD_ACK_P2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUD_ACK_P2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUD_ACK_P2R {
        match value {
            i => WUD_ACK_P2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `wud_ack_p3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUD_ACK_P3R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WUD_ACK_P3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WUD_ACK_P3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WUD_ACK_P3R {
        match value {
            i => WUD_ACK_P3R::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - WUD Mode Acknowledge: P0[7:0]"]
    #[inline]
    pub fn wud_ack_p0(&self) -> WUD_ACK_P0R {
        WUD_ACK_P0R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - WUD Mode Acknowledge: P1[7:0]"]
    #[inline]
    pub fn wud_ack_p1(&self) -> WUD_ACK_P1R {
        WUD_ACK_P1R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - WUD Mode Acknowledge: P2[7:0]"]
    #[inline]
    pub fn wud_ack_p2(&self) -> WUD_ACK_P2R {
        WUD_ACK_P2R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - WUD Mode Acknowledge: P3[7:0]"]
    #[inline]
    pub fn wud_ack_p3(&self) -> WUD_ACK_P3R {
        WUD_ACK_P3R::_from({
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
