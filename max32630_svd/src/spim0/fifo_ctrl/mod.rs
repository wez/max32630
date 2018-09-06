#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFO_CTRL {
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
#[doc = "Possible values of the field `tx_fifo_ae_lvl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_AE_LVLR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TX_FIFO_AE_LVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_FIFO_AE_LVLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_FIFO_AE_LVLR {
        match value {
            i => TX_FIFO_AE_LVLR::_Reserved(i),
        }
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
#[doc = "Possible values of the field `rx_fifo_af_lvl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_AF_LVLR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RX_FIFO_AF_LVLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_FIFO_AF_LVLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_FIFO_AF_LVLR {
        match value {
            i => RX_FIFO_AF_LVLR::_Reserved(i),
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
#[doc = "Values that can be written to the field `tx_fifo_ae_lvl`"]
pub enum TX_FIFO_AE_LVLW {}
impl TX_FIFO_AE_LVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_FIFO_AE_LVLW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_FIFO_AE_LVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_FIFO_AE_LVLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rx_fifo_af_lvl`"]
pub enum RX_FIFO_AF_LVLW {}
impl RX_FIFO_AF_LVLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_FIFO_AF_LVLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FIFO_AF_LVLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FIFO_AF_LVLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:3 - Transaction FIFO AE Level"]
    #[inline]
    pub fn tx_fifo_ae_lvl(&self) -> TX_FIFO_AE_LVLR {
        TX_FIFO_AE_LVLR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:12 - Transaction FIFO Used"]
    #[inline]
    pub fn tx_fifo_used(&self) -> TX_FIFO_USEDR {
        TX_FIFO_USEDR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:20 - Results FIFO AF Level"]
    #[inline]
    pub fn rx_fifo_af_lvl(&self) -> RX_FIFO_AF_LVLR {
        RX_FIFO_AF_LVLR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - Results FIFO Used"]
    #[inline]
    pub fn rx_fifo_used(&self) -> RX_FIFO_USEDR {
        RX_FIFO_USEDR::_from({
            const MASK: u8 = 63;
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
    #[doc = "Bits 0:3 - Transaction FIFO AE Level"]
    #[inline]
    pub fn tx_fifo_ae_lvl(&mut self) -> _TX_FIFO_AE_LVLW {
        _TX_FIFO_AE_LVLW { w: self }
    }
    #[doc = "Bits 16:20 - Results FIFO AF Level"]
    #[inline]
    pub fn rx_fifo_af_lvl(&mut self) -> _RX_FIFO_AF_LVLW {
        _RX_FIFO_AF_LVLW { w: self }
    }
}
