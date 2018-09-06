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
#[doc = "Possible values of the field `start`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl STARTR {
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
            STARTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTR {
        match value {
            i => STARTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `crypt_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRYPT_MODER {
    #[doc = "Perform AES encryption operation."]
    ENCRYPT_MODE,
    #[doc = "Perform AES decryption operation."]
    DECRYPT_MODE,
}
impl CRYPT_MODER {
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
            CRYPT_MODER::ENCRYPT_MODE => false,
            CRYPT_MODER::DECRYPT_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRYPT_MODER {
        match value {
            false => CRYPT_MODER::ENCRYPT_MODE,
            true => CRYPT_MODER::DECRYPT_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `ENCRYPT_MODE`"]
    #[inline]
    pub fn is_encrypt_mode(&self) -> bool {
        *self == CRYPT_MODER::ENCRYPT_MODE
    }
    #[doc = "Checks if the value of the field is `DECRYPT_MODE`"]
    #[inline]
    pub fn is_decrypt_mode(&self) -> bool {
        *self == CRYPT_MODER::DECRYPT_MODE
    }
}
#[doc = "Possible values of the field `exp_key_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXP_KEY_MODER {
    #[doc = "Calculate new expanded key for this operation."]
    CALC_NEW_EXP_KEY,
    #[doc = "Use expanded key calculated by the last operation."]
    USE_LAST_EXP_KEY,
}
impl EXP_KEY_MODER {
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
            EXP_KEY_MODER::CALC_NEW_EXP_KEY => false,
            EXP_KEY_MODER::USE_LAST_EXP_KEY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXP_KEY_MODER {
        match value {
            false => EXP_KEY_MODER::CALC_NEW_EXP_KEY,
            true => EXP_KEY_MODER::USE_LAST_EXP_KEY,
        }
    }
    #[doc = "Checks if the value of the field is `CALC_NEW_EXP_KEY`"]
    #[inline]
    pub fn is_calc_new_exp_key(&self) -> bool {
        *self == EXP_KEY_MODER::CALC_NEW_EXP_KEY
    }
    #[doc = "Checks if the value of the field is `USE_LAST_EXP_KEY`"]
    #[inline]
    pub fn is_use_last_exp_key(&self) -> bool {
        *self == EXP_KEY_MODER::USE_LAST_EXP_KEY
    }
}
#[doc = "Possible values of the field `key_size`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEY_SIZER {
    #[doc = "Use 128-bit AES key size."]
    KEY_SIZE_128,
    #[doc = "Use 192-bit AES key size."]
    KEY_SIZE_192,
    #[doc = "Use 256-bit AES key size."]
    KEY_SIZE_256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KEY_SIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KEY_SIZER::KEY_SIZE_128 => 0,
            KEY_SIZER::KEY_SIZE_192 => 1,
            KEY_SIZER::KEY_SIZE_256 => 2,
            KEY_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KEY_SIZER {
        match value {
            0 => KEY_SIZER::KEY_SIZE_128,
            1 => KEY_SIZER::KEY_SIZE_192,
            2 => KEY_SIZER::KEY_SIZE_256,
            i => KEY_SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `KEY_SIZE_128`"]
    #[inline]
    pub fn is_key_size_128(&self) -> bool {
        *self == KEY_SIZER::KEY_SIZE_128
    }
    #[doc = "Checks if the value of the field is `KEY_SIZE_192`"]
    #[inline]
    pub fn is_key_size_192(&self) -> bool {
        *self == KEY_SIZER::KEY_SIZE_192
    }
    #[doc = "Checks if the value of the field is `KEY_SIZE_256`"]
    #[inline]
    pub fn is_key_size_256(&self) -> bool {
        *self == KEY_SIZER::KEY_SIZE_256
    }
}
#[doc = "Possible values of the field `inten`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl INTENR {
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
            INTENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTENR {
        match value {
            i => INTENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `intfl`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTFLR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl INTFLR {
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
            INTFLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTFLR {
        match value {
            i => INTFLR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `start`"]
pub enum STARTW {}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `crypt_mode`"]
pub enum CRYPT_MODEW {
    #[doc = "Perform AES encryption operation."]
    ENCRYPT_MODE,
    #[doc = "Perform AES decryption operation."]
    DECRYPT_MODE,
}
impl CRYPT_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRYPT_MODEW::ENCRYPT_MODE => false,
            CRYPT_MODEW::DECRYPT_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRYPT_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRYPT_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRYPT_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Perform AES encryption operation."]
    #[inline]
    pub fn encrypt_mode(self) -> &'a mut W {
        self.variant(CRYPT_MODEW::ENCRYPT_MODE)
    }
    #[doc = "Perform AES decryption operation."]
    #[inline]
    pub fn decrypt_mode(self) -> &'a mut W {
        self.variant(CRYPT_MODEW::DECRYPT_MODE)
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
#[doc = "Values that can be written to the field `exp_key_mode`"]
pub enum EXP_KEY_MODEW {
    #[doc = "Calculate new expanded key for this operation."]
    CALC_NEW_EXP_KEY,
    #[doc = "Use expanded key calculated by the last operation."]
    USE_LAST_EXP_KEY,
}
impl EXP_KEY_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXP_KEY_MODEW::CALC_NEW_EXP_KEY => false,
            EXP_KEY_MODEW::USE_LAST_EXP_KEY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXP_KEY_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXP_KEY_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXP_KEY_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Calculate new expanded key for this operation."]
    #[inline]
    pub fn calc_new_exp_key(self) -> &'a mut W {
        self.variant(EXP_KEY_MODEW::CALC_NEW_EXP_KEY)
    }
    #[doc = "Use expanded key calculated by the last operation."]
    #[inline]
    pub fn use_last_exp_key(self) -> &'a mut W {
        self.variant(EXP_KEY_MODEW::USE_LAST_EXP_KEY)
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
#[doc = "Values that can be written to the field `key_size`"]
pub enum KEY_SIZEW {
    #[doc = "Use 128-bit AES key size."]
    KEY_SIZE_128,
    #[doc = "Use 192-bit AES key size."]
    KEY_SIZE_192,
    #[doc = "Use 256-bit AES key size."]
    KEY_SIZE_256,
}
impl KEY_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KEY_SIZEW::KEY_SIZE_128 => 0,
            KEY_SIZEW::KEY_SIZE_192 => 1,
            KEY_SIZEW::KEY_SIZE_256 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEY_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _KEY_SIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEY_SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use 128-bit AES key size."]
    #[inline]
    pub fn key_size_128(self) -> &'a mut W {
        self.variant(KEY_SIZEW::KEY_SIZE_128)
    }
    #[doc = "Use 192-bit AES key size."]
    #[inline]
    pub fn key_size_192(self) -> &'a mut W {
        self.variant(KEY_SIZEW::KEY_SIZE_192)
    }
    #[doc = "Use 256-bit AES key size."]
    #[inline]
    pub fn key_size_256(self) -> &'a mut W {
        self.variant(KEY_SIZEW::KEY_SIZE_256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `inten`"]
pub enum INTENW {}
impl INTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _INTENW<'a> {
    w: &'a mut W,
}
impl<'a> _INTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `intfl`"]
pub enum INTFLW {}
impl INTFLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _INTFLW<'a> {
    w: &'a mut W,
}
impl<'a> _INTFLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTFLW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - AES Start/Busy"]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - AES Encrypt/Decrypt Mode"]
    #[inline]
    pub fn crypt_mode(&self) -> CRYPT_MODER {
        CRYPT_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - AES Expanded Key Mode"]
    #[inline]
    pub fn exp_key_mode(&self) -> EXP_KEY_MODER {
        EXP_KEY_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - AES Key Size Select"]
    #[inline]
    pub fn key_size(&self) -> KEY_SIZER {
        KEY_SIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - AES Interrupt Enable"]
    #[inline]
    pub fn inten(&self) -> INTENR {
        INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - AES Interrupt Flag"]
    #[inline]
    pub fn intfl(&self) -> INTFLR {
        INTFLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
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
    #[doc = "Bit 0 - AES Start/Busy"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 1 - AES Encrypt/Decrypt Mode"]
    #[inline]
    pub fn crypt_mode(&mut self) -> _CRYPT_MODEW {
        _CRYPT_MODEW { w: self }
    }
    #[doc = "Bit 2 - AES Expanded Key Mode"]
    #[inline]
    pub fn exp_key_mode(&mut self) -> _EXP_KEY_MODEW {
        _EXP_KEY_MODEW { w: self }
    }
    #[doc = "Bits 3:4 - AES Key Size Select"]
    #[inline]
    pub fn key_size(&mut self) -> _KEY_SIZEW {
        _KEY_SIZEW { w: self }
    }
    #[doc = "Bit 5 - AES Interrupt Enable"]
    #[inline]
    pub fn inten(&mut self) -> _INTENW {
        _INTENW { w: self }
    }
    #[doc = "Bit 6 - AES Interrupt Flag"]
    #[inline]
    pub fn intfl(&mut self) -> _INTFLW {
        _INTFLW { w: self }
    }
}
