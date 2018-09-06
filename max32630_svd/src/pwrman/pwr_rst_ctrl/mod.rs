#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWR_RST_CTRL {
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
#[doc = "Possible values of the field `afe_powered`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFE_POWEREDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl AFE_POWEREDR {
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
            AFE_POWEREDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AFE_POWEREDR {
        match value {
            i => AFE_POWEREDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `io_active`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_ACTIVER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IO_ACTIVER {
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
            IO_ACTIVER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IO_ACTIVER {
        match value {
            i => IO_ACTIVER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `usb_powered`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_POWEREDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl USB_POWEREDR {
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
            USB_POWEREDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USB_POWEREDR {
        match value {
            i => USB_POWEREDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pullups_enabled`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULLUPS_ENABLEDR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PULLUPS_ENABLEDR {
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
            PULLUPS_ENABLEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PULLUPS_ENABLEDR {
        match value {
            i => PULLUPS_ENABLEDR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `firmware_reset`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIRMWARE_RESETR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FIRMWARE_RESETR {
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
            FIRMWARE_RESETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIRMWARE_RESETR {
        match value {
            i => FIRMWARE_RESETR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `arm_lockup_reset`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARM_LOCKUP_RESETR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ARM_LOCKUP_RESETR {
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
            ARM_LOCKUP_RESETR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARM_LOCKUP_RESETR {
        match value {
            i => ARM_LOCKUP_RESETR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `tamper_detect`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAMPER_DETECTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl TAMPER_DETECTR {
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
            TAMPER_DETECTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAMPER_DETECTR {
        match value {
            i => TAMPER_DETECTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `fw_command_sysman`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FW_COMMAND_SYSMANR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FW_COMMAND_SYSMANR {
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
            FW_COMMAND_SYSMANR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FW_COMMAND_SYSMANR {
        match value {
            i => FW_COMMAND_SYSMANR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `watchdog_timeout`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WATCHDOG_TIMEOUTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl WATCHDOG_TIMEOUTR {
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
            WATCHDOG_TIMEOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WATCHDOG_TIMEOUTR {
        match value {
            i => WATCHDOG_TIMEOUTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `fw_command_arm`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FW_COMMAND_ARMR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FW_COMMAND_ARMR {
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
            FW_COMMAND_ARMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FW_COMMAND_ARMR {
        match value {
            i => FW_COMMAND_ARMR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `arm_lockup`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARM_LOCKUPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl ARM_LOCKUPR {
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
            ARM_LOCKUPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARM_LOCKUPR {
        match value {
            i => ARM_LOCKUPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `srstn_assertion`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRSTN_ASSERTIONR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SRSTN_ASSERTIONR {
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
            SRSTN_ASSERTIONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRSTN_ASSERTIONR {
        match value {
            i => SRSTN_ASSERTIONR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `por`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PORR {
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
            PORR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PORR {
        match value {
            i => PORR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `low_power_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOW_POWER_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl LOW_POWER_MODER {
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
            LOW_POWER_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOW_POWER_MODER {
        match value {
            i => LOW_POWER_MODER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `afe_powered`"]
pub enum AFE_POWEREDW {}
impl AFE_POWEREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _AFE_POWEREDW<'a> {
    w: &'a mut W,
}
impl<'a> _AFE_POWEREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AFE_POWEREDW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `io_active`"]
pub enum IO_ACTIVEW {}
impl IO_ACTIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _IO_ACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _IO_ACTIVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IO_ACTIVEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `usb_powered`"]
pub enum USB_POWEREDW {}
impl USB_POWEREDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _USB_POWEREDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_POWEREDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USB_POWEREDW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pullups_enabled`"]
pub enum PULLUPS_ENABLEDW {}
impl PULLUPS_ENABLEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PULLUPS_ENABLEDW<'a> {
    w: &'a mut W,
}
impl<'a> _PULLUPS_ENABLEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PULLUPS_ENABLEDW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `firmware_reset`"]
pub enum FIRMWARE_RESETW {}
impl FIRMWARE_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FIRMWARE_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _FIRMWARE_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIRMWARE_RESETW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `arm_lockup_reset`"]
pub enum ARM_LOCKUP_RESETW {}
impl ARM_LOCKUP_RESETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _ARM_LOCKUP_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _ARM_LOCKUP_RESETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARM_LOCKUP_RESETW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `low_power_mode`"]
pub enum LOW_POWER_MODEW {}
impl LOW_POWER_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _LOW_POWER_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _LOW_POWER_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOW_POWER_MODEW) -> &'a mut W {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 2 - AFE Powered"]
    #[inline]
    pub fn afe_powered(&self) -> AFE_POWEREDR {
        AFE_POWEREDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - I/O Active"]
    #[inline]
    pub fn io_active(&self) -> IO_ACTIVER {
        IO_ACTIVER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - USB Powered"]
    #[inline]
    pub fn usb_powered(&self) -> USB_POWEREDR {
        USB_POWEREDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Static Pullups Enabled"]
    #[inline]
    pub fn pullups_enabled(&self) -> PULLUPS_ENABLEDR {
        PULLUPS_ENABLEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Firmware Initiated Reset"]
    #[inline]
    pub fn firmware_reset(&self) -> FIRMWARE_RESETR {
        FIRMWARE_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - ARM Lockup Reset"]
    #[inline]
    pub fn arm_lockup_reset(&self) -> ARM_LOCKUP_RESETR {
        ARM_LOCKUP_RESETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Reset Caused By - Tamper Detect"]
    #[inline]
    pub fn tamper_detect(&self) -> TAMPER_DETECTR {
        TAMPER_DETECTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Reset Caused By - Firmware Commanded Reset (SysMan)"]
    #[inline]
    pub fn fw_command_sysman(&self) -> FW_COMMAND_SYSMANR {
        FW_COMMAND_SYSMANR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Reset Caused By - Watchdog Timeout"]
    #[inline]
    pub fn watchdog_timeout(&self) -> WATCHDOG_TIMEOUTR {
        WATCHDOG_TIMEOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Reset Caused By - Firmware Commanded Reset (ARM Core)"]
    #[inline]
    pub fn fw_command_arm(&self) -> FW_COMMAND_ARMR {
        FW_COMMAND_ARMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Reset Caused By - ARM Lockup"]
    #[inline]
    pub fn arm_lockup(&self) -> ARM_LOCKUPR {
        ARM_LOCKUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Reset Caused By - External System Reset"]
    #[inline]
    pub fn srstn_assertion(&self) -> SRSTN_ASSERTIONR {
        SRSTN_ASSERTIONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Reset Caused By - Power On Reset (POR)"]
    #[inline]
    pub fn por(&self) -> PORR {
        PORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Power Manager Dynamic Clock Gating Enable"]
    #[inline]
    pub fn low_power_mode(&self) -> LOW_POWER_MODER {
        LOW_POWER_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 2 - AFE Powered"]
    #[inline]
    pub fn afe_powered(&mut self) -> _AFE_POWEREDW {
        _AFE_POWEREDW { w: self }
    }
    #[doc = "Bit 3 - I/O Active"]
    #[inline]
    pub fn io_active(&mut self) -> _IO_ACTIVEW {
        _IO_ACTIVEW { w: self }
    }
    #[doc = "Bit 4 - USB Powered"]
    #[inline]
    pub fn usb_powered(&mut self) -> _USB_POWEREDW {
        _USB_POWEREDW { w: self }
    }
    #[doc = "Bit 5 - Static Pullups Enabled"]
    #[inline]
    pub fn pullups_enabled(&mut self) -> _PULLUPS_ENABLEDW {
        _PULLUPS_ENABLEDW { w: self }
    }
    #[doc = "Bit 8 - Firmware Initiated Reset"]
    #[inline]
    pub fn firmware_reset(&mut self) -> _FIRMWARE_RESETW {
        _FIRMWARE_RESETW { w: self }
    }
    #[doc = "Bit 9 - ARM Lockup Reset"]
    #[inline]
    pub fn arm_lockup_reset(&mut self) -> _ARM_LOCKUP_RESETW {
        _ARM_LOCKUP_RESETW { w: self }
    }
    #[doc = "Bit 31 - Power Manager Dynamic Clock Gating Enable"]
    #[inline]
    pub fn low_power_mode(&mut self) -> _LOW_POWER_MODEW {
        _LOW_POWER_MODEW { w: self }
    }
}
