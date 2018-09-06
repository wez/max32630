#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ALI_ACK1 {
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
#[doc = "Possible values of the field `ali_ack_p4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALI_ACK_P4R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALI_ACK_P4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALI_ACK_P4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALI_ACK_P4R {
        match value {
            i => ALI_ACK_P4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ali_ack_p5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALI_ACK_P5R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALI_ACK_P5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALI_ACK_P5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALI_ACK_P5R {
        match value {
            i => ALI_ACK_P5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ali_ack_p6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALI_ACK_P6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ALI_ACK_P6R {
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
            ALI_ACK_P6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALI_ACK_P6R {
        match value {
            i => ALI_ACK_P6R::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Analog In Mode Acknowledge: P4[7:0]"]
    #[inline]
    pub fn ali_ack_p4(&self) -> ALI_ACK_P4R {
        ALI_ACK_P4R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Analog In Mode Acknowledge: P5[7:0]"]
    #[inline]
    pub fn ali_ack_p5(&self) -> ALI_ACK_P5R {
        ALI_ACK_P5R::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Analog In Mode Acknowledge: P6[0]"]
    #[inline]
    pub fn ali_ack_p6(&self) -> ALI_ACK_P6R {
        ALI_ACK_P6R::_from({
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
