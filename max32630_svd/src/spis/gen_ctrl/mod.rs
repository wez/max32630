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
#[doc = "Possible values of the field `spi_slave_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI_SLAVE_ENR {
    #[doc = "Disable SPI Slave"]
    DISABLED,
    #[doc = "Enable SPI Slave"]
    ENABLED,
}
impl SPI_SLAVE_ENR {
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
            SPI_SLAVE_ENR::DISABLED => false,
            SPI_SLAVE_ENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPI_SLAVE_ENR {
        match value {
            false => SPI_SLAVE_ENR::DISABLED,
            true => SPI_SLAVE_ENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SPI_SLAVE_ENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SPI_SLAVE_ENR::ENABLED
    }
}
#[doc = "Possible values of the field `tx_fifo_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_ENR {
    #[doc = "Disable SPI Slave TX FIFO"]
    DISABLED,
    #[doc = "Enable SPI Slave TX FIFO"]
    ENABLED,
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
            TX_FIFO_ENR::DISABLED => false,
            TX_FIFO_ENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_FIFO_ENR {
        match value {
            false => TX_FIFO_ENR::DISABLED,
            true => TX_FIFO_ENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == TX_FIFO_ENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == TX_FIFO_ENR::ENABLED
    }
}
#[doc = "Possible values of the field `rx_fifo_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_ENR {
    #[doc = "Disable SPI Slave RX FIFO"]
    DISABLED,
    #[doc = "Enable SPI Slave RX FIFO"]
    ENABLED,
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
            RX_FIFO_ENR::DISABLED => false,
            RX_FIFO_ENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_FIFO_ENR {
        match value {
            false => RX_FIFO_ENR::DISABLED,
            true => RX_FIFO_ENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == RX_FIFO_ENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == RX_FIFO_ENR::ENABLED
    }
}
#[doc = "Possible values of the field `data_width`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_WIDTHR {
    #[doc = "1-bit Wide"]
    X1,
    #[doc = "2-bit Wide/Dual"]
    X2,
    #[doc = "4-bit Wide/Quad"]
    X4,
    #[doc = "Reserved for future use. Do not use."]
    INVALID,
}
impl DATA_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATA_WIDTHR::X1 => 0,
            DATA_WIDTHR::X2 => 1,
            DATA_WIDTHR::X4 => 2,
            DATA_WIDTHR::INVALID => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATA_WIDTHR {
        match value {
            0 => DATA_WIDTHR::X1,
            1 => DATA_WIDTHR::X2,
            2 => DATA_WIDTHR::X4,
            3 => DATA_WIDTHR::INVALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline]
    pub fn is_x1(&self) -> bool {
        *self == DATA_WIDTHR::X1
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline]
    pub fn is_x2(&self) -> bool {
        *self == DATA_WIDTHR::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline]
    pub fn is_x4(&self) -> bool {
        *self == DATA_WIDTHR::X4
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline]
    pub fn is_invalid(&self) -> bool {
        *self == DATA_WIDTHR::INVALID
    }
}
#[doc = "Possible values of the field `spi_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI_MODER {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPI_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPI_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPI_MODER {
        match value {
            i => SPI_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tx_clk_invert`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CLK_INVERTR {
    #[doc = "No Effect"]
    NO_EFFECT,
    #[doc = "Inverts the TX transmit clock such that outgoing data is updated on the opposite clock edge from that specified by spi_mode. Effectively, this inverts the value of the Clock Polarity bit from the value specified in spi_mode."]
    INVERT,
}
impl TX_CLK_INVERTR {
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
            TX_CLK_INVERTR::NO_EFFECT => false,
            TX_CLK_INVERTR::INVERT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_CLK_INVERTR {
        match value {
            false => TX_CLK_INVERTR::NO_EFFECT,
            true => TX_CLK_INVERTR::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == TX_CLK_INVERTR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline]
    pub fn is_invert(&self) -> bool {
        *self == TX_CLK_INVERTR::INVERT
    }
}
#[doc = "Possible values of the field `disable_parking`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_PARKINGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DISABLE_PARKINGR {
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
            DISABLE_PARKINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISABLE_PARKINGR {
        match value {
            i => DISABLE_PARKINGR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `spi_slave_en`"]
pub enum SPI_SLAVE_ENW {
    #[doc = "Disable SPI Slave"]
    DISABLED,
    #[doc = "Enable SPI Slave"]
    ENABLED,
}
impl SPI_SLAVE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPI_SLAVE_ENW::DISABLED => false,
            SPI_SLAVE_ENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI_SLAVE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI_SLAVE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI_SLAVE_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable SPI Slave"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI_SLAVE_ENW::DISABLED)
    }
    #[doc = "Enable SPI Slave"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI_SLAVE_ENW::ENABLED)
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
pub enum TX_FIFO_ENW {
    #[doc = "Disable SPI Slave TX FIFO"]
    DISABLED,
    #[doc = "Enable SPI Slave TX FIFO"]
    ENABLED,
}
impl TX_FIFO_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_FIFO_ENW::DISABLED => false,
            TX_FIFO_ENW::ENABLED => true,
        }
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
    #[doc = "Disable SPI Slave TX FIFO"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TX_FIFO_ENW::DISABLED)
    }
    #[doc = "Enable SPI Slave TX FIFO"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TX_FIFO_ENW::ENABLED)
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
pub enum RX_FIFO_ENW {
    #[doc = "Disable SPI Slave RX FIFO"]
    DISABLED,
    #[doc = "Enable SPI Slave RX FIFO"]
    ENABLED,
}
impl RX_FIFO_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RX_FIFO_ENW::DISABLED => false,
            RX_FIFO_ENW::ENABLED => true,
        }
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
    #[doc = "Disable SPI Slave RX FIFO"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RX_FIFO_ENW::DISABLED)
    }
    #[doc = "Enable SPI Slave RX FIFO"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RX_FIFO_ENW::ENABLED)
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
#[doc = "Values that can be written to the field `data_width`"]
pub enum DATA_WIDTHW {
    #[doc = "1-bit Wide"]
    X1,
    #[doc = "2-bit Wide/Dual"]
    X2,
    #[doc = "4-bit Wide/Quad"]
    X4,
    #[doc = "Reserved for future use. Do not use."]
    INVALID,
}
impl DATA_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATA_WIDTHW::X1 => 0,
            DATA_WIDTHW::X2 => 1,
            DATA_WIDTHW::X4 => 2,
            DATA_WIDTHW::INVALID => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_WIDTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1-bit Wide"]
    #[inline]
    pub fn x1(self) -> &'a mut W {
        self.variant(DATA_WIDTHW::X1)
    }
    #[doc = "2-bit Wide/Dual"]
    #[inline]
    pub fn x2(self) -> &'a mut W {
        self.variant(DATA_WIDTHW::X2)
    }
    #[doc = "4-bit Wide/Quad"]
    #[inline]
    pub fn x4(self) -> &'a mut W {
        self.variant(DATA_WIDTHW::X4)
    }
    #[doc = "Reserved for future use. Do not use."]
    #[inline]
    pub fn invalid(self) -> &'a mut W {
        self.variant(DATA_WIDTHW::INVALID)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `spi_mode`"]
pub enum SPI_MODEW {}
impl SPI_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SPI_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `tx_clk_invert`"]
pub enum TX_CLK_INVERTW {
    #[doc = "No Effect"]
    NO_EFFECT,
    #[doc = "Inverts the TX transmit clock such that outgoing data is updated on the opposite clock edge from that specified by spi_mode. Effectively, this inverts the value of the Clock Polarity bit from the value specified in spi_mode."]
    INVERT,
}
impl TX_CLK_INVERTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TX_CLK_INVERTW::NO_EFFECT => false,
            TX_CLK_INVERTW::INVERT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_CLK_INVERTW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_CLK_INVERTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_CLK_INVERTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Effect"]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TX_CLK_INVERTW::NO_EFFECT)
    }
    #[doc = "Inverts the TX transmit clock such that outgoing data is updated on the opposite clock edge from that specified by spi_mode. Effectively, this inverts the value of the Clock Polarity bit from the value specified in spi_mode."]
    #[inline]
    pub fn invert(self) -> &'a mut W {
        self.variant(TX_CLK_INVERTW::INVERT)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `disable_parking`"]
pub enum DISABLE_PARKINGW {}
impl DISABLE_PARKINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DISABLE_PARKINGW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLE_PARKINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLE_PARKINGW) -> &'a mut W {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - SPI Slave Enable"]
    #[inline]
    pub fn spi_slave_en(&self) -> SPI_SLAVE_ENR {
        SPI_SLAVE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - TX FIFO Enable"]
    #[inline]
    pub fn tx_fifo_en(&self) -> TX_FIFO_ENR {
        TX_FIFO_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SPI RX FIFO Enable"]
    #[inline]
    pub fn rx_fifo_en(&self) -> RX_FIFO_ENR {
        RX_FIFO_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Width of SPI Slave Data Transfers"]
    #[inline]
    pub fn data_width(&self) -> DATA_WIDTHR {
        DATA_WIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Defines Clock Polarity (bit 17) and Clock Phase (bit 16), collectively referred to as SPI Mode."]
    #[inline]
    pub fn spi_mode(&self) -> SPI_MODER {
        SPI_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Invert TX Clock"]
    #[inline]
    pub fn tx_clk_invert(&self) -> TX_CLK_INVERTR {
        TX_CLK_INVERTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Disable automatic resetting of SPI Slave on exit from LP Modes"]
    #[inline]
    pub fn disable_parking(&self) -> DISABLE_PARKINGR {
        DISABLE_PARKINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - SPI Slave Enable"]
    #[inline]
    pub fn spi_slave_en(&mut self) -> _SPI_SLAVE_ENW {
        _SPI_SLAVE_ENW { w: self }
    }
    #[doc = "Bit 1 - TX FIFO Enable"]
    #[inline]
    pub fn tx_fifo_en(&mut self) -> _TX_FIFO_ENW {
        _TX_FIFO_ENW { w: self }
    }
    #[doc = "Bit 2 - SPI RX FIFO Enable"]
    #[inline]
    pub fn rx_fifo_en(&mut self) -> _RX_FIFO_ENW {
        _RX_FIFO_ENW { w: self }
    }
    #[doc = "Bits 4:5 - Width of SPI Slave Data Transfers"]
    #[inline]
    pub fn data_width(&mut self) -> _DATA_WIDTHW {
        _DATA_WIDTHW { w: self }
    }
    #[doc = "Bits 16:17 - Defines Clock Polarity (bit 17) and Clock Phase (bit 16), collectively referred to as SPI Mode."]
    #[inline]
    pub fn spi_mode(&mut self) -> _SPI_MODEW {
        _SPI_MODEW { w: self }
    }
    #[doc = "Bit 20 - Invert TX Clock"]
    #[inline]
    pub fn tx_clk_invert(&mut self) -> _TX_CLK_INVERTW {
        _TX_CLK_INVERTW { w: self }
    }
    #[doc = "Bit 31 - Disable automatic resetting of SPI Slave on exit from LP Modes"]
    #[inline]
    pub fn disable_parking(&mut self) -> _DISABLE_PARKINGW {
        _DISABLE_PARKINGW { w: self }
    }
}
