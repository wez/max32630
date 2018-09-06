#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTFL {
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
#[doc = "Possible values of the field `ow_reset_done`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OW_RESET_DONER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OW_RESET_DONER {
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
            OW_RESET_DONER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OW_RESET_DONER {
        match value {
            i => OW_RESET_DONER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tx_data_empty`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DATA_EMPTYR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_DATA_EMPTYR {
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
            TX_DATA_EMPTYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_DATA_EMPTYR {
        match value {
            i => TX_DATA_EMPTYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_data_ready`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_DATA_READYR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_DATA_READYR {
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
            RX_DATA_READYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_DATA_READYR {
        match value {
            i => RX_DATA_READYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `line_short`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE_SHORTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl LINE_SHORTR {
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
            LINE_SHORTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE_SHORTR {
        match value {
            i => LINE_SHORTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `line_low`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE_LOWR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl LINE_LOWR {
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
            LINE_LOWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE_LOWR {
        match value {
            i => LINE_LOWR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `ow_reset_done`"]
pub enum OW_RESET_DONEW {}
impl OW_RESET_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OW_RESET_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _OW_RESET_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OW_RESET_DONEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `tx_data_empty`"]
pub enum TX_DATA_EMPTYW {}
impl TX_DATA_EMPTYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_DATA_EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DATA_EMPTYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_DATA_EMPTYW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `rx_data_ready`"]
pub enum RX_DATA_READYW {}
impl RX_DATA_READYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_DATA_READYW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_DATA_READYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_DATA_READYW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `line_short`"]
pub enum LINE_SHORTW {}
impl LINE_SHORTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _LINE_SHORTW<'a> {
    w: &'a mut W,
}
impl<'a> _LINE_SHORTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE_SHORTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `line_low`"]
pub enum LINE_LOWW {}
impl LINE_LOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _LINE_LOWW<'a> {
    w: &'a mut W,
}
impl<'a> _LINE_LOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE_LOWW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - OW Reset Sequence Completed"]
    #[inline]
    pub fn ow_reset_done(&self) -> OW_RESET_DONER {
        OW_RESET_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Tx Data Empty Interrupt Flag"]
    #[inline]
    pub fn tx_data_empty(&self) -> TX_DATA_EMPTYR {
        TX_DATA_EMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Rx Data Ready Interrupt Flag"]
    #[inline]
    pub fn rx_data_ready(&self) -> RX_DATA_READYR {
        RX_DATA_READYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Flag"]
    #[inline]
    pub fn line_short(&self) -> LINE_SHORTR {
        LINE_SHORTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Flag"]
    #[inline]
    pub fn line_low(&self) -> LINE_LOWR {
        LINE_LOWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - OW Reset Sequence Completed"]
    #[inline]
    pub fn ow_reset_done(&mut self) -> _OW_RESET_DONEW {
        _OW_RESET_DONEW { w: self }
    }
    #[doc = "Bit 1 - Tx Data Empty Interrupt Flag"]
    #[inline]
    pub fn tx_data_empty(&mut self) -> _TX_DATA_EMPTYW {
        _TX_DATA_EMPTYW { w: self }
    }
    #[doc = "Bit 2 - Rx Data Ready Interrupt Flag"]
    #[inline]
    pub fn rx_data_ready(&mut self) -> _RX_DATA_READYW {
        _RX_DATA_READYW { w: self }
    }
    #[doc = "Bit 3 - OW Line Short Detected Interrupt Flag"]
    #[inline]
    pub fn line_short(&mut self) -> _LINE_SHORTW {
        _LINE_SHORTW { w: self }
    }
    #[doc = "Bit 4 - OW Line Low Detected Interrupt Flag"]
    #[inline]
    pub fn line_low(&mut self) -> _LINE_LOWW {
        _LINE_LOWW { w: self }
    }
}
