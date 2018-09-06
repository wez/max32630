#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG4 {
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
#[doc = "Possible values of the field `pwr_tm_ps_2_gpio`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TM_PS_2_GPIOR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_TM_PS_2_GPIOR {
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
            PWR_TM_PS_2_GPIOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_TM_PS_2_GPIOR {
        match value {
            i => PWR_TM_PS_2_GPIOR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_tm_fast_timers`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TM_FAST_TIMERSR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_TM_FAST_TIMERSR {
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
            PWR_TM_FAST_TIMERSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_TM_FAST_TIMERSR {
        match value {
            i => PWR_TM_FAST_TIMERSR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_usb_dis_comp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_USB_DIS_COMPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_USB_DIS_COMPR {
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
            PWR_USB_DIS_COMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_USB_DIS_COMPR {
        match value {
            i => PWR_USB_DIS_COMPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_ro_tstclk_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_RO_TSTCLK_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_RO_TSTCLK_ENR {
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
            PWR_RO_TSTCLK_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_RO_TSTCLK_ENR {
        match value {
            i => PWR_RO_TSTCLK_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_nr_clk_gate_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_NR_CLK_GATE_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_NR_CLK_GATE_ENR {
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
            PWR_NR_CLK_GATE_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_NR_CLK_GATE_ENR {
        match value {
            i => PWR_NR_CLK_GATE_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_ext_clk_in_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_EXT_CLK_IN_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_EXT_CLK_IN_ENR {
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
            PWR_EXT_CLK_IN_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_EXT_CLK_IN_ENR {
        match value {
            i => PWR_EXT_CLK_IN_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_pseq_32k_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_PSEQ_32K_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_PSEQ_32K_ENR {
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
            PWR_PSEQ_32K_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_PSEQ_32K_ENR {
        match value {
            i => PWR_PSEQ_32K_ENR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `pwr_tm_ps_2_gpio`"]
pub enum PWR_TM_PS_2_GPIOW {}
impl PWR_TM_PS_2_GPIOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TM_PS_2_GPIOW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TM_PS_2_GPIOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TM_PS_2_GPIOW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_tm_fast_timers`"]
pub enum PWR_TM_FAST_TIMERSW {}
impl PWR_TM_FAST_TIMERSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TM_FAST_TIMERSW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TM_FAST_TIMERSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TM_FAST_TIMERSW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_usb_dis_comp`"]
pub enum PWR_USB_DIS_COMPW {}
impl PWR_USB_DIS_COMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_USB_DIS_COMPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_USB_DIS_COMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_USB_DIS_COMPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_ro_tstclk_en`"]
pub enum PWR_RO_TSTCLK_ENW {}
impl PWR_RO_TSTCLK_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_RO_TSTCLK_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_RO_TSTCLK_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_RO_TSTCLK_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_nr_clk_gate_en`"]
pub enum PWR_NR_CLK_GATE_ENW {}
impl PWR_NR_CLK_GATE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_NR_CLK_GATE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_NR_CLK_GATE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_NR_CLK_GATE_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_ext_clk_in_en`"]
pub enum PWR_EXT_CLK_IN_ENW {}
impl PWR_EXT_CLK_IN_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_EXT_CLK_IN_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_EXT_CLK_IN_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_EXT_CLK_IN_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_pseq_32k_en`"]
pub enum PWR_PSEQ_32K_ENW {}
impl PWR_PSEQ_32K_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_PSEQ_32K_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_PSEQ_32K_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_PSEQ_32K_ENW) -> &'a mut W {
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
    #[doc = "Bit 0 - Internal Use Only"]
    #[inline]
    pub fn pwr_tm_ps_2_gpio(&self) -> PWR_TM_PS_2_GPIOR {
        PWR_TM_PS_2_GPIOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Internal Use Only"]
    #[inline]
    pub fn pwr_tm_fast_timers(&self) -> PWR_TM_FAST_TIMERSR {
        PWR_TM_FAST_TIMERSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Internal Use Only"]
    #[inline]
    pub fn pwr_usb_dis_comp(&self) -> PWR_USB_DIS_COMPR {
        PWR_USB_DIS_COMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Internal Use Only"]
    #[inline]
    pub fn pwr_ro_tstclk_en(&self) -> PWR_RO_TSTCLK_ENR {
        PWR_RO_TSTCLK_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Internal Use Only"]
    #[inline]
    pub fn pwr_nr_clk_gate_en(&self) -> PWR_NR_CLK_GATE_ENR {
        PWR_NR_CLK_GATE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Internal Use Only"]
    #[inline]
    pub fn pwr_ext_clk_in_en(&self) -> PWR_EXT_CLK_IN_ENR {
        PWR_EXT_CLK_IN_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Internal Use Only"]
    #[inline]
    pub fn pwr_pseq_32k_en(&self) -> PWR_PSEQ_32K_ENR {
        PWR_PSEQ_32K_ENR::_from({
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
    #[doc = "Bit 0 - Internal Use Only"]
    #[inline]
    pub fn pwr_tm_ps_2_gpio(&mut self) -> _PWR_TM_PS_2_GPIOW {
        _PWR_TM_PS_2_GPIOW { w: self }
    }
    #[doc = "Bit 1 - Internal Use Only"]
    #[inline]
    pub fn pwr_tm_fast_timers(&mut self) -> _PWR_TM_FAST_TIMERSW {
        _PWR_TM_FAST_TIMERSW { w: self }
    }
    #[doc = "Bit 3 - Internal Use Only"]
    #[inline]
    pub fn pwr_usb_dis_comp(&mut self) -> _PWR_USB_DIS_COMPW {
        _PWR_USB_DIS_COMPW { w: self }
    }
    #[doc = "Bit 4 - Internal Use Only"]
    #[inline]
    pub fn pwr_ro_tstclk_en(&mut self) -> _PWR_RO_TSTCLK_ENW {
        _PWR_RO_TSTCLK_ENW { w: self }
    }
    #[doc = "Bit 5 - Internal Use Only"]
    #[inline]
    pub fn pwr_nr_clk_gate_en(&mut self) -> _PWR_NR_CLK_GATE_ENW {
        _PWR_NR_CLK_GATE_ENW { w: self }
    }
    #[doc = "Bit 6 - Internal Use Only"]
    #[inline]
    pub fn pwr_ext_clk_in_en(&mut self) -> _PWR_EXT_CLK_IN_ENW {
        _PWR_EXT_CLK_IN_ENW { w: self }
    }
    #[doc = "Bit 7 - Internal Use Only"]
    #[inline]
    pub fn pwr_pseq_32k_en(&mut self) -> _PWR_PSEQ_32K_ENW {
        _PWR_PSEQ_32K_ENW { w: self }
    }
}
