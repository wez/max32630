#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MASTER_CFG {
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
#[doc = "Possible values of the field `spi_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI_MODER {
    #[doc = "SCK is active high, data is sampled on clock rising edge."]
    SCK_HI_SAMPLE_RISING,
    #[doc = "SCK is active low, data is sampled on clock rising edge."]
    SCK_LO_SAMPLE_FALLING,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPI_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPI_MODER::SCK_HI_SAMPLE_RISING => 0,
            SPI_MODER::SCK_LO_SAMPLE_FALLING => 3,
            SPI_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPI_MODER {
        match value {
            0 => SPI_MODER::SCK_HI_SAMPLE_RISING,
            3 => SPI_MODER::SCK_LO_SAMPLE_FALLING,
            i => SPI_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SCK_HI_SAMPLE_RISING`"]
    #[inline]
    pub fn is_sck_hi_sample_rising(&self) -> bool {
        *self == SPI_MODER::SCK_HI_SAMPLE_RISING
    }
    #[doc = "Checks if the value of the field is `SCK_LO_SAMPLE_FALLING`"]
    #[inline]
    pub fn is_sck_lo_sample_falling(&self) -> bool {
        *self == SPI_MODER::SCK_LO_SAMPLE_FALLING
    }
}
#[doc = "Possible values of the field `ss_act_lo`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_ACT_LOR {
    #[doc = "Enabled slave select (SS) is active high."]
    ACTIVE_HIGH,
    #[doc = "Enabled slave select (SS) is active low."]
    ACTIVE_LOW,
}
impl SS_ACT_LOR {
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
            SS_ACT_LOR::ACTIVE_HIGH => false,
            SS_ACT_LOR::ACTIVE_LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SS_ACT_LOR {
        match value {
            false => SS_ACT_LOR::ACTIVE_HIGH,
            true => SS_ACT_LOR::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline]
    pub fn is_active_high(&self) -> bool {
        *self == SS_ACT_LOR::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline]
    pub fn is_active_low(&self) -> bool {
        *self == SS_ACT_LOR::ACTIVE_LOW
    }
}
#[doc = "Possible values of the field `alt_timing_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALT_TIMING_ENR {
    #[doc = "Alternate timing is disabled."]
    DISABLED,
    #[doc = "Alternate timing will be enabled automatically when needed."]
    ENABLED_AS_NEEDED,
}
impl ALT_TIMING_ENR {
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
            ALT_TIMING_ENR::DISABLED => false,
            ALT_TIMING_ENR::ENABLED_AS_NEEDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALT_TIMING_ENR {
        match value {
            false => ALT_TIMING_ENR::DISABLED,
            true => ALT_TIMING_ENR::ENABLED_AS_NEEDED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ALT_TIMING_ENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED_AS_NEEDED`"]
    #[inline]
    pub fn is_enabled_as_needed(&self) -> bool {
        *self == ALT_TIMING_ENR::ENABLED_AS_NEEDED
    }
}
#[doc = "Possible values of the field `slave_sel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE_SELR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLAVE_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLAVE_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLAVE_SELR {
        match value {
            i => SLAVE_SELR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `sck_hi_clk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCK_HI_CLKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCK_HI_CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCK_HI_CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCK_HI_CLKR {
        match value {
            i => SCK_HI_CLKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `sck_lo_clk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCK_LO_CLKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCK_LO_CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCK_LO_CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCK_LO_CLKR {
        match value {
            i => SCK_LO_CLKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `act_delay`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACT_DELAYR {
    #[doc = "No SS Active timing delay enabled."]
    OFF,
    #[doc = "SS Active timing delay of 2 SPIX module clock cycles."]
    FOR_2_MOD_CLK,
    #[doc = "SS Active timing delay of 4 SPIX module clock cycles."]
    FOR_4_MOD_CLK,
    #[doc = "SS Active timing delay of 8 SPIX module clock cycles."]
    FOR_8_MOD_CLK,
}
impl ACT_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACT_DELAYR::OFF => 0,
            ACT_DELAYR::FOR_2_MOD_CLK => 1,
            ACT_DELAYR::FOR_4_MOD_CLK => 2,
            ACT_DELAYR::FOR_8_MOD_CLK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACT_DELAYR {
        match value {
            0 => ACT_DELAYR::OFF,
            1 => ACT_DELAYR::FOR_2_MOD_CLK,
            2 => ACT_DELAYR::FOR_4_MOD_CLK,
            3 => ACT_DELAYR::FOR_8_MOD_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == ACT_DELAYR::OFF
    }
    #[doc = "Checks if the value of the field is `FOR_2_MOD_CLK`"]
    #[inline]
    pub fn is_for_2_mod_clk(&self) -> bool {
        *self == ACT_DELAYR::FOR_2_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_4_MOD_CLK`"]
    #[inline]
    pub fn is_for_4_mod_clk(&self) -> bool {
        *self == ACT_DELAYR::FOR_4_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_8_MOD_CLK`"]
    #[inline]
    pub fn is_for_8_mod_clk(&self) -> bool {
        *self == ACT_DELAYR::FOR_8_MOD_CLK
    }
}
#[doc = "Possible values of the field `inact_delay`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INACT_DELAYR {
    #[doc = "No SS Active timing delay enabled."]
    OFF,
    #[doc = "SS Active timing delay of 2 SPIX module clock cycles."]
    FOR_2_MOD_CLK,
    #[doc = "SS Active timing delay of 4 SPIX module clock cycles."]
    FOR_4_MOD_CLK,
    #[doc = "SS Active timing delay of 8 SPIX module clock cycles."]
    FOR_8_MOD_CLK,
}
impl INACT_DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INACT_DELAYR::OFF => 0,
            INACT_DELAYR::FOR_2_MOD_CLK => 1,
            INACT_DELAYR::FOR_4_MOD_CLK => 2,
            INACT_DELAYR::FOR_8_MOD_CLK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INACT_DELAYR {
        match value {
            0 => INACT_DELAYR::OFF,
            1 => INACT_DELAYR::FOR_2_MOD_CLK,
            2 => INACT_DELAYR::FOR_4_MOD_CLK,
            3 => INACT_DELAYR::FOR_8_MOD_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == INACT_DELAYR::OFF
    }
    #[doc = "Checks if the value of the field is `FOR_2_MOD_CLK`"]
    #[inline]
    pub fn is_for_2_mod_clk(&self) -> bool {
        *self == INACT_DELAYR::FOR_2_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_4_MOD_CLK`"]
    #[inline]
    pub fn is_for_4_mod_clk(&self) -> bool {
        *self == INACT_DELAYR::FOR_4_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_8_MOD_CLK`"]
    #[inline]
    pub fn is_for_8_mod_clk(&self) -> bool {
        *self == INACT_DELAYR::FOR_8_MOD_CLK
    }
}
#[doc = "Possible values of the field `alt_sck_hi_clk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALT_SCK_HI_CLKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALT_SCK_HI_CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALT_SCK_HI_CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALT_SCK_HI_CLKR {
        match value {
            i => ALT_SCK_HI_CLKR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `alt_sck_lo_clk`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALT_SCK_LO_CLKR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ALT_SCK_LO_CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ALT_SCK_LO_CLKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ALT_SCK_LO_CLKR {
        match value {
            i => ALT_SCK_LO_CLKR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `spi_mode`"]
pub enum SPI_MODEW {
    #[doc = "SCK is active high, data is sampled on clock rising edge."]
    SCK_HI_SAMPLE_RISING,
    #[doc = "SCK is active low, data is sampled on clock rising edge."]
    SCK_LO_SAMPLE_FALLING,
}
impl SPI_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPI_MODEW::SCK_HI_SAMPLE_RISING => 0,
            SPI_MODEW::SCK_LO_SAMPLE_FALLING => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPI_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SPI_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPI_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SCK is active high, data is sampled on clock rising edge."]
    #[inline]
    pub fn sck_hi_sample_rising(self) -> &'a mut W {
        self.variant(SPI_MODEW::SCK_HI_SAMPLE_RISING)
    }
    #[doc = "SCK is active low, data is sampled on clock rising edge."]
    #[inline]
    pub fn sck_lo_sample_falling(self) -> &'a mut W {
        self.variant(SPI_MODEW::SCK_LO_SAMPLE_FALLING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ss_act_lo`"]
pub enum SS_ACT_LOW {
    #[doc = "Enabled slave select (SS) is active high."]
    ACTIVE_HIGH,
    #[doc = "Enabled slave select (SS) is active low."]
    ACTIVE_LOW,
}
impl SS_ACT_LOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SS_ACT_LOW::ACTIVE_HIGH => false,
            SS_ACT_LOW::ACTIVE_LOW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SS_ACT_LOW<'a> {
    w: &'a mut W,
}
impl<'a> _SS_ACT_LOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS_ACT_LOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled slave select (SS) is active high."]
    #[inline]
    pub fn active_high(self) -> &'a mut W {
        self.variant(SS_ACT_LOW::ACTIVE_HIGH)
    }
    #[doc = "Enabled slave select (SS) is active low."]
    #[inline]
    pub fn active_low(self) -> &'a mut W {
        self.variant(SS_ACT_LOW::ACTIVE_LOW)
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
#[doc = "Values that can be written to the field `alt_timing_en`"]
pub enum ALT_TIMING_ENW {
    #[doc = "Alternate timing is disabled."]
    DISABLED,
    #[doc = "Alternate timing will be enabled automatically when needed."]
    ENABLED_AS_NEEDED,
}
impl ALT_TIMING_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALT_TIMING_ENW::DISABLED => false,
            ALT_TIMING_ENW::ENABLED_AS_NEEDED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALT_TIMING_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ALT_TIMING_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALT_TIMING_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Alternate timing is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALT_TIMING_ENW::DISABLED)
    }
    #[doc = "Alternate timing will be enabled automatically when needed."]
    #[inline]
    pub fn enabled_as_needed(self) -> &'a mut W {
        self.variant(ALT_TIMING_ENW::ENABLED_AS_NEEDED)
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
#[doc = "Values that can be written to the field `slave_sel`"]
pub enum SLAVE_SELW {}
impl SLAVE_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SLAVE_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SLAVE_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLAVE_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `sck_hi_clk`"]
pub enum SCK_HI_CLKW {}
impl SCK_HI_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SCK_HI_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SCK_HI_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCK_HI_CLKW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `sck_lo_clk`"]
pub enum SCK_LO_CLKW {}
impl SCK_LO_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SCK_LO_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SCK_LO_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCK_LO_CLKW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `act_delay`"]
pub enum ACT_DELAYW {
    #[doc = "No SS Active timing delay enabled."]
    OFF,
    #[doc = "SS Active timing delay of 2 SPIX module clock cycles."]
    FOR_2_MOD_CLK,
    #[doc = "SS Active timing delay of 4 SPIX module clock cycles."]
    FOR_4_MOD_CLK,
    #[doc = "SS Active timing delay of 8 SPIX module clock cycles."]
    FOR_8_MOD_CLK,
}
impl ACT_DELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACT_DELAYW::OFF => 0,
            ACT_DELAYW::FOR_2_MOD_CLK => 1,
            ACT_DELAYW::FOR_4_MOD_CLK => 2,
            ACT_DELAYW::FOR_8_MOD_CLK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACT_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _ACT_DELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACT_DELAYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No SS Active timing delay enabled."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(ACT_DELAYW::OFF)
    }
    #[doc = "SS Active timing delay of 2 SPIX module clock cycles."]
    #[inline]
    pub fn for_2_mod_clk(self) -> &'a mut W {
        self.variant(ACT_DELAYW::FOR_2_MOD_CLK)
    }
    #[doc = "SS Active timing delay of 4 SPIX module clock cycles."]
    #[inline]
    pub fn for_4_mod_clk(self) -> &'a mut W {
        self.variant(ACT_DELAYW::FOR_4_MOD_CLK)
    }
    #[doc = "SS Active timing delay of 8 SPIX module clock cycles."]
    #[inline]
    pub fn for_8_mod_clk(self) -> &'a mut W {
        self.variant(ACT_DELAYW::FOR_8_MOD_CLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `inact_delay`"]
pub enum INACT_DELAYW {
    #[doc = "No SS Active timing delay enabled."]
    OFF,
    #[doc = "SS Active timing delay of 2 SPIX module clock cycles."]
    FOR_2_MOD_CLK,
    #[doc = "SS Active timing delay of 4 SPIX module clock cycles."]
    FOR_4_MOD_CLK,
    #[doc = "SS Active timing delay of 8 SPIX module clock cycles."]
    FOR_8_MOD_CLK,
}
impl INACT_DELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INACT_DELAYW::OFF => 0,
            INACT_DELAYW::FOR_2_MOD_CLK => 1,
            INACT_DELAYW::FOR_4_MOD_CLK => 2,
            INACT_DELAYW::FOR_8_MOD_CLK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INACT_DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _INACT_DELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INACT_DELAYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No SS Active timing delay enabled."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(INACT_DELAYW::OFF)
    }
    #[doc = "SS Active timing delay of 2 SPIX module clock cycles."]
    #[inline]
    pub fn for_2_mod_clk(self) -> &'a mut W {
        self.variant(INACT_DELAYW::FOR_2_MOD_CLK)
    }
    #[doc = "SS Active timing delay of 4 SPIX module clock cycles."]
    #[inline]
    pub fn for_4_mod_clk(self) -> &'a mut W {
        self.variant(INACT_DELAYW::FOR_4_MOD_CLK)
    }
    #[doc = "SS Active timing delay of 8 SPIX module clock cycles."]
    #[inline]
    pub fn for_8_mod_clk(self) -> &'a mut W {
        self.variant(INACT_DELAYW::FOR_8_MOD_CLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `alt_sck_hi_clk`"]
pub enum ALT_SCK_HI_CLKW {}
impl ALT_SCK_HI_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ALT_SCK_HI_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ALT_SCK_HI_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALT_SCK_HI_CLKW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `alt_sck_lo_clk`"]
pub enum ALT_SCK_LO_CLKW {}
impl ALT_SCK_LO_CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ALT_SCK_LO_CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ALT_SCK_LO_CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALT_SCK_LO_CLKW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - SPIX Mode"]
    #[inline]
    pub fn spi_mode(&self) -> SPI_MODER {
        SPI_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - SPIX Slave Select Polarity"]
    #[inline]
    pub fn ss_act_lo(&self) -> SS_ACT_LOR {
        SS_ACT_LOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Alternate Timing Mode Enable"]
    #[inline]
    pub fn alt_timing_en(&self) -> ALT_TIMING_ENR {
        ALT_TIMING_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - SPIX Slave Select"]
    #[inline]
    pub fn slave_sel(&self) -> SLAVE_SELR {
        SLAVE_SELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - SCK High Clocks"]
    #[inline]
    pub fn sck_hi_clk(&self) -> SCK_HI_CLKR {
        SCK_HI_CLKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - SCK Low Clocks"]
    #[inline]
    pub fn sck_lo_clk(&self) -> SCK_LO_CLKR {
        SCK_LO_CLKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - SS Active Timing"]
    #[inline]
    pub fn act_delay(&self) -> ACT_DELAYR {
        ACT_DELAYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - SS Inactive Timing"]
    #[inline]
    pub fn inact_delay(&self) -> INACT_DELAYR {
        INACT_DELAYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Alt SCK High Clocks"]
    #[inline]
    pub fn alt_sck_hi_clk(&self) -> ALT_SCK_HI_CLKR {
        ALT_SCK_HI_CLKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Alt SCK Low Clocks"]
    #[inline]
    pub fn alt_sck_lo_clk(&self) -> ALT_SCK_LO_CLKR {
        ALT_SCK_LO_CLKR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:1 - SPIX Mode"]
    #[inline]
    pub fn spi_mode(&mut self) -> _SPI_MODEW {
        _SPI_MODEW { w: self }
    }
    #[doc = "Bit 2 - SPIX Slave Select Polarity"]
    #[inline]
    pub fn ss_act_lo(&mut self) -> _SS_ACT_LOW {
        _SS_ACT_LOW { w: self }
    }
    #[doc = "Bit 3 - Alternate Timing Mode Enable"]
    #[inline]
    pub fn alt_timing_en(&mut self) -> _ALT_TIMING_ENW {
        _ALT_TIMING_ENW { w: self }
    }
    #[doc = "Bits 4:6 - SPIX Slave Select"]
    #[inline]
    pub fn slave_sel(&mut self) -> _SLAVE_SELW {
        _SLAVE_SELW { w: self }
    }
    #[doc = "Bits 8:11 - SCK High Clocks"]
    #[inline]
    pub fn sck_hi_clk(&mut self) -> _SCK_HI_CLKW {
        _SCK_HI_CLKW { w: self }
    }
    #[doc = "Bits 12:15 - SCK Low Clocks"]
    #[inline]
    pub fn sck_lo_clk(&mut self) -> _SCK_LO_CLKW {
        _SCK_LO_CLKW { w: self }
    }
    #[doc = "Bits 16:17 - SS Active Timing"]
    #[inline]
    pub fn act_delay(&mut self) -> _ACT_DELAYW {
        _ACT_DELAYW { w: self }
    }
    #[doc = "Bits 18:19 - SS Inactive Timing"]
    #[inline]
    pub fn inact_delay(&mut self) -> _INACT_DELAYW {
        _INACT_DELAYW { w: self }
    }
    #[doc = "Bits 20:23 - Alt SCK High Clocks"]
    #[inline]
    pub fn alt_sck_hi_clk(&mut self) -> _ALT_SCK_HI_CLKW {
        _ALT_SCK_HI_CLKW { w: self }
    }
    #[doc = "Bits 24:27 - Alt SCK Low Clocks"]
    #[inline]
    pub fn alt_sck_lo_clk(&mut self) -> _ALT_SCK_LO_CLKW {
        _ALT_SCK_LO_CLKW { w: self }
    }
}
