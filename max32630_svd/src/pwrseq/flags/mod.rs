#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLAGS {
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
#[doc = "Possible values of the field `pwr_first_boot`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_FIRST_BOOTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_FIRST_BOOTR {
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
            PWR_FIRST_BOOTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_FIRST_BOOTR {
        match value {
            i => PWR_FIRST_BOOTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_sys_reboot`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_SYS_REBOOTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_SYS_REBOOTR {
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
            PWR_SYS_REBOOTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_SYS_REBOOTR {
        match value {
            i => PWR_SYS_REBOOTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_power_fail`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_POWER_FAILR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_POWER_FAILR {
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
            PWR_POWER_FAILR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_POWER_FAILR {
        match value {
            i => PWR_POWER_FAILR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_boot_fail`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_BOOT_FAILR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_BOOT_FAILR {
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
            PWR_BOOT_FAILR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_BOOT_FAILR {
        match value {
            i => PWR_BOOT_FAILR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_flash_discharge`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_FLASH_DISCHARGER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_FLASH_DISCHARGER {
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
            PWR_FLASH_DISCHARGER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_FLASH_DISCHARGER {
        match value {
            i => PWR_FLASH_DISCHARGER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_iowakeup`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_IOWAKEUPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_IOWAKEUPR {
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
            PWR_IOWAKEUPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_IOWAKEUPR {
        match value {
            i => PWR_IOWAKEUPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_vdd12_rst_bad`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VDD12_RST_BADR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_VDD12_RST_BADR {
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
            PWR_VDD12_RST_BADR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_VDD12_RST_BADR {
        match value {
            i => PWR_VDD12_RST_BADR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_vdd18_rst_bad`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VDD18_RST_BADR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_VDD18_RST_BADR {
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
            PWR_VDD18_RST_BADR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_VDD18_RST_BADR {
        match value {
            i => PWR_VDD18_RST_BADR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_vrtc_rst_bad`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VRTC_RST_BADR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_VRTC_RST_BADR {
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
            PWR_VRTC_RST_BADR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_VRTC_RST_BADR {
        match value {
            i => PWR_VRTC_RST_BADR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_vddb_rst_bad`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VDDB_RST_BADR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_VDDB_RST_BADR {
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
            PWR_VDDB_RST_BADR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_VDDB_RST_BADR {
        match value {
            i => PWR_VDDB_RST_BADR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_tvdd12_rst_bad`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TVDD12_RST_BADR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_TVDD12_RST_BADR {
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
            PWR_TVDD12_RST_BADR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_TVDD12_RST_BADR {
        match value {
            i => PWR_TVDD12_RST_BADR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_por18z_fail_latch`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_POR18Z_FAIL_LATCHR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_POR18Z_FAIL_LATCHR {
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
            PWR_POR18Z_FAIL_LATCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_POR18Z_FAIL_LATCHR {
        match value {
            i => PWR_POR18Z_FAIL_LATCHR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rtc_cmpr0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CMPR0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTC_CMPR0R {
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
            RTC_CMPR0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_CMPR0R {
        match value {
            i => RTC_CMPR0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rtc_cmpr1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_CMPR1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTC_CMPR1R {
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
            RTC_CMPR1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_CMPR1R {
        match value {
            i => RTC_CMPR1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rtc_prescale_cmp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_PRESCALE_CMPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTC_PRESCALE_CMPR {
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
            RTC_PRESCALE_CMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_PRESCALE_CMPR {
        match value {
            i => RTC_PRESCALE_CMPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `rtc_rollover`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTC_ROLLOVERR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl RTC_ROLLOVERR {
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
            RTC_ROLLOVERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTC_ROLLOVERR {
        match value {
            i => RTC_ROLLOVERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_usb_plug_wakeup`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_USB_PLUG_WAKEUPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_USB_PLUG_WAKEUPR {
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
            PWR_USB_PLUG_WAKEUPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_USB_PLUG_WAKEUPR {
        match value {
            i => PWR_USB_PLUG_WAKEUPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_usb_remove_wakeup`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_USB_REMOVE_WAKEUPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_USB_REMOVE_WAKEUPR {
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
            PWR_USB_REMOVE_WAKEUPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_USB_REMOVE_WAKEUPR {
        match value {
            i => PWR_USB_REMOVE_WAKEUPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_tvdd12_bad`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TVDD12_BADR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_TVDD12_BADR {
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
            PWR_TVDD12_BADR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_TVDD12_BADR {
        match value {
            i => PWR_TVDD12_BADR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `pwr_power_fail`"]
pub enum PWR_POWER_FAILW {}
impl PWR_POWER_FAILW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_POWER_FAILW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_POWER_FAILW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_POWER_FAILW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_boot_fail`"]
pub enum PWR_BOOT_FAILW {}
impl PWR_BOOT_FAILW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_BOOT_FAILW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_BOOT_FAILW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_BOOT_FAILW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_flash_discharge`"]
pub enum PWR_FLASH_DISCHARGEW {}
impl PWR_FLASH_DISCHARGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_FLASH_DISCHARGEW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_FLASH_DISCHARGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_FLASH_DISCHARGEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_iowakeup`"]
pub enum PWR_IOWAKEUPW {}
impl PWR_IOWAKEUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_IOWAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_IOWAKEUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_IOWAKEUPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_vdd12_rst_bad`"]
pub enum PWR_VDD12_RST_BADW {}
impl PWR_VDD12_RST_BADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VDD12_RST_BADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VDD12_RST_BADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VDD12_RST_BADW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_vdd18_rst_bad`"]
pub enum PWR_VDD18_RST_BADW {}
impl PWR_VDD18_RST_BADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VDD18_RST_BADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VDD18_RST_BADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VDD18_RST_BADW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_vrtc_rst_bad`"]
pub enum PWR_VRTC_RST_BADW {}
impl PWR_VRTC_RST_BADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VRTC_RST_BADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VRTC_RST_BADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VRTC_RST_BADW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_vddb_rst_bad`"]
pub enum PWR_VDDB_RST_BADW {}
impl PWR_VDDB_RST_BADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VDDB_RST_BADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VDDB_RST_BADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VDDB_RST_BADW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_tvdd12_rst_bad`"]
pub enum PWR_TVDD12_RST_BADW {}
impl PWR_TVDD12_RST_BADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TVDD12_RST_BADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TVDD12_RST_BADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TVDD12_RST_BADW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_por18z_fail_latch`"]
pub enum PWR_POR18Z_FAIL_LATCHW {}
impl PWR_POR18Z_FAIL_LATCHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_POR18Z_FAIL_LATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_POR18Z_FAIL_LATCHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_POR18Z_FAIL_LATCHW) -> &'a mut W {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_usb_plug_wakeup`"]
pub enum PWR_USB_PLUG_WAKEUPW {}
impl PWR_USB_PLUG_WAKEUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_USB_PLUG_WAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_USB_PLUG_WAKEUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_USB_PLUG_WAKEUPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_usb_remove_wakeup`"]
pub enum PWR_USB_REMOVE_WAKEUPW {}
impl PWR_USB_REMOVE_WAKEUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_USB_REMOVE_WAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_USB_REMOVE_WAKEUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_USB_REMOVE_WAKEUPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_tvdd12_bad`"]
pub enum PWR_TVDD12_BADW {}
impl PWR_TVDD12_BADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TVDD12_BADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TVDD12_BADW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TVDD12_BADW) -> &'a mut W {
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Initial Boot event detected flag"]
    #[inline]
    pub fn pwr_first_boot(&self) -> PWR_FIRST_BOOTR {
        PWR_FIRST_BOOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Firmware Reset event detected flag"]
    #[inline]
    pub fn pwr_sys_reboot(&self) -> PWR_SYS_REBOOTR {
        PWR_SYS_REBOOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Power Fail event detected flag"]
    #[inline]
    pub fn pwr_power_fail(&self) -> PWR_POWER_FAILR {
        PWR_POWER_FAILR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Boot Fail event detected flag"]
    #[inline]
    pub fn pwr_boot_fail(&self) -> PWR_BOOT_FAILR {
        PWR_BOOT_FAILR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Flash Discharged During Powerfail event detected flag"]
    #[inline]
    pub fn pwr_flash_discharge(&self) -> PWR_FLASH_DISCHARGER {
        PWR_FLASH_DISCHARGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - GPIO Wakeup event detected flag"]
    #[inline]
    pub fn pwr_iowakeup(&self) -> PWR_IOWAKEUPR {
        PWR_IOWAKEUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - VDD12_SW Comparator Tripped event detected flag"]
    #[inline]
    pub fn pwr_vdd12_rst_bad(&self) -> PWR_VDD12_RST_BADR {
        PWR_VDD12_RST_BADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - VDD18_SW Comparator Tripped event detected flag"]
    #[inline]
    pub fn pwr_vdd18_rst_bad(&self) -> PWR_VDD18_RST_BADR {
        PWR_VDD18_RST_BADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - VRTC Comparator Tripped event detected flag"]
    #[inline]
    pub fn pwr_vrtc_rst_bad(&self) -> PWR_VRTC_RST_BADR {
        PWR_VRTC_RST_BADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - VDDB Comparator Tripped event detected flag"]
    #[inline]
    pub fn pwr_vddb_rst_bad(&self) -> PWR_VDDB_RST_BADR {
        PWR_VDDB_RST_BADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - TVDD12 Comparator Tripped event detected flag"]
    #[inline]
    pub fn pwr_tvdd12_rst_bad(&self) -> PWR_TVDD12_RST_BADR {
        PWR_TVDD12_RST_BADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - POR18 and POR18_bg have been tripped"]
    #[inline]
    pub fn pwr_por18z_fail_latch(&self) -> PWR_POR18Z_FAIL_LATCHR {
        PWR_POR18Z_FAIL_LATCHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - RTC Comparator 0 Match event detected flag"]
    #[inline]
    pub fn rtc_cmpr0(&self) -> RTC_CMPR0R {
        RTC_CMPR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - RTC Comparator 1 Match event detected flag"]
    #[inline]
    pub fn rtc_cmpr1(&self) -> RTC_CMPR1R {
        RTC_CMPR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - RTC Prescale Comparator Match event detected flag"]
    #[inline]
    pub fn rtc_prescale_cmp(&self) -> RTC_PRESCALE_CMPR {
        RTC_PRESCALE_CMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - RTC Rollover event detected flag"]
    #[inline]
    pub fn rtc_rollover(&self) -> RTC_ROLLOVERR {
        RTC_ROLLOVERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - USB Power Connect Wakeup event detected flag"]
    #[inline]
    pub fn pwr_usb_plug_wakeup(&self) -> PWR_USB_PLUG_WAKEUPR {
        PWR_USB_PLUG_WAKEUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - USB Power Remove Wakeup event detected flag"]
    #[inline]
    pub fn pwr_usb_remove_wakeup(&self) -> PWR_USB_REMOVE_WAKEUPR {
        PWR_USB_REMOVE_WAKEUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Retention Regulator POR Tripped event detected flag"]
    #[inline]
    pub fn pwr_tvdd12_bad(&self) -> PWR_TVDD12_BADR {
        PWR_TVDD12_BADR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
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
    #[doc = "Bit 2 - Power Fail event detected flag"]
    #[inline]
    pub fn pwr_power_fail(&mut self) -> _PWR_POWER_FAILW {
        _PWR_POWER_FAILW { w: self }
    }
    #[doc = "Bit 3 - Boot Fail event detected flag"]
    #[inline]
    pub fn pwr_boot_fail(&mut self) -> _PWR_BOOT_FAILW {
        _PWR_BOOT_FAILW { w: self }
    }
    #[doc = "Bit 4 - Flash Discharged During Powerfail event detected flag"]
    #[inline]
    pub fn pwr_flash_discharge(&mut self) -> _PWR_FLASH_DISCHARGEW {
        _PWR_FLASH_DISCHARGEW { w: self }
    }
    #[doc = "Bit 5 - GPIO Wakeup event detected flag"]
    #[inline]
    pub fn pwr_iowakeup(&mut self) -> _PWR_IOWAKEUPW {
        _PWR_IOWAKEUPW { w: self }
    }
    #[doc = "Bit 6 - VDD12_SW Comparator Tripped event detected flag"]
    #[inline]
    pub fn pwr_vdd12_rst_bad(&mut self) -> _PWR_VDD12_RST_BADW {
        _PWR_VDD12_RST_BADW { w: self }
    }
    #[doc = "Bit 7 - VDD18_SW Comparator Tripped event detected flag"]
    #[inline]
    pub fn pwr_vdd18_rst_bad(&mut self) -> _PWR_VDD18_RST_BADW {
        _PWR_VDD18_RST_BADW { w: self }
    }
    #[doc = "Bit 8 - VRTC Comparator Tripped event detected flag"]
    #[inline]
    pub fn pwr_vrtc_rst_bad(&mut self) -> _PWR_VRTC_RST_BADW {
        _PWR_VRTC_RST_BADW { w: self }
    }
    #[doc = "Bit 9 - VDDB Comparator Tripped event detected flag"]
    #[inline]
    pub fn pwr_vddb_rst_bad(&mut self) -> _PWR_VDDB_RST_BADW {
        _PWR_VDDB_RST_BADW { w: self }
    }
    #[doc = "Bit 10 - TVDD12 Comparator Tripped event detected flag"]
    #[inline]
    pub fn pwr_tvdd12_rst_bad(&mut self) -> _PWR_TVDD12_RST_BADW {
        _PWR_TVDD12_RST_BADW { w: self }
    }
    #[doc = "Bit 11 - POR18 and POR18_bg have been tripped"]
    #[inline]
    pub fn pwr_por18z_fail_latch(&mut self) -> _PWR_POR18Z_FAIL_LATCHW {
        _PWR_POR18Z_FAIL_LATCHW { w: self }
    }
    #[doc = "Bit 16 - USB Power Connect Wakeup event detected flag"]
    #[inline]
    pub fn pwr_usb_plug_wakeup(&mut self) -> _PWR_USB_PLUG_WAKEUPW {
        _PWR_USB_PLUG_WAKEUPW { w: self }
    }
    #[doc = "Bit 17 - USB Power Remove Wakeup event detected flag"]
    #[inline]
    pub fn pwr_usb_remove_wakeup(&mut self) -> _PWR_USB_REMOVE_WAKEUPW {
        _PWR_USB_REMOVE_WAKEUPW { w: self }
    }
    #[doc = "Bit 18 - Retention Regulator POR Tripped event detected flag"]
    #[inline]
    pub fn pwr_tvdd12_bad(&mut self) -> _PWR_TVDD12_BADW {
        _PWR_TVDD12_BADW { w: self }
    }
}
