#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GEN_CTRL {
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
#[doc = "Possible values of the field `spi_mstr_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI_MSTR_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SPI_MSTR_ENR {
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
            SPI_MSTR_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI_MSTR_ENR {
        match value {
            i => SPI_MSTR_ENR::_Reserved(i),
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
#[doc = "Possible values of the field `bit_bang_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT_BANG_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BIT_BANG_MODER {
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
            BIT_BANG_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIT_BANG_MODER {
        match value {
            i => BIT_BANG_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bb_ss_in_out`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SS_IN_OUTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BB_SS_IN_OUTR {
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
            BB_SS_IN_OUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_SS_IN_OUTR {
        match value {
            i => BB_SS_IN_OUTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bb_sr_in`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SR_INR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BB_SR_INR {
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
            BB_SR_INR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_SR_INR {
        match value {
            i => BB_SR_INR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bb_sck_in_out`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SCK_IN_OUTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BB_SCK_IN_OUTR {
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
            BB_SCK_IN_OUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BB_SCK_IN_OUTR {
        match value {
            i => BB_SCK_IN_OUTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bb_sdio_in`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SDIO_INR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BB_SDIO_INR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BB_SDIO_INR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BB_SDIO_INR {
        match value {
            i => BB_SDIO_INR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bb_sdio_out`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SDIO_OUTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BB_SDIO_OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BB_SDIO_OUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BB_SDIO_OUTR {
        match value {
            i => BB_SDIO_OUTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bb_sdio_dr_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SDIO_DR_ENR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BB_SDIO_DR_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BB_SDIO_DR_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BB_SDIO_DR_ENR {
        match value {
            i => BB_SDIO_DR_ENR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `spi_mstr_en`"]
pub enum SPI_MSTR_ENW {}
impl SPI_MSTR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SPI_MSTR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI_MSTR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI_MSTR_ENW) -> &'a mut W {
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
        const OFFSET: u8 = 1;
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `bit_bang_mode`"]
pub enum BIT_BANG_MODEW {}
impl BIT_BANG_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BIT_BANG_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BIT_BANG_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIT_BANG_MODEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `bb_ss_in_out`"]
pub enum BB_SS_IN_OUTW {}
impl BB_SS_IN_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BB_SS_IN_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_SS_IN_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_SS_IN_OUTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `bb_sck_in_out`"]
pub enum BB_SCK_IN_OUTW {}
impl BB_SCK_IN_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BB_SCK_IN_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_SCK_IN_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_SCK_IN_OUTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `bb_sdio_out`"]
pub enum BB_SDIO_OUTW {}
impl BB_SDIO_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BB_SDIO_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_SDIO_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_SDIO_OUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `bb_sdio_dr_en`"]
pub enum BB_SDIO_DR_ENW {}
impl BB_SDIO_DR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BB_SDIO_DR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _BB_SDIO_DR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BB_SDIO_DR_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bit 0 - Enable/Disable SPI Master"]
    #[inline]
    pub fn spi_mstr_en(&self) -> SPI_MSTR_ENR {
        SPI_MSTR_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Transaction FIFO Enable"]
    #[inline]
    pub fn tx_fifo_en(&self) -> TX_FIFO_ENR {
        TX_FIFO_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Results FIFO Enable"]
    #[inline]
    pub fn rx_fifo_en(&self) -> RX_FIFO_ENR {
        RX_FIFO_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Bit Bang Mode Enable"]
    #[inline]
    pub fn bit_bang_mode(&self) -> BIT_BANG_MODER {
        BIT_BANG_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Bit Bang SS Input/Output"]
    #[inline]
    pub fn bb_ss_in_out(&self) -> BB_SS_IN_OUTR {
        BB_SS_IN_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Bit Bang SR Input"]
    #[inline]
    pub fn bb_sr_in(&self) -> BB_SR_INR {
        BB_SR_INR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Bit Bang SCK Input/Output"]
    #[inline]
    pub fn bb_sck_in_out(&self) -> BB_SCK_IN_OUTR {
        BB_SCK_IN_OUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Bit Bang SDIO Input"]
    #[inline]
    pub fn bb_sdio_in(&self) -> BB_SDIO_INR {
        BB_SDIO_INR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Bit Bang SDIO Output"]
    #[inline]
    pub fn bb_sdio_out(&self) -> BB_SDIO_OUTR {
        BB_SDIO_OUTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Bit Bang SDIO Drive Enable"]
    #[inline]
    pub fn bb_sdio_dr_en(&self) -> BB_SDIO_DR_ENR {
        BB_SDIO_DR_ENR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bit 0 - Enable/Disable SPI Master"]
    #[inline]
    pub fn spi_mstr_en(&mut self) -> _SPI_MSTR_ENW {
        _SPI_MSTR_ENW { w: self }
    }
    #[doc = "Bit 1 - Transaction FIFO Enable"]
    #[inline]
    pub fn tx_fifo_en(&mut self) -> _TX_FIFO_ENW {
        _TX_FIFO_ENW { w: self }
    }
    #[doc = "Bit 2 - Results FIFO Enable"]
    #[inline]
    pub fn rx_fifo_en(&mut self) -> _RX_FIFO_ENW {
        _RX_FIFO_ENW { w: self }
    }
    #[doc = "Bit 3 - Bit Bang Mode Enable"]
    #[inline]
    pub fn bit_bang_mode(&mut self) -> _BIT_BANG_MODEW {
        _BIT_BANG_MODEW { w: self }
    }
    #[doc = "Bit 4 - Bit Bang SS Input/Output"]
    #[inline]
    pub fn bb_ss_in_out(&mut self) -> _BB_SS_IN_OUTW {
        _BB_SS_IN_OUTW { w: self }
    }
    #[doc = "Bit 6 - Bit Bang SCK Input/Output"]
    #[inline]
    pub fn bb_sck_in_out(&mut self) -> _BB_SCK_IN_OUTW {
        _BB_SCK_IN_OUTW { w: self }
    }
    #[doc = "Bits 12:15 - Bit Bang SDIO Output"]
    #[inline]
    pub fn bb_sdio_out(&mut self) -> _BB_SDIO_OUTW {
        _BB_SDIO_OUTW { w: self }
    }
    #[doc = "Bits 16:19 - Bit Bang SDIO Drive Enable"]
    #[inline]
    pub fn bb_sdio_dr_en(&mut self) -> _BB_SDIO_DR_ENW {
        _BB_SDIO_DR_ENW { w: self }
    }
}
