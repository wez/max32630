#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UART0_ACK {
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
#[doc = "Possible values of the field `io_map`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_MAPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IO_MAPR {
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
            IO_MAPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IO_MAPR {
        match value {
            i => IO_MAPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `cts_map`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_MAPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CTS_MAPR {
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
            CTS_MAPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTS_MAPR {
        match value {
            i => CTS_MAPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rts_map`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTS_MAPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTS_MAPR {
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
            RTS_MAPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTS_MAPR {
        match value {
            i => RTS_MAPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `io_req`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_REQR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IO_REQR {
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
            IO_REQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IO_REQR {
        match value {
            i => IO_REQR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `cts_io_req`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_IO_REQR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CTS_IO_REQR {
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
            CTS_IO_REQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTS_IO_REQR {
        match value {
            i => CTS_IO_REQR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rts_io_req`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTS_IO_REQR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTS_IO_REQR {
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
            RTS_IO_REQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTS_IO_REQR {
        match value {
            i => RTS_IO_REQR::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - UART0 TX/RX I/O Mapping Acknowledge"]
    #[inline]
    pub fn io_map(&self) -> IO_MAPR {
        IO_MAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - UART0 CTS I/O Mapping Acknowledge"]
    #[inline]
    pub fn cts_map(&self) -> CTS_MAPR {
        CTS_MAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - UART0 RTS I/O Mapping Acknowledge"]
    #[inline]
    pub fn rts_map(&self) -> RTS_MAPR {
        RTS_MAPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - UART0 TX/RX I/O Acknowledge"]
    #[inline]
    pub fn io_req(&self) -> IO_REQR {
        IO_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - UART0 CTS I/O Acknowledge"]
    #[inline]
    pub fn cts_io_req(&self) -> CTS_IO_REQR {
        CTS_IO_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - UART0 RTS I/O Acknowledge"]
    #[inline]
    pub fn rts_io_req(&self) -> RTS_IO_REQR {
        RTS_IO_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
