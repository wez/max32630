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
#[doc = "Possible values of the field `opsel`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPSELR {
    #[doc = "Exponentiation."]
    EXP,
    #[doc = "Square operation."]
    SQR,
    #[doc = "Multiply."]
    MUL,
    #[doc = "Square operation followed by multiply."]
    SQRMUL,
    #[doc = "Addition."]
    ADD,
    #[doc = "Subtraction."]
    SUB,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPSELR::EXP => 0,
            OPSELR::SQR => 1,
            OPSELR::MUL => 2,
            OPSELR::SQRMUL => 3,
            OPSELR::ADD => 4,
            OPSELR::SUB => 5,
            OPSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPSELR {
        match value {
            0 => OPSELR::EXP,
            1 => OPSELR::SQR,
            2 => OPSELR::MUL,
            3 => OPSELR::SQRMUL,
            4 => OPSELR::ADD,
            5 => OPSELR::SUB,
            i => OPSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXP`"]
    #[inline]
    pub fn is_exp(&self) -> bool {
        *self == OPSELR::EXP
    }
    #[doc = "Checks if the value of the field is `SQR`"]
    #[inline]
    pub fn is_sqr(&self) -> bool {
        *self == OPSELR::SQR
    }
    #[doc = "Checks if the value of the field is `MUL`"]
    #[inline]
    pub fn is_mul(&self) -> bool {
        *self == OPSELR::MUL
    }
    #[doc = "Checks if the value of the field is `SQRMUL`"]
    #[inline]
    pub fn is_sqrmul(&self) -> bool {
        *self == OPSELR::SQRMUL
    }
    #[doc = "Checks if the value of the field is `ADD`"]
    #[inline]
    pub fn is_add(&self) -> bool {
        *self == OPSELR::ADD
    }
    #[doc = "Checks if the value of the field is `SUB`"]
    #[inline]
    pub fn is_sub(&self) -> bool {
        *self == OPSELR::SUB
    }
}
#[doc = "Possible values of the field `ocalc`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCALCR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl OCALCR {
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
            OCALCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OCALCR {
        match value {
            i => OCALCR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `if_done`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF_DONER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IF_DONER {
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
            IF_DONER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IF_DONER {
        match value {
            i => IF_DONER::_Reserved(i),
        }
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
#[doc = "Possible values of the field `if_error`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IF_ERRORR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl IF_ERRORR {
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
            IF_ERRORR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IF_ERRORR {
        match value {
            i => IF_ERRORR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ofs_a`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFS_AR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OFS_AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OFS_AR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OFS_AR {
        match value {
            i => OFS_AR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ofs_b`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFS_BR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OFS_BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OFS_BR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OFS_BR {
        match value {
            i => OFS_BR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ofs_exp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFS_EXPR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OFS_EXPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OFS_EXPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OFS_EXPR {
        match value {
            i => OFS_EXPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ofs_mod`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFS_MODR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OFS_MODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OFS_MODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OFS_MODR {
        match value {
            i => OFS_MODR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `seg_a`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEG_AR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEG_AR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEG_AR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEG_AR {
        match value {
            i => SEG_AR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `seg_b`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEG_BR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEG_BR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEG_BR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEG_BR {
        match value {
            i => SEG_BR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `seg_res`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEG_RESR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEG_RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEG_RESR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEG_RESR {
        match value {
            i => SEG_RESR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `seg_tmp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEG_TMPR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SEG_TMPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SEG_TMPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SEG_TMPR {
        match value {
            i => SEG_TMPR::_Reserved(i),
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
#[doc = "Values that can be written to the field `opsel`"]
pub enum OPSELW {
    #[doc = "Exponentiation."]
    EXP,
    #[doc = "Square operation."]
    SQR,
    #[doc = "Multiply."]
    MUL,
    #[doc = "Square operation followed by multiply."]
    SQRMUL,
    #[doc = "Addition."]
    ADD,
    #[doc = "Subtraction."]
    SUB,
}
impl OPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPSELW::EXP => 0,
            OPSELW::SQR => 1,
            OPSELW::MUL => 2,
            OPSELW::SQRMUL => 3,
            OPSELW::ADD => 4,
            OPSELW::SUB => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _OPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Exponentiation."]
    #[inline]
    pub fn exp(self) -> &'a mut W {
        self.variant(OPSELW::EXP)
    }
    #[doc = "Square operation."]
    #[inline]
    pub fn sqr(self) -> &'a mut W {
        self.variant(OPSELW::SQR)
    }
    #[doc = "Multiply."]
    #[inline]
    pub fn mul(self) -> &'a mut W {
        self.variant(OPSELW::MUL)
    }
    #[doc = "Square operation followed by multiply."]
    #[inline]
    pub fn sqrmul(self) -> &'a mut W {
        self.variant(OPSELW::SQRMUL)
    }
    #[doc = "Addition."]
    #[inline]
    pub fn add(self) -> &'a mut W {
        self.variant(OPSELW::ADD)
    }
    #[doc = "Subtraction."]
    #[inline]
    pub fn sub(self) -> &'a mut W {
        self.variant(OPSELW::SUB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ocalc`"]
pub enum OCALCW {}
impl OCALCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OCALCW<'a> {
    w: &'a mut W,
}
impl<'a> _OCALCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCALCW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `if_done`"]
pub enum IF_DONEW {}
impl IF_DONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _IF_DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _IF_DONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IF_DONEW) -> &'a mut W {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `if_error`"]
pub enum IF_ERRORW {}
impl IF_ERRORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _IF_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _IF_ERRORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IF_ERRORW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ofs_a`"]
pub enum OFS_AW {}
impl OFS_AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OFS_AW<'a> {
    w: &'a mut W,
}
impl<'a> _OFS_AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFS_AW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ofs_b`"]
pub enum OFS_BW {}
impl OFS_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OFS_BW<'a> {
    w: &'a mut W,
}
impl<'a> _OFS_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFS_BW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ofs_exp`"]
pub enum OFS_EXPW {}
impl OFS_EXPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OFS_EXPW<'a> {
    w: &'a mut W,
}
impl<'a> _OFS_EXPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFS_EXPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ofs_mod`"]
pub enum OFS_MODW {}
impl OFS_MODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _OFS_MODW<'a> {
    w: &'a mut W,
}
impl<'a> _OFS_MODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFS_MODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `seg_a`"]
pub enum SEG_AW {}
impl SEG_AW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SEG_AW<'a> {
    w: &'a mut W,
}
impl<'a> _SEG_AW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEG_AW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `seg_b`"]
pub enum SEG_BW {}
impl SEG_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SEG_BW<'a> {
    w: &'a mut W,
}
impl<'a> _SEG_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEG_BW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `seg_res`"]
pub enum SEG_RESW {}
impl SEG_RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SEG_RESW<'a> {
    w: &'a mut W,
}
impl<'a> _SEG_RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEG_RESW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `seg_tmp`"]
pub enum SEG_TMPW {}
impl SEG_TMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SEG_TMPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEG_TMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEG_TMPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Start MAA Calculation"]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Select Operation Type"]
    #[inline]
    pub fn opsel(&self) -> OPSELR {
        OPSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Optimized Calculation Control"]
    #[inline]
    pub fn ocalc(&self) -> OCALCR {
        OCALCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt Flag - Calculation Done"]
    #[inline]
    pub fn if_done(&self) -> IF_DONER {
        IF_DONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - MAA Interrupt Enable"]
    #[inline]
    pub fn inten(&self) -> INTENR {
        INTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Interrupt Flag - Error"]
    #[inline]
    pub fn if_error(&self) -> IF_ERRORR {
        IF_ERRORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Operand A Memory Offset Select"]
    #[inline]
    pub fn ofs_a(&self) -> OFS_AR {
        OFS_AR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Operand B Memory Offset Select"]
    #[inline]
    pub fn ofs_b(&self) -> OFS_BR {
        OFS_BR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Exponent Memory Offset Select"]
    #[inline]
    pub fn ofs_exp(&self) -> OFS_EXPR {
        OFS_EXPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Modulus Memory Select"]
    #[inline]
    pub fn ofs_mod(&self) -> OFS_MODR {
        OFS_MODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Operand A Memory Segment Select"]
    #[inline]
    pub fn seg_a(&self) -> SEG_AR {
        SEG_AR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Operand B Memory Segment Select"]
    #[inline]
    pub fn seg_b(&self) -> SEG_BR {
        SEG_BR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Result Memory Segment Select"]
    #[inline]
    pub fn seg_res(&self) -> SEG_RESR {
        SEG_RESR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Temporary Memory Segment Select"]
    #[inline]
    pub fn seg_tmp(&self) -> SEG_TMPR {
        SEG_TMPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Start MAA Calculation"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bits 1:3 - Select Operation Type"]
    #[inline]
    pub fn opsel(&mut self) -> _OPSELW {
        _OPSELW { w: self }
    }
    #[doc = "Bit 4 - Optimized Calculation Control"]
    #[inline]
    pub fn ocalc(&mut self) -> _OCALCW {
        _OCALCW { w: self }
    }
    #[doc = "Bit 5 - Interrupt Flag - Calculation Done"]
    #[inline]
    pub fn if_done(&mut self) -> _IF_DONEW {
        _IF_DONEW { w: self }
    }
    #[doc = "Bit 6 - MAA Interrupt Enable"]
    #[inline]
    pub fn inten(&mut self) -> _INTENW {
        _INTENW { w: self }
    }
    #[doc = "Bit 7 - Interrupt Flag - Error"]
    #[inline]
    pub fn if_error(&mut self) -> _IF_ERRORW {
        _IF_ERRORW { w: self }
    }
    #[doc = "Bits 8:9 - Operand A Memory Offset Select"]
    #[inline]
    pub fn ofs_a(&mut self) -> _OFS_AW {
        _OFS_AW { w: self }
    }
    #[doc = "Bits 10:11 - Operand B Memory Offset Select"]
    #[inline]
    pub fn ofs_b(&mut self) -> _OFS_BW {
        _OFS_BW { w: self }
    }
    #[doc = "Bits 12:13 - Exponent Memory Offset Select"]
    #[inline]
    pub fn ofs_exp(&mut self) -> _OFS_EXPW {
        _OFS_EXPW { w: self }
    }
    #[doc = "Bits 14:15 - Modulus Memory Select"]
    #[inline]
    pub fn ofs_mod(&mut self) -> _OFS_MODW {
        _OFS_MODW { w: self }
    }
    #[doc = "Bits 16:19 - Operand A Memory Segment Select"]
    #[inline]
    pub fn seg_a(&mut self) -> _SEG_AW {
        _SEG_AW { w: self }
    }
    #[doc = "Bits 20:23 - Operand B Memory Segment Select"]
    #[inline]
    pub fn seg_b(&mut self) -> _SEG_BW {
        _SEG_BW { w: self }
    }
    #[doc = "Bits 24:27 - Result Memory Segment Select"]
    #[inline]
    pub fn seg_res(&mut self) -> _SEG_RESW {
        _SEG_RESW { w: self }
    }
    #[doc = "Bits 28:31 - Temporary Memory Segment Select"]
    #[inline]
    pub fn seg_tmp(&mut self) -> _SEG_TMPW {
        _SEG_TMPW { w: self }
    }
}
