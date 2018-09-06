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
#[doc = "Possible values of the field `tx_nacked`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_NACKEDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_NACKEDR {
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
            TX_NACKEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_NACKEDR {
        match value {
            i => TX_NACKEDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tx_lost_arbitr`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_LOST_ARBITRR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_LOST_ARBITRR {
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
            TX_LOST_ARBITRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_LOST_ARBITRR {
        match value {
            i => TX_LOST_ARBITRR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tx_timeout`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_TIMEOUTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_TIMEOUTR {
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
            TX_TIMEOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_TIMEOUTR {
        match value {
            i => TX_TIMEOUTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tx_fifo_empty`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_EMPTYR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_FIFO_EMPTYR {
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
            TX_FIFO_EMPTYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_FIFO_EMPTYR {
        match value {
            i => TX_FIFO_EMPTYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tx_fifo_3q_empty`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_3Q_EMPTYR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_FIFO_3Q_EMPTYR {
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
            TX_FIFO_3Q_EMPTYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_FIFO_3Q_EMPTYR {
        match value {
            i => TX_FIFO_3Q_EMPTYR::_Reserved(i),
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
#[doc = "Possible values of the field `rx_fifo_2q_full`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_2Q_FULLR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_FIFO_2Q_FULLR {
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
            RX_FIFO_2Q_FULLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FIFO_2Q_FULLR {
        match value {
            i => RX_FIFO_2Q_FULLR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_fifo_3q_full`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_3Q_FULLR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_FIFO_3Q_FULLR {
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
            RX_FIFO_3Q_FULLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FIFO_3Q_FULLR {
        match value {
            i => RX_FIFO_3Q_FULLR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_fifo_full`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_FULLR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_FIFO_FULLR {
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
            RX_FIFO_FULLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FIFO_FULLR {
        match value {
            i => RX_FIFO_FULLR::_Reserved(i),
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
#[doc = "Values that can be written to the field `tx_nacked`"]
pub enum TX_NACKEDW {}
impl TX_NACKEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_NACKEDW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_NACKEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_NACKEDW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `tx_lost_arbitr`"]
pub enum TX_LOST_ARBITRW {}
impl TX_LOST_ARBITRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_LOST_ARBITRW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_LOST_ARBITRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_LOST_ARBITRW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `tx_timeout`"]
pub enum TX_TIMEOUTW {}
impl TX_TIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_TIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_TIMEOUTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `tx_fifo_empty`"]
pub enum TX_FIFO_EMPTYW {}
impl TX_FIFO_EMPTYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_FIFO_EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_FIFO_EMPTYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_FIFO_EMPTYW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `tx_fifo_3q_empty`"]
pub enum TX_FIFO_3Q_EMPTYW {}
impl TX_FIFO_3Q_EMPTYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_FIFO_3Q_EMPTYW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_FIFO_3Q_EMPTYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_FIFO_3Q_EMPTYW) -> &'a mut W {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rx_fifo_2q_full`"]
pub enum RX_FIFO_2Q_FULLW {}
impl RX_FIFO_2Q_FULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_FIFO_2Q_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FIFO_2Q_FULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FIFO_2Q_FULLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `rx_fifo_3q_full`"]
pub enum RX_FIFO_3Q_FULLW {}
impl RX_FIFO_3Q_FULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_FIFO_3Q_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FIFO_3Q_FULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FIFO_3Q_FULLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `rx_fifo_full`"]
pub enum RX_FIFO_FULLW {}
impl RX_FIFO_FULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_FIFO_FULLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FIFO_FULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FIFO_FULLW) -> &'a mut W {
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Transaction Done Int Enable"]
    #[inline]
    pub fn tx_done(&self) -> TX_DONER {
        TX_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transaction NACKed Int Enable"]
    #[inline]
    pub fn tx_nacked(&self) -> TX_NACKEDR {
        TX_NACKEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Transaction Lost Arbitration IntEnable"]
    #[inline]
    pub fn tx_lost_arbitr(&self) -> TX_LOST_ARBITRR {
        TX_LOST_ARBITRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transaction Timed Out Int Enable"]
    #[inline]
    pub fn tx_timeout(&self) -> TX_TIMEOUTR {
        TX_TIMEOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Transaction FIFO Empty Int Enable"]
    #[inline]
    pub fn tx_fifo_empty(&self) -> TX_FIFO_EMPTYR {
        TX_FIFO_EMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Transaction FIFO 3Q Empty Int Enable"]
    #[inline]
    pub fn tx_fifo_3q_empty(&self) -> TX_FIFO_3Q_EMPTYR {
        TX_FIFO_3Q_EMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Results FIFO Not Empty Int Enable"]
    #[inline]
    pub fn rx_fifo_not_empty(&self) -> RX_FIFO_NOT_EMPTYR {
        RX_FIFO_NOT_EMPTYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Results FIFO 2Q Full Int Enable"]
    #[inline]
    pub fn rx_fifo_2q_full(&self) -> RX_FIFO_2Q_FULLR {
        RX_FIFO_2Q_FULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Results FIFO 3Q Full Int Enable"]
    #[inline]
    pub fn rx_fifo_3q_full(&self) -> RX_FIFO_3Q_FULLR {
        RX_FIFO_3Q_FULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Results FIFO Full Int Enable"]
    #[inline]
    pub fn rx_fifo_full(&self) -> RX_FIFO_FULLR {
        RX_FIFO_FULLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Transaction Done Int Enable"]
    #[inline]
    pub fn tx_done(&mut self) -> _TX_DONEW {
        _TX_DONEW { w: self }
    }
    #[doc = "Bit 1 - Transaction NACKed Int Enable"]
    #[inline]
    pub fn tx_nacked(&mut self) -> _TX_NACKEDW {
        _TX_NACKEDW { w: self }
    }
    #[doc = "Bit 2 - Transaction Lost Arbitration IntEnable"]
    #[inline]
    pub fn tx_lost_arbitr(&mut self) -> _TX_LOST_ARBITRW {
        _TX_LOST_ARBITRW { w: self }
    }
    #[doc = "Bit 3 - Transaction Timed Out Int Enable"]
    #[inline]
    pub fn tx_timeout(&mut self) -> _TX_TIMEOUTW {
        _TX_TIMEOUTW { w: self }
    }
    #[doc = "Bit 4 - Transaction FIFO Empty Int Enable"]
    #[inline]
    pub fn tx_fifo_empty(&mut self) -> _TX_FIFO_EMPTYW {
        _TX_FIFO_EMPTYW { w: self }
    }
    #[doc = "Bit 5 - Transaction FIFO 3Q Empty Int Enable"]
    #[inline]
    pub fn tx_fifo_3q_empty(&mut self) -> _TX_FIFO_3Q_EMPTYW {
        _TX_FIFO_3Q_EMPTYW { w: self }
    }
    #[doc = "Bit 6 - Results FIFO Not Empty Int Enable"]
    #[inline]
    pub fn rx_fifo_not_empty(&mut self) -> _RX_FIFO_NOT_EMPTYW {
        _RX_FIFO_NOT_EMPTYW { w: self }
    }
    #[doc = "Bit 7 - Results FIFO 2Q Full Int Enable"]
    #[inline]
    pub fn rx_fifo_2q_full(&mut self) -> _RX_FIFO_2Q_FULLW {
        _RX_FIFO_2Q_FULLW { w: self }
    }
    #[doc = "Bit 8 - Results FIFO 3Q Full Int Enable"]
    #[inline]
    pub fn rx_fifo_3q_full(&mut self) -> _RX_FIFO_3Q_FULLW {
        _RX_FIFO_3Q_FULLW { w: self }
    }
    #[doc = "Bit 9 - Results FIFO Full Int Enable"]
    #[inline]
    pub fn rx_fifo_full(&mut self) -> _RX_FIFO_FULLW {
        _RX_FIFO_FULLW { w: self }
    }
}
