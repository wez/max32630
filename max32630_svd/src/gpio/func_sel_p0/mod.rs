#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FUNC_SEL_P0 {
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
#[doc = "Possible values of the field `pin0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN0R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PIN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIN0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIN0R {
        match value {
            i => PIN0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pin1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN1R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PIN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIN1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIN1R {
        match value {
            i => PIN1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pin2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN2R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PIN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIN2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIN2R {
        match value {
            i => PIN2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pin3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN3R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PIN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIN3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIN3R {
        match value {
            i => PIN3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pin4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN4R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PIN4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIN4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIN4R {
        match value {
            i => PIN4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pin5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN5R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PIN5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIN5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIN5R {
        match value {
            i => PIN5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pin6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN6R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PIN6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIN6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIN6R {
        match value {
            i => PIN6R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pin7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN7R {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PIN7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PIN7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PIN7R {
        match value {
            i => PIN7R::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `pin0`"]
pub enum PIN0W {}
impl PIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin1`"]
pub enum PIN1W {}
impl PIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin2`"]
pub enum PIN2W {}
impl PIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin3`"]
pub enum PIN3W {}
impl PIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN3W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pin4`"]
pub enum PIN4W {}
impl PIN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PIN4W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN4W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pin5`"]
pub enum PIN5W {}
impl PIN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PIN5W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin6`"]
pub enum PIN6W {}
impl PIN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PIN6W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pin7`"]
pub enum PIN7W {}
impl PIN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PIN7W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - P0.0 Output Function Select"]
    #[inline]
    pub fn pin0(&self) -> PIN0R {
        PIN0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - P0.1 Output Function Select"]
    #[inline]
    pub fn pin1(&self) -> PIN1R {
        PIN1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - P0.2 Output Function Select"]
    #[inline]
    pub fn pin2(&self) -> PIN2R {
        PIN2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - P0.3 Output Function Select"]
    #[inline]
    pub fn pin3(&self) -> PIN3R {
        PIN3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - P0.4 Output Function Select"]
    #[inline]
    pub fn pin4(&self) -> PIN4R {
        PIN4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - P0.5 Output Function Select"]
    #[inline]
    pub fn pin5(&self) -> PIN5R {
        PIN5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - P0.6 Output Function Select"]
    #[inline]
    pub fn pin6(&self) -> PIN6R {
        PIN6R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - P0.7 Output Function Select"]
    #[inline]
    pub fn pin7(&self) -> PIN7R {
        PIN7R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - P0.0 Output Function Select"]
    #[inline]
    pub fn pin0(&mut self) -> _PIN0W {
        _PIN0W { w: self }
    }
    #[doc = "Bits 4:7 - P0.1 Output Function Select"]
    #[inline]
    pub fn pin1(&mut self) -> _PIN1W {
        _PIN1W { w: self }
    }
    #[doc = "Bits 8:11 - P0.2 Output Function Select"]
    #[inline]
    pub fn pin2(&mut self) -> _PIN2W {
        _PIN2W { w: self }
    }
    #[doc = "Bits 12:15 - P0.3 Output Function Select"]
    #[inline]
    pub fn pin3(&mut self) -> _PIN3W {
        _PIN3W { w: self }
    }
    #[doc = "Bits 16:19 - P0.4 Output Function Select"]
    #[inline]
    pub fn pin4(&mut self) -> _PIN4W {
        _PIN4W { w: self }
    }
    #[doc = "Bits 20:23 - P0.5 Output Function Select"]
    #[inline]
    pub fn pin5(&mut self) -> _PIN5W {
        _PIN5W { w: self }
    }
    #[doc = "Bits 24:27 - P0.6 Output Function Select"]
    #[inline]
    pub fn pin6(&mut self) -> _PIN6W {
        _PIN6W { w: self }
    }
    #[doc = "Bits 28:31 - P0.7 Output Function Select"]
    #[inline]
    pub fn pin7(&mut self) -> _PIN7W {
        _PIN7W { w: self }
    }
}
