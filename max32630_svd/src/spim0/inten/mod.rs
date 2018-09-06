#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN {
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
#[doc = "Possible values of the field `tx_stalled`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_STALLEDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_STALLEDR {
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
            TX_STALLEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_STALLEDR {
        match value {
            i => TX_STALLEDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_stalled`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_STALLEDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_STALLEDR {
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
            RX_STALLEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_STALLEDR {
        match value {
            i => RX_STALLEDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tx_ready`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_READYR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_READYR {
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
            TX_READYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_READYR {
        match value {
            i => TX_READYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_done`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DONER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_DONER {
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
            RX_DONER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DONER {
        match value {
            i => RX_DONER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tx_fifo_ae`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_AER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_FIFO_AER {
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
            TX_FIFO_AER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_FIFO_AER {
        match value {
            i => TX_FIFO_AER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_fifo_af`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_AFR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_FIFO_AFR {
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
            RX_FIFO_AFR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FIFO_AFR {
        match value {
            i => RX_FIFO_AFR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `tx_stalled`"]
pub enum TX_STALLEDW {}
impl TX_STALLEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_STALLEDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_STALLEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_STALLEDW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `rx_stalled`"]
pub enum RX_STALLEDW {}
impl RX_STALLEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_STALLEDW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_STALLEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_STALLEDW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `tx_ready`"]
pub enum TX_READYW {}
impl TX_READYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_READYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_READYW) -> &'a mut W {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rx_done`"]
pub enum RX_DONEW {}
impl RX_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DONEW) -> &'a mut W {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `tx_fifo_ae`"]
pub enum TX_FIFO_AEW {}
impl TX_FIFO_AEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_FIFO_AEW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_FIFO_AEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_FIFO_AEW) -> &'a mut W {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rx_fifo_af`"]
pub enum RX_FIFO_AFW {}
impl RX_FIFO_AFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_FIFO_AFW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FIFO_AFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FIFO_AFW) -> &'a mut W {
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Transaction Stalled Int Enable"]
    #[inline]
    pub fn tx_stalled(&self) -> TX_STALLEDR {
        TX_STALLEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Results Stalled Int Enable"]
    #[inline]
    pub fn rx_stalled(&self) -> RX_STALLEDR {
        RX_STALLEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Transaction Ready Int Enable"]
    #[inline]
    pub fn tx_ready(&self) -> TX_READYR {
        TX_READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Results Done Int Enable"]
    #[inline]
    pub fn rx_done(&self) -> RX_DONER {
        RX_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - TXFIFO Almost Empty Int Enable"]
    #[inline]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AER {
        TX_FIFO_AER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RXFIFO Almost Full Int Enable"]
    #[inline]
    pub fn rx_fifo_af(&self) -> RX_FIFO_AFR {
        RX_FIFO_AFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Transaction Stalled Int Enable"]
    #[inline]
    pub fn tx_stalled(&mut self) -> _TX_STALLEDW {
        _TX_STALLEDW { w: self }
    }
    #[doc = "Bit 1 - Results Stalled Int Enable"]
    #[inline]
    pub fn rx_stalled(&mut self) -> _RX_STALLEDW {
        _RX_STALLEDW { w: self }
    }
    #[doc = "Bit 2 - Transaction Ready Int Enable"]
    #[inline]
    pub fn tx_ready(&mut self) -> _TX_READYW {
        _TX_READYW { w: self }
    }
    #[doc = "Bit 3 - Results Done Int Enable"]
    #[inline]
    pub fn rx_done(&mut self) -> _RX_DONEW {
        _RX_DONEW { w: self }
    }
    #[doc = "Bit 4 - TXFIFO Almost Empty Int Enable"]
    #[inline]
    pub fn tx_fifo_ae(&mut self) -> _TX_FIFO_AEW {
        _TX_FIFO_AEW { w: self }
    }
    #[doc = "Bit 5 - RXFIFO Almost Full Int Enable"]
    #[inline]
    pub fn rx_fifo_af(&mut self) -> _RX_FIFO_AFW {
        _RX_FIFO_AFW { w: self }
    }
}
