#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TRIM_CALC {
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
#[doc = "Possible values of the field `trim_clk_sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_CLK_SELR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TRIM_CLK_SELR {
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
            TRIM_CLK_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIM_CLK_SELR {
        match value {
            i => TRIM_CLK_SELR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `trim_calc_start`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_CALC_STARTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TRIM_CALC_STARTR {
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
            TRIM_CALC_STARTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIM_CALC_STARTR {
        match value {
            i => TRIM_CALC_STARTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `trim_calc_completed`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_CALC_COMPLETEDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TRIM_CALC_COMPLETEDR {
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
            TRIM_CALC_COMPLETEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIM_CALC_COMPLETEDR {
        match value {
            i => TRIM_CALC_COMPLETEDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `trim_enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TRIM_ENABLER {
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
            TRIM_ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIM_ENABLER {
        match value {
            i => TRIM_ENABLER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `trim_calc_results`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIM_CALC_RESULTSR {
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl TRIM_CALC_RESULTSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            TRIM_CALC_RESULTSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> TRIM_CALC_RESULTSR {
        match value {
            i => TRIM_CALC_RESULTSR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `trim_clk_sel`"]
pub enum TRIM_CLK_SELW {}
impl TRIM_CLK_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_CLK_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_CLK_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIM_CLK_SELW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `trim_calc_start`"]
pub enum TRIM_CALC_STARTW {}
impl TRIM_CALC_STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_CALC_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_CALC_STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIM_CALC_STARTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `trim_enable`"]
pub enum TRIM_ENABLEW {}
impl TRIM_ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TRIM_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIM_ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIM_ENABLEW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Trim Clock Select"]
    #[inline]
    pub fn trim_clk_sel(&self) -> TRIM_CLK_SELR {
        TRIM_CLK_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Start Trim Calculation"]
    #[inline]
    pub fn trim_calc_start(&self) -> TRIM_CALC_STARTR {
        TRIM_CALC_STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Trim Calculation Completed"]
    #[inline]
    pub fn trim_calc_completed(&self) -> TRIM_CALC_COMPLETEDR {
        TRIM_CALC_COMPLETEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Trim Logic Enable"]
    #[inline]
    pub fn trim_enable(&self) -> TRIM_ENABLER {
        TRIM_ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:25 - Trim Calculation Results"]
    #[inline]
    pub fn trim_calc_results(&self) -> TRIM_CALC_RESULTSR {
        TRIM_CALC_RESULTSR::_from({
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bit 0 - Trim Clock Select"]
    #[inline]
    pub fn trim_clk_sel(&mut self) -> _TRIM_CLK_SELW {
        _TRIM_CLK_SELW { w: self }
    }
    #[doc = "Bit 1 - Start Trim Calculation"]
    #[inline]
    pub fn trim_calc_start(&mut self) -> _TRIM_CALC_STARTW {
        _TRIM_CALC_STARTW { w: self }
    }
    #[doc = "Bit 3 - Trim Logic Enable"]
    #[inline]
    pub fn trim_enable(&mut self) -> _TRIM_ENABLEW {
        _TRIM_ENABLEW { w: self }
    }
}
