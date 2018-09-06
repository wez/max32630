#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG1 {
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
#[doc = "Possible values of the field `pwr_clr_io_event_latch`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_CLR_IO_EVENT_LATCHR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_CLR_IO_EVENT_LATCHR {
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
            PWR_CLR_IO_EVENT_LATCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_CLR_IO_EVENT_LATCHR {
        match value {
            i => PWR_CLR_IO_EVENT_LATCHR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_clr_io_cfg_latch`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_CLR_IO_CFG_LATCHR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_CLR_IO_CFG_LATCHR {
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
            PWR_CLR_IO_CFG_LATCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_CLR_IO_CFG_LATCHR {
        match value {
            i => PWR_CLR_IO_CFG_LATCHR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_mbus_gate`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_MBUS_GATER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_MBUS_GATER {
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
            PWR_MBUS_GATER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_MBUS_GATER {
        match value {
            i => PWR_MBUS_GATER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_discharge_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_DISCHARGE_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_DISCHARGE_ENR {
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
            PWR_DISCHARGE_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_DISCHARGE_ENR {
        match value {
            i => PWR_DISCHARGE_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_tvdd12_well`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TVDD12_WELLR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_TVDD12_WELLR {
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
            PWR_TVDD12_WELLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_TVDD12_WELLR {
        match value {
            i => PWR_TVDD12_WELLR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `pwr_clr_io_event_latch`"]
pub enum PWR_CLR_IO_EVENT_LATCHW {}
impl PWR_CLR_IO_EVENT_LATCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_CLR_IO_EVENT_LATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_CLR_IO_EVENT_LATCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_CLR_IO_EVENT_LATCHW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_clr_io_cfg_latch`"]
pub enum PWR_CLR_IO_CFG_LATCHW {}
impl PWR_CLR_IO_CFG_LATCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_CLR_IO_CFG_LATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_CLR_IO_CFG_LATCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_CLR_IO_CFG_LATCHW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_mbus_gate`"]
pub enum PWR_MBUS_GATEW {}
impl PWR_MBUS_GATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_MBUS_GATEW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_MBUS_GATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_MBUS_GATEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_discharge_en`"]
pub enum PWR_DISCHARGE_ENW {}
impl PWR_DISCHARGE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_DISCHARGE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_DISCHARGE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_DISCHARGE_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_tvdd12_well`"]
pub enum PWR_TVDD12_WELLW {}
impl PWR_TVDD12_WELLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TVDD12_WELLW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TVDD12_WELLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TVDD12_WELLW) -> &'a mut W {
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
    #[doc = "Bit 0 - Clear all GPIO Event Seen Latches"]
    #[inline]
    pub fn pwr_clr_io_event_latch(&self) -> PWR_CLR_IO_EVENT_LATCHR {
        PWR_CLR_IO_EVENT_LATCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Clear all GPIO Configuration Latches"]
    #[inline]
    pub fn pwr_clr_io_cfg_latch(&self) -> PWR_CLR_IO_CFG_LATCHR {
        PWR_CLR_IO_CFG_LATCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Freeze GPIO MBus State"]
    #[inline]
    pub fn pwr_mbus_gate(&self) -> PWR_MBUS_GATER {
        PWR_MBUS_GATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable Flash Discharge During Powerfail Event"]
    #[inline]
    pub fn pwr_discharge_en(&self) -> PWR_DISCHARGE_ENR {
        PWR_DISCHARGE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - TVDD12 Well Switch"]
    #[inline]
    pub fn pwr_tvdd12_well(&self) -> PWR_TVDD12_WELLR {
        PWR_TVDD12_WELLR::_from({
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
    #[doc = "Bit 0 - Clear all GPIO Event Seen Latches"]
    #[inline]
    pub fn pwr_clr_io_event_latch(&mut self) -> _PWR_CLR_IO_EVENT_LATCHW {
        _PWR_CLR_IO_EVENT_LATCHW { w: self }
    }
    #[doc = "Bit 1 - Clear all GPIO Configuration Latches"]
    #[inline]
    pub fn pwr_clr_io_cfg_latch(&mut self) -> _PWR_CLR_IO_CFG_LATCHW {
        _PWR_CLR_IO_CFG_LATCHW { w: self }
    }
    #[doc = "Bit 2 - Freeze GPIO MBus State"]
    #[inline]
    pub fn pwr_mbus_gate(&mut self) -> _PWR_MBUS_GATEW {
        _PWR_MBUS_GATEW { w: self }
    }
    #[doc = "Bit 3 - Enable Flash Discharge During Powerfail Event"]
    #[inline]
    pub fn pwr_discharge_en(&mut self) -> _PWR_DISCHARGE_ENW {
        _PWR_DISCHARGE_ENW { w: self }
    }
    #[doc = "Bit 4 - TVDD12 Well Switch"]
    #[inline]
    pub fn pwr_tvdd12_well(&mut self) -> _PWR_TVDD12_WELLW {
        _PWR_TVDD12_WELLW { w: self }
    }
}
