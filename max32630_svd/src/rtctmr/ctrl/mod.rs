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
#[doc = "Possible values of the field `enable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ENABLER {
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
            ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            i => ENABLER::_Reserved(i),
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
#[doc = "Possible values of the field `use_async_flags`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USE_ASYNC_FLAGSR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl USE_ASYNC_FLAGSR {
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
            USE_ASYNC_FLAGSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USE_ASYNC_FLAGSR {
        match value {
            i => USE_ASYNC_FLAGSR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `aggressive_rst`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AGGRESSIVE_RSTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl AGGRESSIVE_RSTR {
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
            AGGRESSIVE_RSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AGGRESSIVE_RSTR {
        match value {
            i => AGGRESSIVE_RSTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `en_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EN_ACTIVER {
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
            EN_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_ACTIVER {
        match value {
            i => EN_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `osc_goto_low_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_GOTO_LOW_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OSC_GOTO_LOW_ACTIVER {
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
            OSC_GOTO_LOW_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSC_GOTO_LOW_ACTIVER {
        match value {
            i => OSC_GOTO_LOW_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `osc_frce_sm_en_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_FRCE_SM_EN_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OSC_FRCE_SM_EN_ACTIVER {
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
            OSC_FRCE_SM_EN_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSC_FRCE_SM_EN_ACTIVER {
        match value {
            i => OSC_FRCE_SM_EN_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `osc_frce_st_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_FRCE_ST_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OSC_FRCE_ST_ACTIVER {
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
            OSC_FRCE_ST_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSC_FRCE_ST_ACTIVER {
        match value {
            i => OSC_FRCE_ST_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `set_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SET_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SET_ACTIVER {
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
            SET_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SET_ACTIVER {
        match value {
            i => SET_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `clr_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CLR_ACTIVER {
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
            CLR_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLR_ACTIVER {
        match value {
            i => CLR_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rollover_clr_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROLLOVER_CLR_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ROLLOVER_CLR_ACTIVER {
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
            ROLLOVER_CLR_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ROLLOVER_CLR_ACTIVER {
        match value {
            i => ROLLOVER_CLR_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `prescale_cmpr0_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALE_CMPR0_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PRESCALE_CMPR0_ACTIVER {
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
            PRESCALE_CMPR0_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRESCALE_CMPR0_ACTIVER {
        match value {
            i => PRESCALE_CMPR0_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `prescale_update_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALE_UPDATE_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PRESCALE_UPDATE_ACTIVER {
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
            PRESCALE_UPDATE_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRESCALE_UPDATE_ACTIVER {
        match value {
            i => PRESCALE_UPDATE_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `cmpr1_clr_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPR1_CLR_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CMPR1_CLR_ACTIVER {
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
            CMPR1_CLR_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMPR1_CLR_ACTIVER {
        match value {
            i => CMPR1_CLR_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `cmpr0_clr_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPR0_CLR_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl CMPR0_CLR_ACTIVER {
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
            CMPR0_CLR_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMPR0_CLR_ACTIVER {
        match value {
            i => CMPR0_CLR_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `enable`"]
pub enum ENABLEW {}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `clear`"]
pub enum CLEARW {}
impl CLEARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CLEARW<'a> {
    w: &'a mut W,
}
impl<'a> _CLEARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLEARW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `use_async_flags`"]
pub enum USE_ASYNC_FLAGSW {}
impl USE_ASYNC_FLAGSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _USE_ASYNC_FLAGSW<'a> {
    w: &'a mut W,
}
impl<'a> _USE_ASYNC_FLAGSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USE_ASYNC_FLAGSW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `aggressive_rst`"]
pub enum AGGRESSIVE_RSTW {}
impl AGGRESSIVE_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _AGGRESSIVE_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _AGGRESSIVE_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AGGRESSIVE_RSTW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RTC Transaction Pending"]
    #[inline]
    pub fn pending(&self) -> PENDINGR {
        PENDINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Use Async RTC Flags"]
    #[inline]
    pub fn use_async_flags(&self) -> USE_ASYNC_FLAGSR {
        USE_ASYNC_FLAGSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Use Aggressive Reset Mode"]
    #[inline]
    pub fn aggressive_rst(&self) -> AGGRESSIVE_RSTR {
        AGGRESSIVE_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable RTC in Active Modes"]
    #[inline]
    pub fn en_active(&self) -> EN_ACTIVER {
        EN_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - osc_goto_low_r transaction is pending"]
    #[inline]
    pub fn osc_goto_low_active(&self) -> OSC_GOTO_LOW_ACTIVER {
        OSC_GOTO_LOW_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - osc_force_mode transaction is pending"]
    #[inline]
    pub fn osc_frce_sm_en_active(&self) -> OSC_FRCE_SM_EN_ACTIVER {
        OSC_FRCE_SM_EN_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - osc_force_state transaction is pending"]
    #[inline]
    pub fn osc_frce_st_active(&self) -> OSC_FRCE_ST_ACTIVER {
        OSC_FRCE_ST_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - timer_set_active"]
    #[inline]
    pub fn set_active(&self) -> SET_ACTIVER {
        SET_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - RTC clear is pending"]
    #[inline]
    pub fn clr_active(&self) -> CLR_ACTIVER {
        CLR_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - rollover clr is pending"]
    #[inline]
    pub fn rollover_clr_active(&self) -> ROLLOVER_CLR_ACTIVER {
        ROLLOVER_CLR_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - prescale cmpr0 is pending"]
    #[inline]
    pub fn prescale_cmpr0_active(&self) -> PRESCALE_CMPR0_ACTIVER {
        PRESCALE_CMPR0_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - prescale update transaction is pending"]
    #[inline]
    pub fn prescale_update_active(&self) -> PRESCALE_UPDATE_ACTIVER {
        PRESCALE_UPDATE_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - cmpr1 clear transaction is pending"]
    #[inline]
    pub fn cmpr1_clr_active(&self) -> CMPR1_CLR_ACTIVER {
        CMPR1_CLR_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - cmpr0 clear transaction is pending"]
    #[inline]
    pub fn cmpr0_clr_active(&self) -> CMPR0_CLR_ACTIVER {
        CMPR0_CLR_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - RTC Timer Clear Bit"]
    #[inline]
    pub fn clear(&mut self) -> _CLEARW {
        _CLEARW { w: self }
    }
    #[doc = "Bit 3 - Use Async RTC Flags"]
    #[inline]
    pub fn use_async_flags(&mut self) -> _USE_ASYNC_FLAGSW {
        _USE_ASYNC_FLAGSW { w: self }
    }
    #[doc = "Bit 4 - Use Aggressive Reset Mode"]
    #[inline]
    pub fn aggressive_rst(&mut self) -> _AGGRESSIVE_RSTW {
        _AGGRESSIVE_RSTW { w: self }
    }
}
