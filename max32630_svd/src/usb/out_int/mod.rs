#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUT_INT {
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
#[doc = "Possible values of the field `outdav0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDAV0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OUTDAV0R {
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
            OUTDAV0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTDAV0R {
        match value {
            i => OUTDAV0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `outdav1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDAV1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OUTDAV1R {
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
            OUTDAV1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTDAV1R {
        match value {
            i => OUTDAV1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `outdav2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDAV2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OUTDAV2R {
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
            OUTDAV2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTDAV2R {
        match value {
            i => OUTDAV2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `outdav3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDAV3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OUTDAV3R {
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
            OUTDAV3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTDAV3R {
        match value {
            i => OUTDAV3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `outdav4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDAV4R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OUTDAV4R {
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
            OUTDAV4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTDAV4R {
        match value {
            i => OUTDAV4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `outdav5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDAV5R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OUTDAV5R {
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
            OUTDAV5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTDAV5R {
        match value {
            i => OUTDAV5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `outdav6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDAV6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OUTDAV6R {
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
            OUTDAV6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTDAV6R {
        match value {
            i => OUTDAV6R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `outdav7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTDAV7R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OUTDAV7R {
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
            OUTDAV7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OUTDAV7R {
        match value {
            i => OUTDAV7R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `outdav0`"]
pub enum OUTDAV0W {}
impl OUTDAV0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OUTDAV0W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTDAV0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTDAV0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `outdav1`"]
pub enum OUTDAV1W {}
impl OUTDAV1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OUTDAV1W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTDAV1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTDAV1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `outdav2`"]
pub enum OUTDAV2W {}
impl OUTDAV2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OUTDAV2W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTDAV2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTDAV2W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `outdav3`"]
pub enum OUTDAV3W {}
impl OUTDAV3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OUTDAV3W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTDAV3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTDAV3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `outdav4`"]
pub enum OUTDAV4W {}
impl OUTDAV4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OUTDAV4W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTDAV4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTDAV4W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `outdav5`"]
pub enum OUTDAV5W {}
impl OUTDAV5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OUTDAV5W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTDAV5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTDAV5W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `outdav6`"]
pub enum OUTDAV6W {}
impl OUTDAV6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OUTDAV6W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTDAV6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTDAV6W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `outdav7`"]
pub enum OUTDAV7W {}
impl OUTDAV7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OUTDAV7W<'a> {
    w: &'a mut W,
}
impl<'a> _OUTDAV7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OUTDAV7W) -> &'a mut W {
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
    #[doc = "Bit 0 - Endpoint 0 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav0(&self) -> OUTDAV0R {
        OUTDAV0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Endpoint 1 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav1(&self) -> OUTDAV1R {
        OUTDAV1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Endpoint 2 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav2(&self) -> OUTDAV2R {
        OUTDAV2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Endpoint 3 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav3(&self) -> OUTDAV3R {
        OUTDAV3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Endpoint 4 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav4(&self) -> OUTDAV4R {
        OUTDAV4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Endpoint 5 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav5(&self) -> OUTDAV5R {
        OUTDAV5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Endpoint 6 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav6(&self) -> OUTDAV6R {
        OUTDAV6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Endpoint 7 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav7(&self) -> OUTDAV7R {
        OUTDAV7R::_from({
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
    #[doc = "Bit 0 - Endpoint 0 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav0(&mut self) -> _OUTDAV0W {
        _OUTDAV0W { w: self }
    }
    #[doc = "Bit 1 - Endpoint 1 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav1(&mut self) -> _OUTDAV1W {
        _OUTDAV1W { w: self }
    }
    #[doc = "Bit 2 - Endpoint 2 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav2(&mut self) -> _OUTDAV2W {
        _OUTDAV2W { w: self }
    }
    #[doc = "Bit 3 - Endpoint 3 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav3(&mut self) -> _OUTDAV3W {
        _OUTDAV3W { w: self }
    }
    #[doc = "Bit 4 - Endpoint 4 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav4(&mut self) -> _OUTDAV4W {
        _OUTDAV4W { w: self }
    }
    #[doc = "Bit 5 - Endpoint 5 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav5(&mut self) -> _OUTDAV5W {
        _OUTDAV5W { w: self }
    }
    #[doc = "Bit 6 - Endpoint 6 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav6(&mut self) -> _OUTDAV6W {
        _OUTDAV6W { w: self }
    }
    #[doc = "Bit 7 - Endpoint 7 Data Available Interrupt Flag"]
    #[inline]
    pub fn outdav7(&mut self) -> _OUTDAV7W {
        _OUTDAV7W { w: self }
    }
}
