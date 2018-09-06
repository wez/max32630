#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::REG0 {
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
#[doc = "Possible values of the field `pwr_lp1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_LP1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_LP1R {
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
            PWR_LP1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_LP1R {
        match value {
            i => PWR_LP1R::_Reserved(i),
        }
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
#[doc = "Possible values of the field `pwr_flashen_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_FLASHEN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_FLASHEN_RUNR {
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
            PWR_FLASHEN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_FLASHEN_RUNR {
        match value {
            i => PWR_FLASHEN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_flashen_slp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_FLASHEN_SLPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_FLASHEN_SLPR {
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
            PWR_FLASHEN_SLPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_FLASHEN_SLPR {
        match value {
            i => PWR_FLASHEN_SLPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_retregen_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_RETREGEN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_RETREGEN_RUNR {
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
            PWR_RETREGEN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_RETREGEN_RUNR {
        match value {
            i => PWR_RETREGEN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_retregen_slp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_RETREGEN_SLPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_RETREGEN_SLPR {
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
            PWR_RETREGEN_SLPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_RETREGEN_SLPR {
        match value {
            i => PWR_RETREGEN_SLPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_roen_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_ROEN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_ROEN_RUNR {
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
            PWR_ROEN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_ROEN_RUNR {
        match value {
            i => PWR_ROEN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_roen_slp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_ROEN_SLPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_ROEN_SLPR {
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
            PWR_ROEN_SLPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_ROEN_SLPR {
        match value {
            i => PWR_ROEN_SLPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_nren_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_NREN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_NREN_RUNR {
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
            PWR_NREN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_NREN_RUNR {
        match value {
            i => PWR_NREN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_nren_slp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_NREN_SLPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_NREN_SLPR {
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
            PWR_NREN_SLPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_NREN_SLPR {
        match value {
            i => PWR_NREN_SLPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_rtcen_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_RTCEN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_RTCEN_RUNR {
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
            PWR_RTCEN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_RTCEN_RUNR {
        match value {
            i => PWR_RTCEN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_rtcen_slp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_RTCEN_SLPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_RTCEN_SLPR {
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
            PWR_RTCEN_SLPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_RTCEN_SLPR {
        match value {
            i => PWR_RTCEN_SLPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_svm12en_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_SVM12EN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_SVM12EN_RUNR {
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
            PWR_SVM12EN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_SVM12EN_RUNR {
        match value {
            i => PWR_SVM12EN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_svm18en_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_SVM18EN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_SVM18EN_RUNR {
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
            PWR_SVM18EN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_SVM18EN_RUNR {
        match value {
            i => PWR_SVM18EN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_svmrtcen_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_SVMRTCEN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_SVMRTCEN_RUNR {
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
            PWR_SVMRTCEN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_SVMRTCEN_RUNR {
        match value {
            i => PWR_SVMRTCEN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_svm_vddb_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_SVM_VDDB_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_SVM_VDDB_RUNR {
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
            PWR_SVM_VDDB_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_SVM_VDDB_RUNR {
        match value {
            i => PWR_SVM_VDDB_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_svmtvdd12en_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_SVMTVDD12EN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_SVMTVDD12EN_RUNR {
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
            PWR_SVMTVDD12EN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_SVMTVDD12EN_RUNR {
        match value {
            i => PWR_SVMTVDD12EN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_vdd12_swen_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VDD12_SWEN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_VDD12_SWEN_RUNR {
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
            PWR_VDD12_SWEN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_VDD12_SWEN_RUNR {
        match value {
            i => PWR_VDD12_SWEN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_vdd12_swen_slp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VDD12_SWEN_SLPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_VDD12_SWEN_SLPR {
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
            PWR_VDD12_SWEN_SLPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_VDD12_SWEN_SLPR {
        match value {
            i => PWR_VDD12_SWEN_SLPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_vdd18_swen_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VDD18_SWEN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_VDD18_SWEN_RUNR {
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
            PWR_VDD18_SWEN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_VDD18_SWEN_RUNR {
        match value {
            i => PWR_VDD18_SWEN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_vdd18_swen_slp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_VDD18_SWEN_SLPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_VDD18_SWEN_SLPR {
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
            PWR_VDD18_SWEN_SLPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_VDD18_SWEN_SLPR {
        match value {
            i => PWR_VDD18_SWEN_SLPR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_tvdd12_swen_run`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TVDD12_SWEN_RUNR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_TVDD12_SWEN_RUNR {
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
            PWR_TVDD12_SWEN_RUNR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_TVDD12_SWEN_RUNR {
        match value {
            i => PWR_TVDD12_SWEN_RUNR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pwr_tvdd12_swen_slp`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWR_TVDD12_SWEN_SLPR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl PWR_TVDD12_SWEN_SLPR {
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
            PWR_TVDD12_SWEN_SLPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWR_TVDD12_SWEN_SLPR {
        match value {
            i => PWR_TVDD12_SWEN_SLPR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `pwr_lp1`"]
pub enum PWR_LP1W {}
impl PWR_LP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_LP1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_LP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_LP1W) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_first_boot`"]
pub enum PWR_FIRST_BOOTW {}
impl PWR_FIRST_BOOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_FIRST_BOOTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_FIRST_BOOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_FIRST_BOOTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_sys_reboot`"]
pub enum PWR_SYS_REBOOTW {}
impl PWR_SYS_REBOOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_SYS_REBOOTW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_SYS_REBOOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_SYS_REBOOTW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_flashen_run`"]
pub enum PWR_FLASHEN_RUNW {}
impl PWR_FLASHEN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_FLASHEN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_FLASHEN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_FLASHEN_RUNW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_flashen_slp`"]
pub enum PWR_FLASHEN_SLPW {}
impl PWR_FLASHEN_SLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_FLASHEN_SLPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_FLASHEN_SLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_FLASHEN_SLPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_retregen_run`"]
pub enum PWR_RETREGEN_RUNW {}
impl PWR_RETREGEN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_RETREGEN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_RETREGEN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_RETREGEN_RUNW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_retregen_slp`"]
pub enum PWR_RETREGEN_SLPW {}
impl PWR_RETREGEN_SLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_RETREGEN_SLPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_RETREGEN_SLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_RETREGEN_SLPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_roen_run`"]
pub enum PWR_ROEN_RUNW {}
impl PWR_ROEN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_ROEN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_ROEN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_ROEN_RUNW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_roen_slp`"]
pub enum PWR_ROEN_SLPW {}
impl PWR_ROEN_SLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_ROEN_SLPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_ROEN_SLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_ROEN_SLPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_nren_run`"]
pub enum PWR_NREN_RUNW {}
impl PWR_NREN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_NREN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_NREN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_NREN_RUNW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_nren_slp`"]
pub enum PWR_NREN_SLPW {}
impl PWR_NREN_SLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_NREN_SLPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_NREN_SLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_NREN_SLPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_rtcen_run`"]
pub enum PWR_RTCEN_RUNW {}
impl PWR_RTCEN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_RTCEN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_RTCEN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_RTCEN_RUNW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_rtcen_slp`"]
pub enum PWR_RTCEN_SLPW {}
impl PWR_RTCEN_SLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_RTCEN_SLPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_RTCEN_SLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_RTCEN_SLPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_svm12en_run`"]
pub enum PWR_SVM12EN_RUNW {}
impl PWR_SVM12EN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_SVM12EN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_SVM12EN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_SVM12EN_RUNW) -> &'a mut W {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_svm18en_run`"]
pub enum PWR_SVM18EN_RUNW {}
impl PWR_SVM18EN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_SVM18EN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_SVM18EN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_SVM18EN_RUNW) -> &'a mut W {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_svmrtcen_run`"]
pub enum PWR_SVMRTCEN_RUNW {}
impl PWR_SVMRTCEN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_SVMRTCEN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_SVMRTCEN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_SVMRTCEN_RUNW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_svm_vddb_run`"]
pub enum PWR_SVM_VDDB_RUNW {}
impl PWR_SVM_VDDB_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_SVM_VDDB_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_SVM_VDDB_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_SVM_VDDB_RUNW) -> &'a mut W {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_svmtvdd12en_run`"]
pub enum PWR_SVMTVDD12EN_RUNW {}
impl PWR_SVMTVDD12EN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_SVMTVDD12EN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_SVMTVDD12EN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_SVMTVDD12EN_RUNW) -> &'a mut W {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_vdd12_swen_run`"]
pub enum PWR_VDD12_SWEN_RUNW {}
impl PWR_VDD12_SWEN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VDD12_SWEN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VDD12_SWEN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VDD12_SWEN_RUNW) -> &'a mut W {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_vdd12_swen_slp`"]
pub enum PWR_VDD12_SWEN_SLPW {}
impl PWR_VDD12_SWEN_SLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VDD12_SWEN_SLPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VDD12_SWEN_SLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VDD12_SWEN_SLPW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pwr_vdd18_swen_run`"]
pub enum PWR_VDD18_SWEN_RUNW {}
impl PWR_VDD18_SWEN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VDD18_SWEN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VDD18_SWEN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VDD18_SWEN_RUNW) -> &'a mut W {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_vdd18_swen_slp`"]
pub enum PWR_VDD18_SWEN_SLPW {}
impl PWR_VDD18_SWEN_SLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_VDD18_SWEN_SLPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_VDD18_SWEN_SLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_VDD18_SWEN_SLPW) -> &'a mut W {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_tvdd12_swen_run`"]
pub enum PWR_TVDD12_SWEN_RUNW {}
impl PWR_TVDD12_SWEN_RUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TVDD12_SWEN_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TVDD12_SWEN_RUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TVDD12_SWEN_RUNW) -> &'a mut W {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `pwr_tvdd12_swen_slp`"]
pub enum PWR_TVDD12_SWEN_SLPW {}
impl PWR_TVDD12_SWEN_SLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PWR_TVDD12_SWEN_SLPW<'a> {
    w: &'a mut W,
}
impl<'a> _PWR_TVDD12_SWEN_SLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWR_TVDD12_SWEN_SLPW) -> &'a mut W {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Shutdown Power Mode Select"]
    #[inline]
    pub fn pwr_lp1(&self) -> PWR_LP1R {
        PWR_LP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wake on First Boot"]
    #[inline]
    pub fn pwr_first_boot(&self) -> PWR_FIRST_BOOTR {
        PWR_FIRST_BOOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Enable Flash Operation during Run Mode"]
    #[inline]
    pub fn pwr_flashen_run(&self) -> PWR_FLASHEN_RUNR {
        PWR_FLASHEN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Enable Flash Operation during Sleep Mode"]
    #[inline]
    pub fn pwr_flashen_slp(&self) -> PWR_FLASHEN_SLPR {
        PWR_FLASHEN_SLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Enable Retention Regulator Operation during Run Mode"]
    #[inline]
    pub fn pwr_retregen_run(&self) -> PWR_RETREGEN_RUNR {
        PWR_RETREGEN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Enable Retention Regulator Operation during Sleep Mode"]
    #[inline]
    pub fn pwr_retregen_slp(&self) -> PWR_RETREGEN_SLPR {
        PWR_RETREGEN_SLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable 96MHz System Relaxation Oscillator in Run Mode"]
    #[inline]
    pub fn pwr_roen_run(&self) -> PWR_ROEN_RUNR {
        PWR_ROEN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable 96MHz System Relaxation Oscillator in Sleep Mode"]
    #[inline]
    pub fn pwr_roen_slp(&self) -> PWR_ROEN_SLPR {
        PWR_ROEN_SLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enable Nano Oscillator in Run Mode"]
    #[inline]
    pub fn pwr_nren_run(&self) -> PWR_NREN_RUNR {
        PWR_NREN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Enable Nano Oscillator in Sleep Mode"]
    #[inline]
    pub fn pwr_nren_slp(&self) -> PWR_NREN_SLPR {
        PWR_NREN_SLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Enable Real Time Clock Operation during Run Mode"]
    #[inline]
    pub fn pwr_rtcen_run(&self) -> PWR_RTCEN_RUNR {
        PWR_RTCEN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Enable Real Time Clock Operation during Sleep Mode"]
    #[inline]
    pub fn pwr_rtcen_slp(&self) -> PWR_RTCEN_SLPR {
        PWR_RTCEN_SLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enable VDD12_SW SVM operation during Run Mode"]
    #[inline]
    pub fn pwr_svm12en_run(&self) -> PWR_SVM12EN_RUNR {
        PWR_SVM12EN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Enable VDD18_SW SVM operation during Run Mode"]
    #[inline]
    pub fn pwr_svm18en_run(&self) -> PWR_SVM18EN_RUNR {
        PWR_SVM18EN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enable VRTC SVM operation during Run Mode"]
    #[inline]
    pub fn pwr_svmrtcen_run(&self) -> PWR_SVMRTCEN_RUNR {
        PWR_SVMRTCEN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Enable VDDB SVM operation during Run Mode"]
    #[inline]
    pub fn pwr_svm_vddb_run(&self) -> PWR_SVM_VDDB_RUNR {
        PWR_SVM_VDDB_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Enable TVDD12 SVM operation during Run Mode"]
    #[inline]
    pub fn pwr_svmtvdd12en_run(&self) -> PWR_SVMTVDD12EN_RUNR {
        PWR_SVMTVDD12EN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Enable VDD12 switching during Run Mode"]
    #[inline]
    pub fn pwr_vdd12_swen_run(&self) -> PWR_VDD12_SWEN_RUNR {
        PWR_VDD12_SWEN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Enable VDD12 switching during Sleep Mode"]
    #[inline]
    pub fn pwr_vdd12_swen_slp(&self) -> PWR_VDD12_SWEN_SLPR {
        PWR_VDD12_SWEN_SLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Enable VDD18 switching during Run Mode"]
    #[inline]
    pub fn pwr_vdd18_swen_run(&self) -> PWR_VDD18_SWEN_RUNR {
        PWR_VDD18_SWEN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Enable VDD18 switching during Sleep Mode"]
    #[inline]
    pub fn pwr_vdd18_swen_slp(&self) -> PWR_VDD18_SWEN_SLPR {
        PWR_VDD18_SWEN_SLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Enable TVDD12 switching during Run Mode"]
    #[inline]
    pub fn pwr_tvdd12_swen_run(&self) -> PWR_TVDD12_SWEN_RUNR {
        PWR_TVDD12_SWEN_RUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Enable TVDD12 switching during Sleep Mode"]
    #[inline]
    pub fn pwr_tvdd12_swen_slp(&self) -> PWR_TVDD12_SWEN_SLPR {
        PWR_TVDD12_SWEN_SLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Shutdown Power Mode Select"]
    #[inline]
    pub fn pwr_lp1(&mut self) -> _PWR_LP1W {
        _PWR_LP1W { w: self }
    }
    #[doc = "Bit 1 - Wake on First Boot"]
    #[inline]
    pub fn pwr_first_boot(&mut self) -> _PWR_FIRST_BOOTW {
        _PWR_FIRST_BOOTW { w: self }
    }
    #[doc = "Bit 2 - Firmware System Reboot Request"]
    #[inline]
    pub fn pwr_sys_reboot(&mut self) -> _PWR_SYS_REBOOTW {
        _PWR_SYS_REBOOTW { w: self }
    }
    #[doc = "Bit 3 - Enable Flash Operation during Run Mode"]
    #[inline]
    pub fn pwr_flashen_run(&mut self) -> _PWR_FLASHEN_RUNW {
        _PWR_FLASHEN_RUNW { w: self }
    }
    #[doc = "Bit 4 - Enable Flash Operation during Sleep Mode"]
    #[inline]
    pub fn pwr_flashen_slp(&mut self) -> _PWR_FLASHEN_SLPW {
        _PWR_FLASHEN_SLPW { w: self }
    }
    #[doc = "Bit 5 - Enable Retention Regulator Operation during Run Mode"]
    #[inline]
    pub fn pwr_retregen_run(&mut self) -> _PWR_RETREGEN_RUNW {
        _PWR_RETREGEN_RUNW { w: self }
    }
    #[doc = "Bit 6 - Enable Retention Regulator Operation during Sleep Mode"]
    #[inline]
    pub fn pwr_retregen_slp(&mut self) -> _PWR_RETREGEN_SLPW {
        _PWR_RETREGEN_SLPW { w: self }
    }
    #[doc = "Bit 7 - Enable 96MHz System Relaxation Oscillator in Run Mode"]
    #[inline]
    pub fn pwr_roen_run(&mut self) -> _PWR_ROEN_RUNW {
        _PWR_ROEN_RUNW { w: self }
    }
    #[doc = "Bit 8 - Enable 96MHz System Relaxation Oscillator in Sleep Mode"]
    #[inline]
    pub fn pwr_roen_slp(&mut self) -> _PWR_ROEN_SLPW {
        _PWR_ROEN_SLPW { w: self }
    }
    #[doc = "Bit 9 - Enable Nano Oscillator in Run Mode"]
    #[inline]
    pub fn pwr_nren_run(&mut self) -> _PWR_NREN_RUNW {
        _PWR_NREN_RUNW { w: self }
    }
    #[doc = "Bit 10 - Enable Nano Oscillator in Sleep Mode"]
    #[inline]
    pub fn pwr_nren_slp(&mut self) -> _PWR_NREN_SLPW {
        _PWR_NREN_SLPW { w: self }
    }
    #[doc = "Bit 11 - Enable Real Time Clock Operation during Run Mode"]
    #[inline]
    pub fn pwr_rtcen_run(&mut self) -> _PWR_RTCEN_RUNW {
        _PWR_RTCEN_RUNW { w: self }
    }
    #[doc = "Bit 12 - Enable Real Time Clock Operation during Sleep Mode"]
    #[inline]
    pub fn pwr_rtcen_slp(&mut self) -> _PWR_RTCEN_SLPW {
        _PWR_RTCEN_SLPW { w: self }
    }
    #[doc = "Bit 13 - Enable VDD12_SW SVM operation during Run Mode"]
    #[inline]
    pub fn pwr_svm12en_run(&mut self) -> _PWR_SVM12EN_RUNW {
        _PWR_SVM12EN_RUNW { w: self }
    }
    #[doc = "Bit 15 - Enable VDD18_SW SVM operation during Run Mode"]
    #[inline]
    pub fn pwr_svm18en_run(&mut self) -> _PWR_SVM18EN_RUNW {
        _PWR_SVM18EN_RUNW { w: self }
    }
    #[doc = "Bit 17 - Enable VRTC SVM operation during Run Mode"]
    #[inline]
    pub fn pwr_svmrtcen_run(&mut self) -> _PWR_SVMRTCEN_RUNW {
        _PWR_SVMRTCEN_RUNW { w: self }
    }
    #[doc = "Bit 19 - Enable VDDB SVM operation during Run Mode"]
    #[inline]
    pub fn pwr_svm_vddb_run(&mut self) -> _PWR_SVM_VDDB_RUNW {
        _PWR_SVM_VDDB_RUNW { w: self }
    }
    #[doc = "Bit 21 - Enable TVDD12 SVM operation during Run Mode"]
    #[inline]
    pub fn pwr_svmtvdd12en_run(&mut self) -> _PWR_SVMTVDD12EN_RUNW {
        _PWR_SVMTVDD12EN_RUNW { w: self }
    }
    #[doc = "Bit 23 - Enable VDD12 switching during Run Mode"]
    #[inline]
    pub fn pwr_vdd12_swen_run(&mut self) -> _PWR_VDD12_SWEN_RUNW {
        _PWR_VDD12_SWEN_RUNW { w: self }
    }
    #[doc = "Bit 24 - Enable VDD12 switching during Sleep Mode"]
    #[inline]
    pub fn pwr_vdd12_swen_slp(&mut self) -> _PWR_VDD12_SWEN_SLPW {
        _PWR_VDD12_SWEN_SLPW { w: self }
    }
    #[doc = "Bit 25 - Enable VDD18 switching during Run Mode"]
    #[inline]
    pub fn pwr_vdd18_swen_run(&mut self) -> _PWR_VDD18_SWEN_RUNW {
        _PWR_VDD18_SWEN_RUNW { w: self }
    }
    #[doc = "Bit 26 - Enable VDD18 switching during Sleep Mode"]
    #[inline]
    pub fn pwr_vdd18_swen_slp(&mut self) -> _PWR_VDD18_SWEN_SLPW {
        _PWR_VDD18_SWEN_SLPW { w: self }
    }
    #[doc = "Bit 27 - Enable TVDD12 switching during Run Mode"]
    #[inline]
    pub fn pwr_tvdd12_swen_run(&mut self) -> _PWR_TVDD12_SWEN_RUNW {
        _PWR_TVDD12_SWEN_RUNW { w: self }
    }
    #[doc = "Bit 28 - Enable TVDD12 switching during Sleep Mode"]
    #[inline]
    pub fn pwr_tvdd12_swen_slp(&mut self) -> _PWR_TVDD12_SWEN_SLPW {
        _PWR_TVDD12_SWEN_SLPW { w: self }
    }
}
