#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ENABLER {
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
            ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            i => ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ll_stopped`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LL_STOPPEDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl LL_STOPPEDR {
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
            LL_STOPPEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LL_STOPPEDR {
        match value {
            i => LL_STOPPEDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `manual`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MANUALR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MANUALR {
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
            MANUALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MANUALR {
        match value {
            i => MANUALR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `bus_error`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUS_ERRORR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUS_ERRORR {
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
            BUS_ERRORR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUS_ERRORR {
        match value {
            i => BUS_ERRORR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `to_stat`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TO_STATR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TO_STATR {
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
            TO_STATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TO_STATR {
        match value {
            i => TO_STATR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `to_sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TO_SELR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TO_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TO_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TO_SELR {
        match value {
            i => TO_SELR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ps_sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_SELR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PS_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PS_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PS_SELR {
        match value {
            i => PS_SELR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `interrupt`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERRUPTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl INTERRUPTR {
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
            INTERRUPTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTERRUPTR {
        match value {
            i => INTERRUPTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `int_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl INT_ENR {
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
            INT_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INT_ENR {
        match value {
            i => INT_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `burst_size`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURST_SIZER {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BURST_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BURST_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BURST_SIZER {
        match value {
            i => BURST_SIZER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `enable`"]
pub enum ENABLEW {}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ll_stopped`"]
pub enum LL_STOPPEDW {}
impl LL_STOPPEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _LL_STOPPEDW<'a> {
    w: &'a mut W,
}
impl<'a> _LL_STOPPEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LL_STOPPEDW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `manual`"]
pub enum MANUALW {}
impl MANUALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MANUALW<'a> {
    w: &'a mut W,
}
impl<'a> _MANUALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MANUALW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `bus_error`"]
pub enum BUS_ERRORW {}
impl BUS_ERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUS_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _BUS_ERRORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUS_ERRORW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `to_stat`"]
pub enum TO_STATW {}
impl TO_STATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TO_STATW<'a> {
    w: &'a mut W,
}
impl<'a> _TO_STATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TO_STATW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `to_sel`"]
pub enum TO_SELW {}
impl TO_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TO_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _TO_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TO_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ps_sel`"]
pub enum PS_SELW {}
impl PS_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PS_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _PS_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PS_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `interrupt`"]
pub enum INTERRUPTW {}
impl INTERRUPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _INTERRUPTW<'a> {
    w: &'a mut W,
}
impl<'a> _INTERRUPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTERRUPTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `int_en`"]
pub enum INT_ENW {}
impl INT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `burst_size`"]
pub enum BURST_SIZEW {}
impl BURST_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BURST_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _BURST_SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURST_SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - PMU Channel Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Linked List Engine Status"]
    #[inline]
    pub fn ll_stopped(&self) -> LL_STOPPEDR {
        LL_STOPPEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Manual Mode Enable"]
    #[inline]
    pub fn manual(&self) -> MANUALR {
        MANUALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - AHB Bus Error Interrupt Flag"]
    #[inline]
    pub fn bus_error(&self) -> BUS_ERRORR {
        BUS_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - AHB Bus Timeout Interrupt Flag"]
    #[inline]
    pub fn to_stat(&self) -> TO_STATR {
        TO_STATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 11:13 - Time Out Interval Select"]
    #[inline]
    pub fn to_sel(&self) -> TO_SELR {
        TO_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Time Out Interval Prescale Select"]
    #[inline]
    pub fn ps_sel(&self) -> PS_SELR {
        PS_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Descriptor Interrupt Flag"]
    #[inline]
    pub fn interrupt(&self) -> INTERRUPTR {
        INTERRUPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - PMU Channel Interrupt Enable"]
    #[inline]
    pub fn int_en(&self) -> INT_ENR {
        INT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:28 - DMA Maximum Burst Size"]
    #[inline]
    pub fn burst_size(&self) -> BURST_SIZER {
        BURST_SIZER::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - PMU Channel Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 2 - Linked List Engine Status"]
    #[inline]
    pub fn ll_stopped(&mut self) -> _LL_STOPPEDW {
        _LL_STOPPEDW { w: self }
    }
    #[doc = "Bit 3 - Manual Mode Enable"]
    #[inline]
    pub fn manual(&mut self) -> _MANUALW {
        _MANUALW { w: self }
    }
    #[doc = "Bit 4 - AHB Bus Error Interrupt Flag"]
    #[inline]
    pub fn bus_error(&mut self) -> _BUS_ERRORW {
        _BUS_ERRORW { w: self }
    }
    #[doc = "Bit 6 - AHB Bus Timeout Interrupt Flag"]
    #[inline]
    pub fn to_stat(&mut self) -> _TO_STATW {
        _TO_STATW { w: self }
    }
    #[doc = "Bits 11:13 - Time Out Interval Select"]
    #[inline]
    pub fn to_sel(&mut self) -> _TO_SELW {
        _TO_SELW { w: self }
    }
    #[doc = "Bits 14:15 - Time Out Interval Prescale Select"]
    #[inline]
    pub fn ps_sel(&mut self) -> _PS_SELW {
        _PS_SELW { w: self }
    }
    #[doc = "Bit 16 - Descriptor Interrupt Flag"]
    #[inline]
    pub fn interrupt(&mut self) -> _INTERRUPTW {
        _INTERRUPTW { w: self }
    }
    #[doc = "Bit 17 - PMU Channel Interrupt Enable"]
    #[inline]
    pub fn int_en(&mut self) -> _INT_ENW {
        _INT_ENW { w: self }
    }
    #[doc = "Bits 24:28 - DMA Maximum Burst Size"]
    #[inline]
    pub fn burst_size(&mut self) -> _BURST_SIZEW {
        _BURST_SIZEW { w: self }
    }
}
