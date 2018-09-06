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
#[doc = "Possible values of the field `tx_no_data`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_NO_DATAR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_NO_DATAR {
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
            TX_NO_DATAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_NO_DATAR {
        match value {
            i => TX_NO_DATAR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rx_lost_data`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_LOST_DATAR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RX_LOST_DATAR {
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
            RX_LOST_DATAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RX_LOST_DATAR {
        match value {
            i => RX_LOST_DATAR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ss_asserted`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_ASSERTEDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SS_ASSERTEDR {
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
            SS_ASSERTEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SS_ASSERTEDR {
        match value {
            i => SS_ASSERTEDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ss_deasserted`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_DEASSERTEDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SS_DEASSERTEDR {
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
            SS_DEASSERTEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SS_DEASSERTEDR {
        match value {
            i => SS_DEASSERTEDR::_Reserved(i),
        }
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
        const OFFSET: u8 = 0;
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `tx_no_data`"]
pub enum TX_NO_DATAW {}
impl TX_NO_DATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_NO_DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_NO_DATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_NO_DATAW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `rx_lost_data`"]
pub enum RX_LOST_DATAW {}
impl RX_LOST_DATAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RX_LOST_DATAW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_LOST_DATAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_LOST_DATAW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ss_asserted`"]
pub enum SS_ASSERTEDW {}
impl SS_ASSERTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SS_ASSERTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SS_ASSERTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS_ASSERTEDW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ss_deasserted`"]
pub enum SS_DEASSERTEDW {}
impl SS_DEASSERTEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SS_DEASSERTEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SS_DEASSERTEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS_DEASSERTEDW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - TX FIFO Almost Empty"]
    #[inline]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AER {
        TX_FIFO_AER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RX FIFO Almost Full"]
    #[inline]
    pub fn rx_fifo_af(&self) -> RX_FIFO_AFR {
        RX_FIFO_AFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - TX FIFO Empty"]
    #[inline]
    pub fn tx_no_data(&self) -> TX_NO_DATAR {
        TX_NO_DATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RX FIFO Overflow"]
    #[inline]
    pub fn rx_lost_data(&self) -> RX_LOST_DATAR {
        RX_LOST_DATAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Slave Select Asserted"]
    #[inline]
    pub fn ss_asserted(&self) -> SS_ASSERTEDR {
        SS_ASSERTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Slave Select Deasserted"]
    #[inline]
    pub fn ss_deasserted(&self) -> SS_DEASSERTEDR {
        SS_DEASSERTEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - TX FIFO Almost Empty"]
    #[inline]
    pub fn tx_fifo_ae(&mut self) -> _TX_FIFO_AEW {
        _TX_FIFO_AEW { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Almost Full"]
    #[inline]
    pub fn rx_fifo_af(&mut self) -> _RX_FIFO_AFW {
        _RX_FIFO_AFW { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Empty"]
    #[inline]
    pub fn tx_no_data(&mut self) -> _TX_NO_DATAW {
        _TX_NO_DATAW { w: self }
    }
    #[doc = "Bit 3 - RX FIFO Overflow"]
    #[inline]
    pub fn rx_lost_data(&mut self) -> _RX_LOST_DATAW {
        _RX_LOST_DATAW { w: self }
    }
    #[doc = "Bit 5 - Slave Select Asserted"]
    #[inline]
    pub fn ss_asserted(&mut self) -> _SS_ASSERTEDW {
        _SS_ASSERTEDW { w: self }
    }
    #[doc = "Bit 6 - Slave Select Deasserted"]
    #[inline]
    pub fn ss_deasserted(&mut self) -> _SS_DEASSERTEDW {
        _SS_DEASSERTEDW { w: self }
    }
}
