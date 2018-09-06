#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PERFORM {
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
#[doc = "Possible values of the field `delay_se_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DELAY_SE_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl DELAY_SE_ENR {
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
            DELAY_SE_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DELAY_SE_ENR {
        match value {
            i => DELAY_SE_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `fast_read_mode_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAST_READ_MODE_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl FAST_READ_MODE_ENR {
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
            FAST_READ_MODE_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FAST_READ_MODE_ENR {
        match value {
            i => FAST_READ_MODE_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `en_prevent_fail`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_PREVENT_FAILR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EN_PREVENT_FAILR {
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
            EN_PREVENT_FAILR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_PREVENT_FAILR {
        match value {
            i => EN_PREVENT_FAILR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `en_back2back_rds`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_BACK2BACK_RDSR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EN_BACK2BACK_RDSR {
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
            EN_BACK2BACK_RDSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_BACK2BACK_RDSR {
        match value {
            i => EN_BACK2BACK_RDSR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `en_back2back_wrs`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_BACK2BACK_WRSR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EN_BACK2BACK_WRSR {
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
            EN_BACK2BACK_WRSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_BACK2BACK_WRSR {
        match value {
            i => EN_BACK2BACK_WRSR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `en_merge_grab_gnt`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_MERGE_GRAB_GNTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EN_MERGE_GRAB_GNTR {
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
            EN_MERGE_GRAB_GNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_MERGE_GRAB_GNTR {
        match value {
            i => EN_MERGE_GRAB_GNTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `auto_tacc`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_TACCR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl AUTO_TACCR {
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
            AUTO_TACCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTO_TACCR {
        match value {
            i => AUTO_TACCR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `auto_clkdiv`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_CLKDIVR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl AUTO_CLKDIVR {
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
            AUTO_CLKDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AUTO_CLKDIVR {
        match value {
            i => AUTO_CLKDIVR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `delay_se_en`"]
pub enum DELAY_SE_ENW {}
impl DELAY_SE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _DELAY_SE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DELAY_SE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DELAY_SE_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `fast_read_mode_en`"]
pub enum FAST_READ_MODE_ENW {}
impl FAST_READ_MODE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _FAST_READ_MODE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FAST_READ_MODE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FAST_READ_MODE_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `en_prevent_fail`"]
pub enum EN_PREVENT_FAILW {}
impl EN_PREVENT_FAILW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EN_PREVENT_FAILW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_PREVENT_FAILW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_PREVENT_FAILW) -> &'a mut W {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `en_back2back_rds`"]
pub enum EN_BACK2BACK_RDSW {}
impl EN_BACK2BACK_RDSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EN_BACK2BACK_RDSW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_BACK2BACK_RDSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_BACK2BACK_RDSW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `en_back2back_wrs`"]
pub enum EN_BACK2BACK_WRSW {}
impl EN_BACK2BACK_WRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EN_BACK2BACK_WRSW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_BACK2BACK_WRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_BACK2BACK_WRSW) -> &'a mut W {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `en_merge_grab_gnt`"]
pub enum EN_MERGE_GRAB_GNTW {}
impl EN_MERGE_GRAB_GNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EN_MERGE_GRAB_GNTW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_MERGE_GRAB_GNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_MERGE_GRAB_GNTW) -> &'a mut W {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `auto_tacc`"]
pub enum AUTO_TACCW {}
impl AUTO_TACCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _AUTO_TACCW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTO_TACCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTO_TACCW) -> &'a mut W {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `auto_clkdiv`"]
pub enum AUTO_CLKDIVW {}
impl AUTO_CLKDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _AUTO_CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTO_CLKDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTO_CLKDIVW) -> &'a mut W {
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bit 0 - Delay SE Enable (Deprecated)"]
    #[inline]
    pub fn delay_se_en(&self) -> DELAY_SE_ENR {
        DELAY_SE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Fast Read Mode Enable (Deprecated)"]
    #[inline]
    pub fn fast_read_mode_en(&self) -> FAST_READ_MODE_ENR {
        FAST_READ_MODE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Prevent Fail Flag Set on FLC Busy"]
    #[inline]
    pub fn en_prevent_fail(&self) -> EN_PREVENT_FAILR {
        EN_PREVENT_FAILR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable Back To Back Reads"]
    #[inline]
    pub fn en_back2back_rds(&self) -> EN_BACK2BACK_RDSR {
        EN_BACK2BACK_RDSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enable Back To Back Writes"]
    #[inline]
    pub fn en_back2back_wrs(&self) -> EN_BACK2BACK_WRSR {
        EN_BACK2BACK_WRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable Merge Grab GNT"]
    #[inline]
    pub fn en_merge_grab_gnt(&self) -> EN_MERGE_GRAB_GNTR {
        EN_MERGE_GRAB_GNTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Auto TACC"]
    #[inline]
    pub fn auto_tacc(&self) -> AUTO_TACCR {
        AUTO_TACCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Auto CLKDIV"]
    #[inline]
    pub fn auto_clkdiv(&self) -> AUTO_CLKDIVR {
        AUTO_CLKDIVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
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
    #[doc = "Bit 0 - Delay SE Enable (Deprecated)"]
    #[inline]
    pub fn delay_se_en(&mut self) -> _DELAY_SE_ENW {
        _DELAY_SE_ENW { w: self }
    }
    #[doc = "Bit 8 - Fast Read Mode Enable (Deprecated)"]
    #[inline]
    pub fn fast_read_mode_en(&mut self) -> _FAST_READ_MODE_ENW {
        _FAST_READ_MODE_ENW { w: self }
    }
    #[doc = "Bit 12 - Prevent Fail Flag Set on FLC Busy"]
    #[inline]
    pub fn en_prevent_fail(&mut self) -> _EN_PREVENT_FAILW {
        _EN_PREVENT_FAILW { w: self }
    }
    #[doc = "Bit 16 - Enable Back To Back Reads"]
    #[inline]
    pub fn en_back2back_rds(&mut self) -> _EN_BACK2BACK_RDSW {
        _EN_BACK2BACK_RDSW { w: self }
    }
    #[doc = "Bit 20 - Enable Back To Back Writes"]
    #[inline]
    pub fn en_back2back_wrs(&mut self) -> _EN_BACK2BACK_WRSW {
        _EN_BACK2BACK_WRSW { w: self }
    }
    #[doc = "Bit 24 - Enable Merge Grab GNT"]
    #[inline]
    pub fn en_merge_grab_gnt(&mut self) -> _EN_MERGE_GRAB_GNTW {
        _EN_MERGE_GRAB_GNTW { w: self }
    }
    #[doc = "Bit 28 - Auto TACC"]
    #[inline]
    pub fn auto_tacc(&mut self) -> _AUTO_TACCW {
        _AUTO_TACCW { w: self }
    }
    #[doc = "Bit 29 - Auto CLKDIV"]
    #[inline]
    pub fn auto_clkdiv(&mut self) -> _AUTO_CLKDIVW {
        _AUTO_CLKDIVW { w: self }
    }
}
