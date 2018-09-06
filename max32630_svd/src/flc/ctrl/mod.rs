#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `write`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WRITER {
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
            WRITER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRITER {
        match value {
            i => WRITER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `mass_erase`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASS_ERASER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MASS_ERASER {
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
            MASS_ERASER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MASS_ERASER {
        match value {
            i => MASS_ERASER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `page_erase`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAGE_ERASER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PAGE_ERASER {
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
            PAGE_ERASER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PAGE_ERASER {
        match value {
            i => PAGE_ERASER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `erase_code`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASE_CODER {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ERASE_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ERASE_CODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ERASE_CODER {
        match value {
            i => ERASE_CODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `info_block_unlock`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INFO_BLOCK_UNLOCKR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl INFO_BLOCK_UNLOCKR {
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
            INFO_BLOCK_UNLOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INFO_BLOCK_UNLOCKR {
        match value {
            i => INFO_BLOCK_UNLOCKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `write_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WRITE_ENABLER {
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
            WRITE_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRITE_ENABLER {
        match value {
            i => WRITE_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pending`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PENDINGR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PENDINGR {
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
            PENDINGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PENDINGR {
        match value {
            i => PENDINGR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `info_block_valid`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INFO_BLOCK_VALIDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl INFO_BLOCK_VALIDR {
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
            INFO_BLOCK_VALIDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INFO_BLOCK_VALIDR {
        match value {
            i => INFO_BLOCK_VALIDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `auto_incre_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_INCRE_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl AUTO_INCRE_MODER {
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
            AUTO_INCRE_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTO_INCRE_MODER {
        match value {
            i => AUTO_INCRE_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `flsh_unlock`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLSH_UNLOCKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLSH_UNLOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLSH_UNLOCKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLSH_UNLOCKR {
        match value {
            i => FLSH_UNLOCKR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `write`"]
pub enum WRITEW {}
impl WRITEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WRITEW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRITEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `mass_erase`"]
pub enum MASS_ERASEW {}
impl MASS_ERASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MASS_ERASEW<'a> {
    w: &'a mut W,
}
impl<'a> _MASS_ERASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MASS_ERASEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `page_erase`"]
pub enum PAGE_ERASEW {}
impl PAGE_ERASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PAGE_ERASEW<'a> {
    w: &'a mut W,
}
impl<'a> _PAGE_ERASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAGE_ERASEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `erase_code`"]
pub enum ERASE_CODEW {}
impl ERASE_CODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ERASE_CODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ERASE_CODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERASE_CODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `auto_incre_mode`"]
pub enum AUTO_INCRE_MODEW {}
impl AUTO_INCRE_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _AUTO_INCRE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTO_INCRE_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTO_INCRE_MODEW) -> &'a mut W {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `flsh_unlock`"]
pub enum FLSH_UNLOCKW {}
impl FLSH_UNLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLSH_UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _FLSH_UNLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLSH_UNLOCKW) -> &'a mut W {
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
    #[doc = "Bit 0 - Start Flash Write Operation"]
    #[inline]
    pub fn write(&self) -> WRITER {
        WRITER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Start Flash Mass Erase Operation"]
    #[inline]
    pub fn mass_erase(&self) -> MASS_ERASER {
        MASS_ERASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Start Flash Page Erase Operation"]
    #[inline]
    pub fn page_erase(&self) -> PAGE_ERASER {
        PAGE_ERASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - Flash Erase Code"]
    #[inline]
    pub fn erase_code(&self) -> ERASE_CODER {
        ERASE_CODER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Flash Info Block Locked"]
    #[inline]
    pub fn info_block_unlock(&self) -> INFO_BLOCK_UNLOCKR {
        INFO_BLOCK_UNLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Flash Writes Enabled"]
    #[inline]
    pub fn write_enable(&self) -> WRITE_ENABLER {
        WRITE_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Flash Controller Status"]
    #[inline]
    pub fn pending(&self) -> PENDINGR {
        PENDINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Info Block Valid Status"]
    #[inline]
    pub fn info_block_valid(&self) -> INFO_BLOCK_VALIDR {
        INFO_BLOCK_VALIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Address Auto-Increment Mode"]
    #[inline]
    pub fn auto_incre_mode(&self) -> AUTO_INCRE_MODER {
        AUTO_INCRE_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:31 - Flash Write/Erase Enable"]
    #[inline]
    pub fn flsh_unlock(&self) -> FLSH_UNLOCKR {
        FLSH_UNLOCKR::_from({
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
    #[doc = "Bit 0 - Start Flash Write Operation"]
    #[inline]
    pub fn write(&mut self) -> _WRITEW {
        _WRITEW { w: self }
    }
    #[doc = "Bit 1 - Start Flash Mass Erase Operation"]
    #[inline]
    pub fn mass_erase(&mut self) -> _MASS_ERASEW {
        _MASS_ERASEW { w: self }
    }
    #[doc = "Bit 2 - Start Flash Page Erase Operation"]
    #[inline]
    pub fn page_erase(&mut self) -> _PAGE_ERASEW {
        _PAGE_ERASEW { w: self }
    }
    #[doc = "Bits 8:15 - Flash Erase Code"]
    #[inline]
    pub fn erase_code(&mut self) -> _ERASE_CODEW {
        _ERASE_CODEW { w: self }
    }
    #[doc = "Bit 27 - Address Auto-Increment Mode"]
    #[inline]
    pub fn auto_incre_mode(&mut self) -> _AUTO_INCRE_MODEW {
        _AUTO_INCRE_MODEW { w: self }
    }
    #[doc = "Bits 28:31 - Flash Write/Erase Enable"]
    #[inline]
    pub fn flsh_unlock(&mut self) -> _FLSH_UNLOCKW {
        _FLSH_UNLOCKW { w: self }
    }
}
