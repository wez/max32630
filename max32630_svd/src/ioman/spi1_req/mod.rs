#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPI1_REQ {
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
#[doc = "Possible values of the field `core_io_req`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CORE_IO_REQR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CORE_IO_REQR {
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
            CORE_IO_REQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CORE_IO_REQR {
        match value {
            i => CORE_IO_REQR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ss0_io_req`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS0_IO_REQR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SS0_IO_REQR {
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
            SS0_IO_REQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SS0_IO_REQR {
        match value {
            i => SS0_IO_REQR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ss1_io_req`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS1_IO_REQR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SS1_IO_REQR {
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
            SS1_IO_REQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SS1_IO_REQR {
        match value {
            i => SS1_IO_REQR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ss2_io_req`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS2_IO_REQR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SS2_IO_REQR {
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
            SS2_IO_REQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SS2_IO_REQR {
        match value {
            i => SS2_IO_REQR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `quad_io_req`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QUAD_IO_REQR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl QUAD_IO_REQR {
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
            QUAD_IO_REQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> QUAD_IO_REQR {
        match value {
            i => QUAD_IO_REQR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `fast_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FAST_MODER {
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
            FAST_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAST_MODER {
        match value {
            i => FAST_MODER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `core_io_req`"]
pub enum CORE_IO_REQW {}
impl CORE_IO_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CORE_IO_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _CORE_IO_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CORE_IO_REQW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ss0_io_req`"]
pub enum SS0_IO_REQW {}
impl SS0_IO_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SS0_IO_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _SS0_IO_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS0_IO_REQW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ss1_io_req`"]
pub enum SS1_IO_REQW {}
impl SS1_IO_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SS1_IO_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _SS1_IO_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS1_IO_REQW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ss2_io_req`"]
pub enum SS2_IO_REQW {}
impl SS2_IO_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SS2_IO_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _SS2_IO_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS2_IO_REQW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `quad_io_req`"]
pub enum QUAD_IO_REQW {}
impl QUAD_IO_REQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _QUAD_IO_REQW<'a> {
    w: &'a mut W,
}
impl<'a> _QUAD_IO_REQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: QUAD_IO_REQW) -> &'a mut W {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `fast_mode`"]
pub enum FAST_MODEW {}
impl FAST_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FAST_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _FAST_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAST_MODEW) -> &'a mut W {
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
    #[doc = "Bit 4 - SPI Master 1 Core I/O Request"]
    #[inline]
    pub fn core_io_req(&self) -> CORE_IO_REQR {
        CORE_IO_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - SPI Master 1 SS[0] I/O Request"]
    #[inline]
    pub fn ss0_io_req(&self) -> SS0_IO_REQR {
        SS0_IO_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - SPI Master 1 SS[1] I/O Request"]
    #[inline]
    pub fn ss1_io_req(&self) -> SS1_IO_REQR {
        SS1_IO_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - SPI Master 1 SS[2] I/O Request"]
    #[inline]
    pub fn ss2_io_req(&self) -> SS2_IO_REQR {
        SS2_IO_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - SPI Master 1 Quad I/O Request"]
    #[inline]
    pub fn quad_io_req(&self) -> QUAD_IO_REQR {
        QUAD_IO_REQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - SPI Master 1 Fast Mode Request"]
    #[inline]
    pub fn fast_mode(&self) -> FAST_MODER {
        FAST_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 4 - SPI Master 1 Core I/O Request"]
    #[inline]
    pub fn core_io_req(&mut self) -> _CORE_IO_REQW {
        _CORE_IO_REQW { w: self }
    }
    #[doc = "Bit 8 - SPI Master 1 SS[0] I/O Request"]
    #[inline]
    pub fn ss0_io_req(&mut self) -> _SS0_IO_REQW {
        _SS0_IO_REQW { w: self }
    }
    #[doc = "Bit 9 - SPI Master 1 SS[1] I/O Request"]
    #[inline]
    pub fn ss1_io_req(&mut self) -> _SS1_IO_REQW {
        _SS1_IO_REQW { w: self }
    }
    #[doc = "Bit 10 - SPI Master 1 SS[2] I/O Request"]
    #[inline]
    pub fn ss2_io_req(&mut self) -> _SS2_IO_REQW {
        _SS2_IO_REQW { w: self }
    }
    #[doc = "Bit 20 - SPI Master 1 Quad I/O Request"]
    #[inline]
    pub fn quad_io_req(&mut self) -> _QUAD_IO_REQW {
        _QUAD_IO_REQW { w: self }
    }
    #[doc = "Bit 24 - SPI Master 1 Fast Mode Request"]
    #[inline]
    pub fn fast_mode(&mut self) -> _FAST_MODEW {
        _FAST_MODEW { w: self }
    }
}
