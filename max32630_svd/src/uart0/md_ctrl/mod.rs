#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MD_CTRL {
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
#[doc = "Possible values of the field `slave_addr`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_ADDRR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLAVE_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLAVE_ADDRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLAVE_ADDRR {
        match value {
            i => SLAVE_ADDRR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `slave_addr_msk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_ADDR_MSKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLAVE_ADDR_MSKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLAVE_ADDR_MSKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLAVE_ADDR_MSKR {
        match value {
            i => SLAVE_ADDR_MSKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `md_mstr`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MD_MSTRR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MD_MSTRR {
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
            MD_MSTRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MD_MSTRR {
        match value {
            i => MD_MSTRR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tx_addr_mark`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_ADDR_MARKR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TX_ADDR_MARKR {
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
            TX_ADDR_MARKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TX_ADDR_MARKR {
        match value {
            i => TX_ADDR_MARKR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `slave_addr`"]
pub enum SLAVE_ADDRW {}
impl SLAVE_ADDRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SLAVE_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _SLAVE_ADDRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLAVE_ADDRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `slave_addr_msk`"]
pub enum SLAVE_ADDR_MSKW {}
impl SLAVE_ADDR_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SLAVE_ADDR_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _SLAVE_ADDR_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLAVE_ADDR_MSKW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `md_mstr`"]
pub enum MD_MSTRW {}
impl MD_MSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MD_MSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _MD_MSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MD_MSTRW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `tx_addr_mark`"]
pub enum TX_ADDR_MARKW {}
impl TX_ADDR_MARKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TX_ADDR_MARKW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_ADDR_MARKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_ADDR_MARKW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Slave Address"]
    #[inline]
    pub fn slave_addr(&self) -> SLAVE_ADDRR {
        SLAVE_ADDRR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Slave Address Mask"]
    #[inline]
    pub fn slave_addr_msk(&self) -> SLAVE_ADDR_MSKR {
        SLAVE_ADDR_MSKR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Multidrop Master"]
    #[inline]
    pub fn md_mstr(&self) -> MD_MSTRR {
        MD_MSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - RX Address Mark"]
    #[inline]
    pub fn tx_addr_mark(&self) -> TX_ADDR_MARKR {
        TX_ADDR_MARKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
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
    #[doc = "Bits 0:7 - Slave Address"]
    #[inline]
    pub fn slave_addr(&mut self) -> _SLAVE_ADDRW {
        _SLAVE_ADDRW { w: self }
    }
    #[doc = "Bits 8:15 - Slave Address Mask"]
    #[inline]
    pub fn slave_addr_msk(&mut self) -> _SLAVE_ADDR_MSKW {
        _SLAVE_ADDR_MSKW { w: self }
    }
    #[doc = "Bit 16 - Multidrop Master"]
    #[inline]
    pub fn md_mstr(&mut self) -> _MD_MSTRW {
        _MD_MSTRW { w: self }
    }
    #[doc = "Bit 17 - RX Address Mark"]
    #[inline]
    pub fn tx_addr_mark(&mut self) -> _TX_ADDR_MARKW {
        _TX_ADDR_MARKW { w: self }
    }
}
