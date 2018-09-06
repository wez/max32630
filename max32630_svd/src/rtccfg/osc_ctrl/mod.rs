#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSC_CTRL {
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
#[doc = "Possible values of the field `osc_bypass`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_BYPASSR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OSC_BYPASSR {
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
            OSC_BYPASSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSC_BYPASSR {
        match value {
            i => OSC_BYPASSR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `osc_disable_r`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_DISABLE_RR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OSC_DISABLE_RR {
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
            OSC_DISABLE_RR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSC_DISABLE_RR {
        match value {
            i => OSC_DISABLE_RR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `osc_disable_sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_DISABLE_SELR {
    #[doc = "PowerSequencer controls the reset state of the RTC"]
    PWRSEQ_CONTROL,
    #[doc = "RTC reset controlled by osc_disable_r bit"]
    RTC_DOMAIN_CONTROL,
}
impl OSC_DISABLE_SELR {
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
            OSC_DISABLE_SELR::PWRSEQ_CONTROL => false,
            OSC_DISABLE_SELR::RTC_DOMAIN_CONTROL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSC_DISABLE_SELR {
        match value {
            false => OSC_DISABLE_SELR::PWRSEQ_CONTROL,
            true => OSC_DISABLE_SELR::RTC_DOMAIN_CONTROL,
        }
    }
    #[doc = "Checks if the value of the field is `PWRSEQ_CONTROL`"]
    #[inline]
    pub fn is_pwr_seq_control(&self) -> bool {
        *self == OSC_DISABLE_SELR::PWRSEQ_CONTROL
    }
    #[doc = "Checks if the value of the field is `RTC_DOMAIN_CONTROL`"]
    #[inline]
    pub fn is_rtc_domain_control(&self) -> bool {
        *self == OSC_DISABLE_SELR::RTC_DOMAIN_CONTROL
    }
}
#[doc = "Possible values of the field `osc_disable_o`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_DISABLE_OR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OSC_DISABLE_OR {
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
            OSC_DISABLE_OR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSC_DISABLE_OR {
        match value {
            i => OSC_DISABLE_OR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `osc_bypass`"]
pub enum OSC_BYPASSW {}
impl OSC_BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OSC_BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC_BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSC_BYPASSW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `osc_disable_r`"]
pub enum OSC_DISABLE_RW {}
impl OSC_DISABLE_RW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OSC_DISABLE_RW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC_DISABLE_RW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSC_DISABLE_RW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `osc_disable_sel`"]
pub enum OSC_DISABLE_SELW {
    #[doc = "PowerSequencer controls the reset state of the RTC"]
    PWRSEQ_CONTROL,
    #[doc = "RTC reset controlled by osc_disable_r bit"]
    RTC_DOMAIN_CONTROL,
}
impl OSC_DISABLE_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSC_DISABLE_SELW::PWRSEQ_CONTROL => false,
            OSC_DISABLE_SELW::RTC_DOMAIN_CONTROL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSC_DISABLE_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _OSC_DISABLE_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSC_DISABLE_SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PowerSequencer controls the reset state of the RTC"]
    #[inline]
    pub fn pwr_seq_control(self) -> &'a mut W {
        self.variant(OSC_DISABLE_SELW::PWRSEQ_CONTROL)
    }
    #[doc = "RTC reset controlled by osc_disable_r bit"]
    #[inline]
    pub fn rtc_domain_control(self) -> &'a mut W {
        self.variant(OSC_DISABLE_SELW::RTC_DOMAIN_CONTROL)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Bypass RTC oscillator"]
    #[inline]
    pub fn osc_bypass(&self) -> OSC_BYPASSR {
        OSC_BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - if osc_disable_sel = 1, this will hold the RTC in reset."]
    #[inline]
    pub fn osc_disable_r(&self) -> OSC_DISABLE_RR {
        OSC_DISABLE_RR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Select RTC Oscillator Disable Control Source"]
    #[inline]
    pub fn osc_disable_sel(&self) -> OSC_DISABLE_SELR {
        OSC_DISABLE_SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Reset RTC Oscillator"]
    #[inline]
    pub fn osc_disable_o(&self) -> OSC_DISABLE_OR {
        OSC_DISABLE_OR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Bypass RTC oscillator"]
    #[inline]
    pub fn osc_bypass(&mut self) -> _OSC_BYPASSW {
        _OSC_BYPASSW { w: self }
    }
    #[doc = "Bit 1 - if osc_disable_sel = 1, this will hold the RTC in reset."]
    #[inline]
    pub fn osc_disable_r(&mut self) -> _OSC_DISABLE_RW {
        _OSC_DISABLE_RW { w: self }
    }
    #[doc = "Bit 2 - Select RTC Oscillator Disable Control Source"]
    #[inline]
    pub fn osc_disable_sel(&mut self) -> _OSC_DISABLE_SELW {
        _OSC_DISABLE_SELW { w: self }
    }
}
