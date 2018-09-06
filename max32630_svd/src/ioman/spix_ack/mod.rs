#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPIX_ACK {
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
#[doc = "Possible values of the field `core_io_ack`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE_IO_ACKR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CORE_IO_ACKR {
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
            CORE_IO_ACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CORE_IO_ACKR {
        match value {
            i => CORE_IO_ACKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ss0_io_ack`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS0_IO_ACKR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SS0_IO_ACKR {
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
            SS0_IO_ACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SS0_IO_ACKR {
        match value {
            i => SS0_IO_ACKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ss1_io_ack`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS1_IO_ACKR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SS1_IO_ACKR {
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
            SS1_IO_ACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SS1_IO_ACKR {
        match value {
            i => SS1_IO_ACKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ss2_io_ack`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS2_IO_ACKR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SS2_IO_ACKR {
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
            SS2_IO_ACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SS2_IO_ACKR {
        match value {
            i => SS2_IO_ACKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `quad_io_ack`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUAD_IO_ACKR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl QUAD_IO_ACKR {
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
            QUAD_IO_ACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QUAD_IO_ACKR {
        match value {
            i => QUAD_IO_ACKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `fast_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FAST_MODER {
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
            FAST_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAST_MODER {
        match value {
            i => FAST_MODER::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 4 - SPIX Core I/O Acknowledge"]
    #[inline]
    pub fn core_io_ack(&self) -> CORE_IO_ACKR {
        CORE_IO_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - SPIX SS[0] I/O Acknowledge"]
    #[inline]
    pub fn ss0_io_ack(&self) -> SS0_IO_ACKR {
        SS0_IO_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - SPIX SS[1] I/O Acknowledge"]
    #[inline]
    pub fn ss1_io_ack(&self) -> SS1_IO_ACKR {
        SS1_IO_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - SPIX SS[2] I/O Acknowledge"]
    #[inline]
    pub fn ss2_io_ack(&self) -> SS2_IO_ACKR {
        SS2_IO_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - SPIX Quad I/O Acknowledge"]
    #[inline]
    pub fn quad_io_ack(&self) -> QUAD_IO_ACKR {
        QUAD_IO_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - SPIX Fast Mode Acknowledge"]
    #[inline]
    pub fn fast_mode(&self) -> FAST_MODER {
        FAST_MODER::_from({
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
