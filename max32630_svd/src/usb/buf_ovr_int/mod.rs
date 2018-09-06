#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BUF_OVR_INT {
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
#[doc = "Possible values of the field `buf_ovr0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_OVR0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_OVR0R {
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
            BUF_OVR0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_OVR0R {
        match value {
            i => BUF_OVR0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf_ovr1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_OVR1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_OVR1R {
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
            BUF_OVR1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_OVR1R {
        match value {
            i => BUF_OVR1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf_ovr2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_OVR2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_OVR2R {
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
            BUF_OVR2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_OVR2R {
        match value {
            i => BUF_OVR2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf_ovr3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_OVR3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_OVR3R {
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
            BUF_OVR3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_OVR3R {
        match value {
            i => BUF_OVR3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf_ovr4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_OVR4R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_OVR4R {
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
            BUF_OVR4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_OVR4R {
        match value {
            i => BUF_OVR4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf_ovr5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_OVR5R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_OVR5R {
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
            BUF_OVR5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_OVR5R {
        match value {
            i => BUF_OVR5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf_ovr6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_OVR6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_OVR6R {
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
            BUF_OVR6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_OVR6R {
        match value {
            i => BUF_OVR6R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `buf_ovr7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUF_OVR7R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BUF_OVR7R {
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
            BUF_OVR7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BUF_OVR7R {
        match value {
            i => BUF_OVR7R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `buf_ovr0`"]
pub enum BUF_OVR0W {}
impl BUF_OVR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_OVR0W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_OVR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_OVR0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `buf_ovr1`"]
pub enum BUF_OVR1W {}
impl BUF_OVR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_OVR1W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_OVR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_OVR1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `buf_ovr2`"]
pub enum BUF_OVR2W {}
impl BUF_OVR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_OVR2W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_OVR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_OVR2W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `buf_ovr3`"]
pub enum BUF_OVR3W {}
impl BUF_OVR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_OVR3W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_OVR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_OVR3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `buf_ovr4`"]
pub enum BUF_OVR4W {}
impl BUF_OVR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_OVR4W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_OVR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_OVR4W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `buf_ovr5`"]
pub enum BUF_OVR5W {}
impl BUF_OVR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_OVR5W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_OVR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_OVR5W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `buf_ovr6`"]
pub enum BUF_OVR6W {}
impl BUF_OVR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_OVR6W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_OVR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_OVR6W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `buf_ovr7`"]
pub enum BUF_OVR7W {}
impl BUF_OVR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _BUF_OVR7W<'a> {
    w: &'a mut W,
}
impl<'a> _BUF_OVR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BUF_OVR7W) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Endpoint 0 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr0(&self) -> BUF_OVR0R {
        BUF_OVR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Endpoint 1 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr1(&self) -> BUF_OVR1R {
        BUF_OVR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Endpoint 2 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr2(&self) -> BUF_OVR2R {
        BUF_OVR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Endpoint 3 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr3(&self) -> BUF_OVR3R {
        BUF_OVR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Endpoint 4 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr4(&self) -> BUF_OVR4R {
        BUF_OVR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Endpoint 5 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr5(&self) -> BUF_OVR5R {
        BUF_OVR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Endpoint 6 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr6(&self) -> BUF_OVR6R {
        BUF_OVR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Endpoint 7 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr7(&self) -> BUF_OVR7R {
        BUF_OVR7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 0 - Endpoint 0 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr0(&mut self) -> _BUF_OVR0W {
        _BUF_OVR0W { w: self }
    }
    #[doc = "Bit 1 - Endpoint 1 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr1(&mut self) -> _BUF_OVR1W {
        _BUF_OVR1W { w: self }
    }
    #[doc = "Bit 2 - Endpoint 2 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr2(&mut self) -> _BUF_OVR2W {
        _BUF_OVR2W { w: self }
    }
    #[doc = "Bit 3 - Endpoint 3 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr3(&mut self) -> _BUF_OVR3W {
        _BUF_OVR3W { w: self }
    }
    #[doc = "Bit 4 - Endpoint 4 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr4(&mut self) -> _BUF_OVR4W {
        _BUF_OVR4W { w: self }
    }
    #[doc = "Bit 5 - Endpoint 5 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr5(&mut self) -> _BUF_OVR5W {
        _BUF_OVR5W { w: self }
    }
    #[doc = "Bit 6 - Endpoint 6 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr6(&mut self) -> _BUF_OVR6W {
        _BUF_OVR6W { w: self }
    }
    #[doc = "Bit 7 - Endpoint 7 Buffer Overflow Interrupt Flag"]
    #[inline]
    pub fn buf_ovr7(&mut self) -> _BUF_OVR7W {
        _BUF_OVR7W { w: self }
    }
}
