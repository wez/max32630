#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WUD_SEEN1 {
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
#[doc = "Possible values of the field `gpio32`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO32R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO32R {
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
            GPIO32R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO32R {
        match value {
            i => GPIO32R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio33`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO33R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO33R {
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
            GPIO33R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO33R {
        match value {
            i => GPIO33R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio34`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO34R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO34R {
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
            GPIO34R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO34R {
        match value {
            i => GPIO34R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio35`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO35R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO35R {
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
            GPIO35R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO35R {
        match value {
            i => GPIO35R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio36`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO36R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO36R {
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
            GPIO36R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO36R {
        match value {
            i => GPIO36R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio37`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO37R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO37R {
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
            GPIO37R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO37R {
        match value {
            i => GPIO37R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio38`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO38R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO38R {
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
            GPIO38R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO38R {
        match value {
            i => GPIO38R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio39`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO39R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO39R {
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
            GPIO39R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO39R {
        match value {
            i => GPIO39R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio40`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO40R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO40R {
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
            GPIO40R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO40R {
        match value {
            i => GPIO40R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio41`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO41R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO41R {
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
            GPIO41R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO41R {
        match value {
            i => GPIO41R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio42`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO42R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO42R {
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
            GPIO42R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO42R {
        match value {
            i => GPIO42R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio43`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO43R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO43R {
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
            GPIO43R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO43R {
        match value {
            i => GPIO43R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio44`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO44R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO44R {
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
            GPIO44R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO44R {
        match value {
            i => GPIO44R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio45`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO45R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO45R {
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
            GPIO45R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO45R {
        match value {
            i => GPIO45R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio46`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO46R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO46R {
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
            GPIO46R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO46R {
        match value {
            i => GPIO46R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio47`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO47R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO47R {
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
            GPIO47R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO47R {
        match value {
            i => GPIO47R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio48`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO48R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO48R {
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
            GPIO48R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO48R {
        match value {
            i => GPIO48R::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Wake-Up Detect Status for P4.0"]
    #[inline]
    pub fn gpio32(&self) -> GPIO32R {
        GPIO32R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wake-Up Detect Status for P4.1"]
    #[inline]
    pub fn gpio33(&self) -> GPIO33R {
        GPIO33R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Wake-Up Detect Status for P4.2"]
    #[inline]
    pub fn gpio34(&self) -> GPIO34R {
        GPIO34R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Wake-Up Detect Status for P4.3"]
    #[inline]
    pub fn gpio35(&self) -> GPIO35R {
        GPIO35R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Wake-Up Detect Status for P4.4"]
    #[inline]
    pub fn gpio36(&self) -> GPIO36R {
        GPIO36R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Wake-Up Detect Status for P4.5"]
    #[inline]
    pub fn gpio37(&self) -> GPIO37R {
        GPIO37R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Wake-Up Detect Status for P4.6"]
    #[inline]
    pub fn gpio38(&self) -> GPIO38R {
        GPIO38R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Wake-Up Detect Status for P4.7"]
    #[inline]
    pub fn gpio39(&self) -> GPIO39R {
        GPIO39R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Wake-Up Detect Status for P5.0"]
    #[inline]
    pub fn gpio40(&self) -> GPIO40R {
        GPIO40R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Wake-Up Detect Status for P5.1"]
    #[inline]
    pub fn gpio41(&self) -> GPIO41R {
        GPIO41R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Wake-Up Detect Status for P5.2"]
    #[inline]
    pub fn gpio42(&self) -> GPIO42R {
        GPIO42R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Wake-Up Detect Status for P5.3"]
    #[inline]
    pub fn gpio43(&self) -> GPIO43R {
        GPIO43R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Wake-Up Detect Status for P5.4"]
    #[inline]
    pub fn gpio44(&self) -> GPIO44R {
        GPIO44R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Wake-Up Detect Status for P5.5"]
    #[inline]
    pub fn gpio45(&self) -> GPIO45R {
        GPIO45R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Wake-Up Detect Status for P5.6"]
    #[inline]
    pub fn gpio46(&self) -> GPIO46R {
        GPIO46R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Wake-Up Detect Status for P5.7"]
    #[inline]
    pub fn gpio47(&self) -> GPIO47R {
        GPIO47R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Wake-Up Detect Status for P6.0"]
    #[inline]
    pub fn gpio48(&self) -> GPIO48R {
        GPIO48R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
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
}
