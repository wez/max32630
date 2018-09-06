#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN1 {
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
#[doc = "Possible values of the field `sram_addr_wrapped`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_ADDR_WRAPPEDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SRAM_ADDR_WRAPPEDR {
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
            SRAM_ADDR_WRAPPEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRAM_ADDR_WRAPPEDR {
        match value {
            i => SRAM_ADDR_WRAPPEDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `invalid_flash_addr`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALID_FLASH_ADDRR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl INVALID_FLASH_ADDRR {
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
            INVALID_FLASH_ADDRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVALID_FLASH_ADDRR {
        match value {
            i => INVALID_FLASH_ADDRR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `flash_read_locked`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_READ_LOCKEDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLASH_READ_LOCKEDR {
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
            FLASH_READ_LOCKEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH_READ_LOCKEDR {
        match value {
            i => FLASH_READ_LOCKEDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `trim_update_done`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_UPDATE_DONER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TRIM_UPDATE_DONER {
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
            TRIM_UPDATE_DONER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIM_UPDATE_DONER {
        match value {
            i => TRIM_UPDATE_DONER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `flc_state_done`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLC_STATE_DONER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLC_STATE_DONER {
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
            FLC_STATE_DONER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLC_STATE_DONER {
        match value {
            i => FLC_STATE_DONER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `flc_prog_complete`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLC_PROG_COMPLETER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FLC_PROG_COMPLETER {
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
            FLC_PROG_COMPLETER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLC_PROG_COMPLETER {
        match value {
            i => FLC_PROG_COMPLETER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `sram_addr_wrapped`"]
pub enum SRAM_ADDR_WRAPPEDW {}
impl SRAM_ADDR_WRAPPEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SRAM_ADDR_WRAPPEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SRAM_ADDR_WRAPPEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRAM_ADDR_WRAPPEDW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `invalid_flash_addr`"]
pub enum INVALID_FLASH_ADDRW {}
impl INVALID_FLASH_ADDRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _INVALID_FLASH_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _INVALID_FLASH_ADDRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVALID_FLASH_ADDRW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `flash_read_locked`"]
pub enum FLASH_READ_LOCKEDW {}
impl FLASH_READ_LOCKEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLASH_READ_LOCKEDW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_READ_LOCKEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH_READ_LOCKEDW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `trim_update_done`"]
pub enum TRIM_UPDATE_DONEW {}
impl TRIM_UPDATE_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_UPDATE_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_UPDATE_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIM_UPDATE_DONEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `flc_state_done`"]
pub enum FLC_STATE_DONEW {}
impl FLC_STATE_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLC_STATE_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLC_STATE_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLC_STATE_DONEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `flc_prog_complete`"]
pub enum FLC_PROG_COMPLETEW {}
impl FLC_PROG_COMPLETEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FLC_PROG_COMPLETEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLC_PROG_COMPLETEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLC_PROG_COMPLETEW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SRAM Address Wrapped Interrupt Enable/Disable"]
    #[inline]
    pub fn sram_addr_wrapped(&self) -> SRAM_ADDR_WRAPPEDR {
        SRAM_ADDR_WRAPPEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Invalid Flash Address Interrupt Enable/Disable"]
    #[inline]
    pub fn invalid_flash_addr(&self) -> INVALID_FLASH_ADDRR {
        INVALID_FLASH_ADDRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Flash Read from Locked Area Interrupt Enable/Disable"]
    #[inline]
    pub fn flash_read_locked(&self) -> FLASH_READ_LOCKEDR {
        FLASH_READ_LOCKEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Trim Update Complete Interrupt Enable/Disable"]
    #[inline]
    pub fn trim_update_done(&self) -> TRIM_UPDATE_DONER {
        TRIM_UPDATE_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - FLC State Machine Reached DONE Interrupt Enable/Disable"]
    #[inline]
    pub fn flc_state_done(&self) -> FLC_STATE_DONER {
        FLC_STATE_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Program (Write or Erase) Op Completed Int Enable/Disable"]
    #[inline]
    pub fn flc_prog_complete(&self) -> FLC_PROG_COMPLETER {
        FLC_PROG_COMPLETER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - SRAM Address Wrapped Interrupt Enable/Disable"]
    #[inline]
    pub fn sram_addr_wrapped(&mut self) -> _SRAM_ADDR_WRAPPEDW {
        _SRAM_ADDR_WRAPPEDW { w: self }
    }
    #[doc = "Bit 1 - Invalid Flash Address Interrupt Enable/Disable"]
    #[inline]
    pub fn invalid_flash_addr(&mut self) -> _INVALID_FLASH_ADDRW {
        _INVALID_FLASH_ADDRW { w: self }
    }
    #[doc = "Bit 2 - Flash Read from Locked Area Interrupt Enable/Disable"]
    #[inline]
    pub fn flash_read_locked(&mut self) -> _FLASH_READ_LOCKEDW {
        _FLASH_READ_LOCKEDW { w: self }
    }
    #[doc = "Bit 3 - Trim Update Complete Interrupt Enable/Disable"]
    #[inline]
    pub fn trim_update_done(&mut self) -> _TRIM_UPDATE_DONEW {
        _TRIM_UPDATE_DONEW { w: self }
    }
    #[doc = "Bit 4 - FLC State Machine Reached DONE Interrupt Enable/Disable"]
    #[inline]
    pub fn flc_state_done(&mut self) -> _FLC_STATE_DONEW {
        _FLC_STATE_DONEW { w: self }
    }
    #[doc = "Bit 5 - Program (Write or Erase) Op Completed Int Enable/Disable"]
    #[inline]
    pub fn flc_prog_complete(&mut self) -> _FLC_PROG_COMPLETEW {
        _FLC_PROG_COMPLETEW { w: self }
    }
}
