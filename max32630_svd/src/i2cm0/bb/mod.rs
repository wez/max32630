#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BB {
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
#[doc = "Possible values of the field `bb_scl_out`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SCL_OUTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BB_SCL_OUTR {
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
            BB_SCL_OUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_SCL_OUTR {
        match value {
            i => BB_SCL_OUTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bb_sda_out`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SDA_OUTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BB_SDA_OUTR {
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
            BB_SDA_OUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_SDA_OUTR {
        match value {
            i => BB_SDA_OUTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bb_scl_in_val`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SCL_IN_VALR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BB_SCL_IN_VALR {
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
            BB_SCL_IN_VALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_SCL_IN_VALR {
        match value {
            i => BB_SCL_IN_VALR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bb_sda_in_val`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SDA_IN_VALR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BB_SDA_IN_VALR {
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
            BB_SDA_IN_VALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_SDA_IN_VALR {
        match value {
            i => BB_SDA_IN_VALR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_fifo_cnt`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_CNTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RX_FIFO_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_FIFO_CNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_FIFO_CNTR {
        match value {
            i => RX_FIFO_CNTR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `bb_scl_out`"]
pub enum BB_SCL_OUTW {}
impl BB_SCL_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BB_SCL_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_SCL_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_SCL_OUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `bb_sda_out`"]
pub enum BB_SDA_OUTW {}
impl BB_SDA_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BB_SDA_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_SDA_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_SDA_OUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Bit Bang SCL Output"]
    #[inline]
    pub fn bb_scl_out(&self) -> BB_SCL_OUTR {
        BB_SCL_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Bit Bang SDA Output"]
    #[inline]
    pub fn bb_sda_out(&self) -> BB_SDA_OUTR {
        BB_SDA_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Bit Bang SCL Input Value"]
    #[inline]
    pub fn bb_scl_in_val(&self) -> BB_SCL_IN_VALR {
        BB_SCL_IN_VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Bit Bang SCL Input Value"]
    #[inline]
    pub fn bb_sda_in_val(&self) -> BB_SDA_IN_VALR {
        BB_SDA_IN_VALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Results FIFO Data Received Count"]
    #[inline]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNTR {
        RX_FIFO_CNTR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Bit Bang SCL Output"]
    #[inline]
    pub fn bb_scl_out(&mut self) -> _BB_SCL_OUTW {
        _BB_SCL_OUTW { w: self }
    }
    #[doc = "Bit 1 - Bit Bang SDA Output"]
    #[inline]
    pub fn bb_sda_out(&mut self) -> _BB_SDA_OUTW {
        _BB_SDA_OUTW { w: self }
    }
}
