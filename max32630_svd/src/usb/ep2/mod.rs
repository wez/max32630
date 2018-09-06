#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EP2 {
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
#[doc = "Possible values of the field `ep_dir`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_DIRR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EP_DIRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EP_DIRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EP_DIRR {
        match value {
            i => EP_DIRR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ep_buf2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_BUF2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EP_BUF2R {
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
            EP_BUF2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP_BUF2R {
        match value {
            i => EP_BUF2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ep_int_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_INT_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EP_INT_ENR {
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
            EP_INT_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP_INT_ENR {
        match value {
            i => EP_INT_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ep_nak_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_NAK_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EP_NAK_ENR {
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
            EP_NAK_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP_NAK_ENR {
        match value {
            i => EP_NAK_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ep_dt`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_DTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EP_DTR {
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
            EP_DTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP_DTR {
        match value {
            i => EP_DTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ep_stall`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_STALLR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EP_STALLR {
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
            EP_STALLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP_STALLR {
        match value {
            i => EP_STALLR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ep_st_stall`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_ST_STALLR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EP_ST_STALLR {
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
            EP_ST_STALLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP_ST_STALLR {
        match value {
            i => EP_ST_STALLR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ep_st_ack`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_ST_ACKR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EP_ST_ACKR {
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
            EP_ST_ACKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP_ST_ACKR {
        match value {
            i => EP_ST_ACKR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `ep_dir`"]
pub enum EP_DIRW {}
impl EP_DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EP_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP_DIRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ep_buf2`"]
pub enum EP_BUF2W {}
impl EP_BUF2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EP_BUF2W<'a> {
    w: &'a mut W,
}
impl<'a> _EP_BUF2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP_BUF2W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ep_int_en`"]
pub enum EP_INT_ENW {}
impl EP_INT_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EP_INT_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_INT_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP_INT_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ep_nak_en`"]
pub enum EP_NAK_ENW {}
impl EP_NAK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EP_NAK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_NAK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP_NAK_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ep_dt`"]
pub enum EP_DTW {}
impl EP_DTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EP_DTW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_DTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP_DTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ep_stall`"]
pub enum EP_STALLW {}
impl EP_STALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EP_STALLW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_STALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP_STALLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ep_st_stall`"]
pub enum EP_ST_STALLW {}
impl EP_ST_STALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EP_ST_STALLW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_ST_STALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP_ST_STALLW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ep_st_ack`"]
pub enum EP_ST_ACKW {}
impl EP_ST_ACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EP_ST_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _EP_ST_ACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP_ST_ACKW) -> &'a mut W {
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:1 - Endpoint Direction"]
    #[inline]
    pub fn ep_dir(&self) -> EP_DIRR {
        EP_DIRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Endpoint Double Buffered Enable"]
    #[inline]
    pub fn ep_buf2(&self) -> EP_BUF2R {
        EP_BUF2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Endpoint Transfer Complete Interrupt Enable"]
    #[inline]
    pub fn ep_int_en(&self) -> EP_INT_ENR {
        EP_INT_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Endpoint NAK Interrupt Enable"]
    #[inline]
    pub fn ep_nak_en(&self) -> EP_NAK_ENR {
        EP_NAK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Endpoint Data Toggle Clear"]
    #[inline]
    pub fn ep_dt(&self) -> EP_DTR {
        EP_DTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Endpoint Stall"]
    #[inline]
    pub fn ep_stall(&self) -> EP_STALLR {
        EP_STALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Endpoint Stall Status Stage of Control Transfer"]
    #[inline]
    pub fn ep_st_stall(&self) -> EP_ST_STALLR {
        EP_ST_STALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Endpoint Acknowledge Status Stage of Control Transfer"]
    #[inline]
    pub fn ep_st_ack(&self) -> EP_ST_ACKR {
        EP_ST_ACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:1 - Endpoint Direction"]
    #[inline]
    pub fn ep_dir(&mut self) -> _EP_DIRW {
        _EP_DIRW { w: self }
    }
    #[doc = "Bit 3 - Endpoint Double Buffered Enable"]
    #[inline]
    pub fn ep_buf2(&mut self) -> _EP_BUF2W {
        _EP_BUF2W { w: self }
    }
    #[doc = "Bit 4 - Endpoint Transfer Complete Interrupt Enable"]
    #[inline]
    pub fn ep_int_en(&mut self) -> _EP_INT_ENW {
        _EP_INT_ENW { w: self }
    }
    #[doc = "Bit 5 - Endpoint NAK Interrupt Enable"]
    #[inline]
    pub fn ep_nak_en(&mut self) -> _EP_NAK_ENW {
        _EP_NAK_ENW { w: self }
    }
    #[doc = "Bit 6 - Endpoint Data Toggle Clear"]
    #[inline]
    pub fn ep_dt(&mut self) -> _EP_DTW {
        _EP_DTW { w: self }
    }
    #[doc = "Bit 8 - Endpoint Stall"]
    #[inline]
    pub fn ep_stall(&mut self) -> _EP_STALLW {
        _EP_STALLW { w: self }
    }
    #[doc = "Bit 9 - Endpoint Stall Status Stage of Control Transfer"]
    #[inline]
    pub fn ep_st_stall(&mut self) -> _EP_ST_STALLW {
        _EP_ST_STALLW { w: self }
    }
    #[doc = "Bit 10 - Endpoint Acknowledge Status Stage of Control Transfer"]
    #[inline]
    pub fn ep_st_ack(&mut self) -> _EP_ST_ACKW {
        _EP_ST_ACKW { w: self }
    }
}
