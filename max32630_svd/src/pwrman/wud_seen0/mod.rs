#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WUD_SEEN0 {
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
#[doc = "Possible values of the field `gpio0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO0R {
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
            GPIO0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO0R {
        match value {
            i => GPIO0R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO1R {
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
            GPIO1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO1R {
        match value {
            i => GPIO1R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO2R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO2R {
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
            GPIO2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO2R {
        match value {
            i => GPIO2R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO3R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO3R {
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
            GPIO3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO3R {
        match value {
            i => GPIO3R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO4R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO4R {
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
            GPIO4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO4R {
        match value {
            i => GPIO4R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO5R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO5R {
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
            GPIO5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO5R {
        match value {
            i => GPIO5R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO6R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO6R {
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
            GPIO6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO6R {
        match value {
            i => GPIO6R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO7R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO7R {
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
            GPIO7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO7R {
        match value {
            i => GPIO7R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO8R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO8R {
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
            GPIO8R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO8R {
        match value {
            i => GPIO8R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO9R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO9R {
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
            GPIO9R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO9R {
        match value {
            i => GPIO9R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO10R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO10R {
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
            GPIO10R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO10R {
        match value {
            i => GPIO10R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO11R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO11R {
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
            GPIO11R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO11R {
        match value {
            i => GPIO11R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO12R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO12R {
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
            GPIO12R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO12R {
        match value {
            i => GPIO12R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO13R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO13R {
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
            GPIO13R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO13R {
        match value {
            i => GPIO13R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO14R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO14R {
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
            GPIO14R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO14R {
        match value {
            i => GPIO14R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO15R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO15R {
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
            GPIO15R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO15R {
        match value {
            i => GPIO15R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO16R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO16R {
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
            GPIO16R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO16R {
        match value {
            i => GPIO16R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO17R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO17R {
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
            GPIO17R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO17R {
        match value {
            i => GPIO17R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO18R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO18R {
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
            GPIO18R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO18R {
        match value {
            i => GPIO18R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO19R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO19R {
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
            GPIO19R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO19R {
        match value {
            i => GPIO19R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO20R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO20R {
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
            GPIO20R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO20R {
        match value {
            i => GPIO20R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO21R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO21R {
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
            GPIO21R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO21R {
        match value {
            i => GPIO21R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO22R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO22R {
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
            GPIO22R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO22R {
        match value {
            i => GPIO22R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO23R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO23R {
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
            GPIO23R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO23R {
        match value {
            i => GPIO23R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO24R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO24R {
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
            GPIO24R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO24R {
        match value {
            i => GPIO24R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO25R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO25R {
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
            GPIO25R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO25R {
        match value {
            i => GPIO25R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO26R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO26R {
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
            GPIO26R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO26R {
        match value {
            i => GPIO26R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO27R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO27R {
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
            GPIO27R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO27R {
        match value {
            i => GPIO27R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO28R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO28R {
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
            GPIO28R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO28R {
        match value {
            i => GPIO28R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO29R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO29R {
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
            GPIO29R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO29R {
        match value {
            i => GPIO29R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO30R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO30R {
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
            GPIO30R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO30R {
        match value {
            i => GPIO30R::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO31R {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl GPIO31R {
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
            GPIO31R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GPIO31R {
        match value {
            i => GPIO31R::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Wake-Up Detect Status for P0.0"]
    #[inline]
    pub fn gpio0(&self) -> GPIO0R {
        GPIO0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wake-Up Detect Status for P0.1"]
    #[inline]
    pub fn gpio1(&self) -> GPIO1R {
        GPIO1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Wake-Up Detect Status for P0.2"]
    #[inline]
    pub fn gpio2(&self) -> GPIO2R {
        GPIO2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Wake-Up Detect Status for P0.3"]
    #[inline]
    pub fn gpio3(&self) -> GPIO3R {
        GPIO3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Wake-Up Detect Status for P0.4"]
    #[inline]
    pub fn gpio4(&self) -> GPIO4R {
        GPIO4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Wake-Up Detect Status for P0.5"]
    #[inline]
    pub fn gpio5(&self) -> GPIO5R {
        GPIO5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Wake-Up Detect Status for P0.6"]
    #[inline]
    pub fn gpio6(&self) -> GPIO6R {
        GPIO6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Wake-Up Detect Status for P0.7"]
    #[inline]
    pub fn gpio7(&self) -> GPIO7R {
        GPIO7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Wake-Up Detect Status for P1.0"]
    #[inline]
    pub fn gpio8(&self) -> GPIO8R {
        GPIO8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Wake-Up Detect Status for P1.1"]
    #[inline]
    pub fn gpio9(&self) -> GPIO9R {
        GPIO9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Wake-Up Detect Status for P1.2"]
    #[inline]
    pub fn gpio10(&self) -> GPIO10R {
        GPIO10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Wake-Up Detect Status for P1.3"]
    #[inline]
    pub fn gpio11(&self) -> GPIO11R {
        GPIO11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Wake-Up Detect Status for P1.4"]
    #[inline]
    pub fn gpio12(&self) -> GPIO12R {
        GPIO12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Wake-Up Detect Status for P1.5"]
    #[inline]
    pub fn gpio13(&self) -> GPIO13R {
        GPIO13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Wake-Up Detect Status for P1.6"]
    #[inline]
    pub fn gpio14(&self) -> GPIO14R {
        GPIO14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Wake-Up Detect Status for P1.7"]
    #[inline]
    pub fn gpio15(&self) -> GPIO15R {
        GPIO15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Wake-Up Detect Status for P2.0"]
    #[inline]
    pub fn gpio16(&self) -> GPIO16R {
        GPIO16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Wake-Up Detect Status for P2.1"]
    #[inline]
    pub fn gpio17(&self) -> GPIO17R {
        GPIO17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Wake-Up Detect Status for P2.2"]
    #[inline]
    pub fn gpio18(&self) -> GPIO18R {
        GPIO18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Wake-Up Detect Status for P2.3"]
    #[inline]
    pub fn gpio19(&self) -> GPIO19R {
        GPIO19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Wake-Up Detect Status for P2.4"]
    #[inline]
    pub fn gpio20(&self) -> GPIO20R {
        GPIO20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Wake-Up Detect Status for P2.5"]
    #[inline]
    pub fn gpio21(&self) -> GPIO21R {
        GPIO21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Wake-Up Detect Status for P2.6"]
    #[inline]
    pub fn gpio22(&self) -> GPIO22R {
        GPIO22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Wake-Up Detect Status for P2.7"]
    #[inline]
    pub fn gpio23(&self) -> GPIO23R {
        GPIO23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Wake-Up Detect Status for P3.0"]
    #[inline]
    pub fn gpio24(&self) -> GPIO24R {
        GPIO24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Wake-Up Detect Status for P3.1"]
    #[inline]
    pub fn gpio25(&self) -> GPIO25R {
        GPIO25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Wake-Up Detect Status for P3.2"]
    #[inline]
    pub fn gpio26(&self) -> GPIO26R {
        GPIO26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Wake-Up Detect Status for P3.3"]
    #[inline]
    pub fn gpio27(&self) -> GPIO27R {
        GPIO27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Wake-Up Detect Status for P3.4"]
    #[inline]
    pub fn gpio28(&self) -> GPIO28R {
        GPIO28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Wake-Up Detect Status for P3.5"]
    #[inline]
    pub fn gpio29(&self) -> GPIO29R {
        GPIO29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Wake-Up Detect Status for P3.6"]
    #[inline]
    pub fn gpio30(&self) -> GPIO30R {
        GPIO30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Wake-Up Detect Status for P3.7"]
    #[inline]
    pub fn gpio31(&self) -> GPIO31R {
        GPIO31R::_from({
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
}
