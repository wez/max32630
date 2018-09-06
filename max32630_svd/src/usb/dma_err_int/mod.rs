#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMA_ERR_INT {
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
#[doc = "Possible values of the field `dma_err0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ERR0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DMA_ERR0R {
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
            DMA_ERR0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_ERR0R {
        match value {
            i => DMA_ERR0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `dma_err1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ERR1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DMA_ERR1R {
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
            DMA_ERR1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_ERR1R {
        match value {
            i => DMA_ERR1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `dma_err2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ERR2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DMA_ERR2R {
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
            DMA_ERR2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_ERR2R {
        match value {
            i => DMA_ERR2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `dma_err3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ERR3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DMA_ERR3R {
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
            DMA_ERR3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_ERR3R {
        match value {
            i => DMA_ERR3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `dma_err4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ERR4R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DMA_ERR4R {
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
            DMA_ERR4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_ERR4R {
        match value {
            i => DMA_ERR4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `dma_err5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ERR5R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DMA_ERR5R {
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
            DMA_ERR5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_ERR5R {
        match value {
            i => DMA_ERR5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `dma_err6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ERR6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DMA_ERR6R {
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
            DMA_ERR6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_ERR6R {
        match value {
            i => DMA_ERR6R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `dma_err7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ERR7R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DMA_ERR7R {
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
            DMA_ERR7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMA_ERR7R {
        match value {
            i => DMA_ERR7R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `dma_err0`"]
pub enum DMA_ERR0W {}
impl DMA_ERR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ERR0W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ERR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_ERR0W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `dma_err1`"]
pub enum DMA_ERR1W {}
impl DMA_ERR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ERR1W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ERR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_ERR1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `dma_err2`"]
pub enum DMA_ERR2W {}
impl DMA_ERR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ERR2W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ERR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_ERR2W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `dma_err3`"]
pub enum DMA_ERR3W {}
impl DMA_ERR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ERR3W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ERR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_ERR3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `dma_err4`"]
pub enum DMA_ERR4W {}
impl DMA_ERR4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ERR4W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ERR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_ERR4W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `dma_err5`"]
pub enum DMA_ERR5W {}
impl DMA_ERR5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ERR5W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ERR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_ERR5W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `dma_err6`"]
pub enum DMA_ERR6W {}
impl DMA_ERR6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ERR6W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ERR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_ERR6W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `dma_err7`"]
pub enum DMA_ERR7W {}
impl DMA_ERR7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DMA_ERR7W<'a> {
    w: &'a mut W,
}
impl<'a> _DMA_ERR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMA_ERR7W) -> &'a mut W {
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
    #[doc = "Bit 0 - Endpoint 0 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err0(&self) -> DMA_ERR0R {
        DMA_ERR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Endpoint 1 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err1(&self) -> DMA_ERR1R {
        DMA_ERR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Endpoint 2 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err2(&self) -> DMA_ERR2R {
        DMA_ERR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Endpoint 3 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err3(&self) -> DMA_ERR3R {
        DMA_ERR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Endpoint 4 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err4(&self) -> DMA_ERR4R {
        DMA_ERR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Endpoint 5 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err5(&self) -> DMA_ERR5R {
        DMA_ERR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Endpoint 6 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err6(&self) -> DMA_ERR6R {
        DMA_ERR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Endpoint 7 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err7(&self) -> DMA_ERR7R {
        DMA_ERR7R::_from({
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
    #[doc = "Bit 0 - Endpoint 0 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err0(&mut self) -> _DMA_ERR0W {
        _DMA_ERR0W { w: self }
    }
    #[doc = "Bit 1 - Endpoint 1 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err1(&mut self) -> _DMA_ERR1W {
        _DMA_ERR1W { w: self }
    }
    #[doc = "Bit 2 - Endpoint 2 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err2(&mut self) -> _DMA_ERR2W {
        _DMA_ERR2W { w: self }
    }
    #[doc = "Bit 3 - Endpoint 3 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err3(&mut self) -> _DMA_ERR3W {
        _DMA_ERR3W { w: self }
    }
    #[doc = "Bit 4 - Endpoint 4 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err4(&mut self) -> _DMA_ERR4W {
        _DMA_ERR4W { w: self }
    }
    #[doc = "Bit 5 - Endpoint 5 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err5(&mut self) -> _DMA_ERR5W {
        _DMA_ERR5W { w: self }
    }
    #[doc = "Bit 6 - Endpoint 6 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err6(&mut self) -> _DMA_ERR6W {
        _DMA_ERR6W { w: self }
    }
    #[doc = "Bit 7 - Endpoint 7 DMA Error Interrupt Flag"]
    #[inline]
    pub fn dma_err7(&mut self) -> _DMA_ERR7W {
        _DMA_ERR7W { w: self }
    }
}
