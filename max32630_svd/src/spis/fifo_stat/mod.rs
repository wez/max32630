#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFO_STAT {
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
#[doc = "Possible values of the field `tx_fifo_used`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_USEDR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TX_FIFO_USEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_FIFO_USEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_FIFO_USEDR {
        match value {
            i => TX_FIFO_USEDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_fifo_used`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_USEDR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RX_FIFO_USEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_FIFO_USEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_FIFO_USEDR {
        match value {
            i => RX_FIFO_USEDR::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Number of Bytes in Transmit FIFO"]
    #[inline]
    pub fn tx_fifo_used(&self) -> TX_FIFO_USEDR {
        TX_FIFO_USEDR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - Number of Bytes in Receive FIFO"]
    #[inline]
    pub fn rx_fifo_used(&self) -> RX_FIFO_USEDR {
        RX_FIFO_USEDR::_from({
            const MASK: u8 = 63;
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
}
