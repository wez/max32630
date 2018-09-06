#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `uart_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl UART_ENR {
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
            UART_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UART_ENR {
        match value {
            i => UART_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_fifo_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_FIFO_ENR {
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
            RX_FIFO_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FIFO_ENR {
        match value {
            i => RX_FIFO_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tx_fifo_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_FIFO_ENR {
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
            TX_FIFO_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_FIFO_ENR {
        match value {
            i => TX_FIFO_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `data_size`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_SIZER {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATA_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATA_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATA_SIZER {
        match value {
            i => DATA_SIZER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `extra_stop`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRA_STOPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EXTRA_STOPR {
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
            EXTRA_STOPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTRA_STOPR {
        match value {
            i => EXTRA_STOPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `parity`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PARITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PARITYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PARITYR {
        match value {
            i => PARITYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `cts_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CTS_ENR {
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
            CTS_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTS_ENR {
        match value {
            i => CTS_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `cts_polarity`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_POLARITYR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CTS_POLARITYR {
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
            CTS_POLARITYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CTS_POLARITYR {
        match value {
            i => CTS_POLARITYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rts_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTS_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTS_ENR {
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
            RTS_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTS_ENR {
        match value {
            i => RTS_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rts_polarity`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTS_POLARITYR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTS_POLARITYR {
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
            RTS_POLARITYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTS_POLARITYR {
        match value {
            i => RTS_POLARITYR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rts_level`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTS_LEVELR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RTS_LEVELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTS_LEVELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTS_LEVELR {
        match value {
            i => RTS_LEVELR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `uart_en`"]
pub enum UART_ENW {}
impl UART_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UART_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `rx_fifo_en`"]
pub enum RX_FIFO_ENW {}
impl RX_FIFO_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_FIFO_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_FIFO_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_FIFO_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `tx_fifo_en`"]
pub enum TX_FIFO_ENW {}
impl TX_FIFO_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_FIFO_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_FIFO_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_FIFO_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `data_size`"]
pub enum DATA_SIZEW {}
impl DATA_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DATA_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `extra_stop`"]
pub enum EXTRA_STOPW {}
impl EXTRA_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EXTRA_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTRA_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTRA_STOPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `parity`"]
pub enum PARITYW {}
impl PARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARITYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `cts_en`"]
pub enum CTS_ENW {}
impl CTS_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CTS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CTS_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTS_ENW) -> &'a mut W {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `cts_polarity`"]
pub enum CTS_POLARITYW {}
impl CTS_POLARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CTS_POLARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _CTS_POLARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CTS_POLARITYW) -> &'a mut W {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rts_en`"]
pub enum RTS_ENW {}
impl RTS_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RTS_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTS_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTS_ENW) -> &'a mut W {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rts_polarity`"]
pub enum RTS_POLARITYW {}
impl RTS_POLARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RTS_POLARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _RTS_POLARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTS_POLARITYW) -> &'a mut W {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rts_level`"]
pub enum RTS_LEVELW {}
impl RTS_LEVELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RTS_LEVELW<'a> {
    w: &'a mut W,
}
impl<'a> _RTS_LEVELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTS_LEVELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - UART Enable"]
    #[inline]
    pub fn uart_en(&self) -> UART_ENR {
        UART_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RX FIFO Enable"]
    #[inline]
    pub fn rx_fifo_en(&self) -> RX_FIFO_ENR {
        RX_FIFO_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - TX FIFO Enable"]
    #[inline]
    pub fn tx_fifo_en(&self) -> TX_FIFO_ENR {
        TX_FIFO_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Data Size"]
    #[inline]
    pub fn data_size(&self) -> DATA_SIZER {
        DATA_SIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Extra Stop Enable"]
    #[inline]
    pub fn extra_stop(&self) -> EXTRA_STOPR {
        EXTRA_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - Parity Mode"]
    #[inline]
    pub fn parity(&self) -> PARITYR {
        PARITYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - CTS Enable"]
    #[inline]
    pub fn cts_en(&self) -> CTS_ENR {
        CTS_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - CTS Polarity"]
    #[inline]
    pub fn cts_polarity(&self) -> CTS_POLARITYR {
        CTS_POLARITYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - RTS Enable"]
    #[inline]
    pub fn rts_en(&self) -> RTS_ENR {
        RTS_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - RTS Polarity"]
    #[inline]
    pub fn rts_polarity(&self) -> RTS_POLARITYR {
        RTS_POLARITYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 20:25 - RX FIFO LTE Level for RTS Assert"]
    #[inline]
    pub fn rts_level(&self) -> RTS_LEVELR {
        RTS_LEVELR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - UART Enable"]
    #[inline]
    pub fn uart_en(&mut self) -> _UART_ENW {
        _UART_ENW { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Enable"]
    #[inline]
    pub fn rx_fifo_en(&mut self) -> _RX_FIFO_ENW {
        _RX_FIFO_ENW { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Enable"]
    #[inline]
    pub fn tx_fifo_en(&mut self) -> _TX_FIFO_ENW {
        _TX_FIFO_ENW { w: self }
    }
    #[doc = "Bits 4:5 - Data Size"]
    #[inline]
    pub fn data_size(&mut self) -> _DATA_SIZEW {
        _DATA_SIZEW { w: self }
    }
    #[doc = "Bit 8 - Extra Stop Enable"]
    #[inline]
    pub fn extra_stop(&mut self) -> _EXTRA_STOPW {
        _EXTRA_STOPW { w: self }
    }
    #[doc = "Bits 12:13 - Parity Mode"]
    #[inline]
    pub fn parity(&mut self) -> _PARITYW {
        _PARITYW { w: self }
    }
    #[doc = "Bit 16 - CTS Enable"]
    #[inline]
    pub fn cts_en(&mut self) -> _CTS_ENW {
        _CTS_ENW { w: self }
    }
    #[doc = "Bit 17 - CTS Polarity"]
    #[inline]
    pub fn cts_polarity(&mut self) -> _CTS_POLARITYW {
        _CTS_POLARITYW { w: self }
    }
    #[doc = "Bit 18 - RTS Enable"]
    #[inline]
    pub fn rts_en(&mut self) -> _RTS_ENW {
        _RTS_ENW { w: self }
    }
    #[doc = "Bit 19 - RTS Polarity"]
    #[inline]
    pub fn rts_polarity(&mut self) -> _RTS_POLARITYW {
        _RTS_POLARITYW { w: self }
    }
    #[doc = "Bits 20:25 - RX FIFO LTE Level for RTS Assert"]
    #[inline]
    pub fn rts_level(&mut self) -> _RTS_LEVELW {
        _RTS_LEVELW { w: self }
    }
}
