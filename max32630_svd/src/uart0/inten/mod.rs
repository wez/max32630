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
#[doc = "Possible values of the field `tx_done`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_DONER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_DONER {
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
            TX_DONER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_DONER {
        match value {
            i => TX_DONER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tx_unstalled`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_UNSTALLEDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_UNSTALLEDR {
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
            TX_UNSTALLEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_UNSTALLEDR {
        match value {
            i => TX_UNSTALLEDR::_Reserved(i),
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
#[doc = "Possible values of the field `rx_fifo_not_empty`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_NOT_EMPTYR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_FIFO_NOT_EMPTYR {
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
            RX_FIFO_NOT_EMPTYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FIFO_NOT_EMPTYR {
        match value {
            i => RX_FIFO_NOT_EMPTYR::_Reserved(i),
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
#[doc = "Possible values of the field `rx_fifo_overflow`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_OVERFLOWR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_FIFO_OVERFLOWR {
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
            RX_FIFO_OVERFLOWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FIFO_OVERFLOWR {
        match value {
            i => RX_FIFO_OVERFLOWR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_framing_err`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FRAMING_ERRR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_FRAMING_ERRR {
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
            RX_FRAMING_ERRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FRAMING_ERRR {
        match value {
            i => RX_FRAMING_ERRR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_parity_err`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_PARITY_ERRR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_PARITY_ERRR {
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
            RX_PARITY_ERRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_PARITY_ERRR {
        match value {
            i => RX_PARITY_ERRR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `tx_done`"]
pub enum TX_DONEW {}
impl TX_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_DONEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `tx_unstalled`"]
pub enum TX_UNSTALLEDW {}
impl TX_UNSTALLEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_UNSTALLEDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_UNSTALLEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_UNSTALLEDW) -> &'a mut W {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rx_fifo_not_empty`"]
pub enum RX_FIFO_NOT_EMPTYW {}
impl RX_FIFO_NOT_EMPTYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_FIFO_NOT_EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FIFO_NOT_EMPTYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FIFO_NOT_EMPTYW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `rx_fifo_overflow`"]
pub enum RX_FIFO_OVERFLOWW {}
impl RX_FIFO_OVERFLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_FIFO_OVERFLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FIFO_OVERFLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FIFO_OVERFLOWW) -> &'a mut W {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rx_framing_err`"]
pub enum RX_FRAMING_ERRW {}
impl RX_FRAMING_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_FRAMING_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FRAMING_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FRAMING_ERRW) -> &'a mut W {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rx_parity_err`"]
pub enum RX_PARITY_ERRW {}
impl RX_PARITY_ERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_PARITY_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_PARITY_ERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_PARITY_ERRW) -> &'a mut W {
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - TX Done Interrupt Enable/Disable"]
    #[inline]
    pub fn tx_done(&self) -> TX_DONER {
        TX_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TX Unstalled Interrupt Enable/Disable"]
    #[inline]
    pub fn tx_unstalled(&self) -> TX_UNSTALLEDR {
        TX_UNSTALLEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - TX FIFO Almost Empty Interrupt Enable/Disable"]
    #[inline]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AER {
        TX_FIFO_AER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RX FIFO Not Empty Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_fifo_not_empty(&self) -> RX_FIFO_NOT_EMPTYR {
        RX_FIFO_NOT_EMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - RX Stalled Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_stalled(&self) -> RX_STALLEDR {
        RX_STALLEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - RX FIFO Almost Full Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_fifo_af(&self) -> RX_FIFO_AFR {
        RX_FIFO_AFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - RX FIFO Overflow Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOWR {
        RX_FIFO_OVERFLOWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - RX Framing Error Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_framing_err(&self) -> RX_FRAMING_ERRR {
        RX_FRAMING_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - RX Parity Error Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_parity_err(&self) -> RX_PARITY_ERRR {
        RX_PARITY_ERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - TX Done Interrupt Enable/Disable"]
    #[inline]
    pub fn tx_done(&mut self) -> _TX_DONEW {
        _TX_DONEW { w: self }
    }
    #[doc = "Bit 1 - TX Unstalled Interrupt Enable/Disable"]
    #[inline]
    pub fn tx_unstalled(&mut self) -> _TX_UNSTALLEDW {
        _TX_UNSTALLEDW { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Almost Empty Interrupt Enable/Disable"]
    #[inline]
    pub fn tx_fifo_ae(&mut self) -> _TX_FIFO_AEW {
        _TX_FIFO_AEW { w: self }
    }
    #[doc = "Bit 3 - RX FIFO Not Empty Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_fifo_not_empty(&mut self) -> _RX_FIFO_NOT_EMPTYW {
        _RX_FIFO_NOT_EMPTYW { w: self }
    }
    #[doc = "Bit 4 - RX Stalled Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_stalled(&mut self) -> _RX_STALLEDW {
        _RX_STALLEDW { w: self }
    }
    #[doc = "Bit 5 - RX FIFO Almost Full Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_fifo_af(&mut self) -> _RX_FIFO_AFW {
        _RX_FIFO_AFW { w: self }
    }
    #[doc = "Bit 6 - RX FIFO Overflow Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_fifo_overflow(&mut self) -> _RX_FIFO_OVERFLOWW {
        _RX_FIFO_OVERFLOWW { w: self }
    }
    #[doc = "Bit 7 - RX Framing Error Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_framing_err(&mut self) -> _RX_FRAMING_ERRW {
        _RX_FRAMING_ERRW { w: self }
    }
    #[doc = "Bit 8 - RX Parity Error Interrupt Enable/Disable"]
    #[inline]
    pub fn rx_parity_err(&mut self) -> _RX_PARITY_ERRW {
        _RX_PARITY_ERRW { w: self }
    }
}
