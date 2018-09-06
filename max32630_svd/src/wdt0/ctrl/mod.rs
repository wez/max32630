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
#[doc = "Possible values of the field `int_period`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INT_PERIODR {
    #[doc = "2^31 WDT clocks"]
    _2_31_CLKS,
    #[doc = "2^30 WDT clocks"]
    _2_30_CLKS,
    #[doc = "2^29 WDT clocks"]
    _2_29_CLKS,
    #[doc = "2^28 WDT clocks"]
    _2_28_CLKS,
    #[doc = "2^27 WDT clocks"]
    _2_27_CLKS,
    #[doc = "2^26 WDT clocks"]
    _2_26_CLKS,
    #[doc = "2^25 WDT clocks"]
    _2_25_CLKS,
    #[doc = "2^24 WDT clocks"]
    _2_24_CLKS,
    #[doc = "2^23 WDT clocks"]
    _2_23_CLKS,
    #[doc = "2^22 WDT clocks"]
    _2_22_CLKS,
    #[doc = "2^21 WDT clocks"]
    _2_21_CLKS,
    #[doc = "2^20 WDT clocks"]
    _2_20_CLKS,
    #[doc = "2^19 WDT clocks"]
    _2_19_CLKS,
    #[doc = "2^18 WDT clocks"]
    _2_18_CLKS,
    #[doc = "2^17 WDT clocks"]
    _2_17_CLKS,
    #[doc = "2^16 WDT clocks"]
    _2_16_CLKS,
}
impl INT_PERIODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INT_PERIODR::_2_31_CLKS => 0,
            INT_PERIODR::_2_30_CLKS => 1,
            INT_PERIODR::_2_29_CLKS => 2,
            INT_PERIODR::_2_28_CLKS => 3,
            INT_PERIODR::_2_27_CLKS => 4,
            INT_PERIODR::_2_26_CLKS => 5,
            INT_PERIODR::_2_25_CLKS => 6,
            INT_PERIODR::_2_24_CLKS => 7,
            INT_PERIODR::_2_23_CLKS => 8,
            INT_PERIODR::_2_22_CLKS => 9,
            INT_PERIODR::_2_21_CLKS => 10,
            INT_PERIODR::_2_20_CLKS => 11,
            INT_PERIODR::_2_19_CLKS => 12,
            INT_PERIODR::_2_18_CLKS => 13,
            INT_PERIODR::_2_17_CLKS => 14,
            INT_PERIODR::_2_16_CLKS => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INT_PERIODR {
        match value {
            0 => INT_PERIODR::_2_31_CLKS,
            1 => INT_PERIODR::_2_30_CLKS,
            2 => INT_PERIODR::_2_29_CLKS,
            3 => INT_PERIODR::_2_28_CLKS,
            4 => INT_PERIODR::_2_27_CLKS,
            5 => INT_PERIODR::_2_26_CLKS,
            6 => INT_PERIODR::_2_25_CLKS,
            7 => INT_PERIODR::_2_24_CLKS,
            8 => INT_PERIODR::_2_23_CLKS,
            9 => INT_PERIODR::_2_22_CLKS,
            10 => INT_PERIODR::_2_21_CLKS,
            11 => INT_PERIODR::_2_20_CLKS,
            12 => INT_PERIODR::_2_19_CLKS,
            13 => INT_PERIODR::_2_18_CLKS,
            14 => INT_PERIODR::_2_17_CLKS,
            15 => INT_PERIODR::_2_16_CLKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2_31_CLKS`"]
    #[inline]
    pub fn is_2_31_clks(&self) -> bool {
        *self == INT_PERIODR::_2_31_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_30_CLKS`"]
    #[inline]
    pub fn is_2_30_clks(&self) -> bool {
        *self == INT_PERIODR::_2_30_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_29_CLKS`"]
    #[inline]
    pub fn is_2_29_clks(&self) -> bool {
        *self == INT_PERIODR::_2_29_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_28_CLKS`"]
    #[inline]
    pub fn is_2_28_clks(&self) -> bool {
        *self == INT_PERIODR::_2_28_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_27_CLKS`"]
    #[inline]
    pub fn is_2_27_clks(&self) -> bool {
        *self == INT_PERIODR::_2_27_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_26_CLKS`"]
    #[inline]
    pub fn is_2_26_clks(&self) -> bool {
        *self == INT_PERIODR::_2_26_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_25_CLKS`"]
    #[inline]
    pub fn is_2_25_clks(&self) -> bool {
        *self == INT_PERIODR::_2_25_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_24_CLKS`"]
    #[inline]
    pub fn is_2_24_clks(&self) -> bool {
        *self == INT_PERIODR::_2_24_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_23_CLKS`"]
    #[inline]
    pub fn is_2_23_clks(&self) -> bool {
        *self == INT_PERIODR::_2_23_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_22_CLKS`"]
    #[inline]
    pub fn is_2_22_clks(&self) -> bool {
        *self == INT_PERIODR::_2_22_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_21_CLKS`"]
    #[inline]
    pub fn is_2_21_clks(&self) -> bool {
        *self == INT_PERIODR::_2_21_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_20_CLKS`"]
    #[inline]
    pub fn is_2_20_clks(&self) -> bool {
        *self == INT_PERIODR::_2_20_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_19_CLKS`"]
    #[inline]
    pub fn is_2_19_clks(&self) -> bool {
        *self == INT_PERIODR::_2_19_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_18_CLKS`"]
    #[inline]
    pub fn is_2_18_clks(&self) -> bool {
        *self == INT_PERIODR::_2_18_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_17_CLKS`"]
    #[inline]
    pub fn is_2_17_clks(&self) -> bool {
        *self == INT_PERIODR::_2_17_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_16_CLKS`"]
    #[inline]
    pub fn is_2_16_clks(&self) -> bool {
        *self == INT_PERIODR::_2_16_CLKS
    }
}
#[doc = "Possible values of the field `rst_period`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_PERIODR {
    #[doc = "2^31 WDT clocks."]
    _2_31_CLKS,
    #[doc = "2^30 WDT clocks."]
    _2_30_CLKS,
    #[doc = "2^29 WDT clocks."]
    _2_29_CLKS,
    #[doc = "2^28 WDT clocks."]
    _2_28_CLKS,
    #[doc = "2^27 WDT clocks."]
    _2_27_CLKS,
    #[doc = "2^26 WDT clocks."]
    _2_26_CLKS,
    #[doc = "2^25 WDT clocks."]
    _2_25_CLKS,
    #[doc = "2^24 WDT clocks."]
    _2_24_CLKS,
    #[doc = "2^23 WDT clocks."]
    _2_23_CLKS,
    #[doc = "2^22 WDT clocks."]
    _2_22_CLKS,
    #[doc = "2^21 WDT clocks."]
    _2_21_CLKS,
    #[doc = "2^20 WDT clocks."]
    _2_20_CLKS,
    #[doc = "2^19 WDT clocks."]
    _2_19_CLKS,
    #[doc = "2^18 WDT clocks."]
    _2_18_CLKS,
    #[doc = "2^17 WDT clocks."]
    _2_17_CLKS,
    #[doc = "2^16 WDT clocks."]
    _2_16_CLKS,
}
impl RST_PERIODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RST_PERIODR::_2_31_CLKS => 0,
            RST_PERIODR::_2_30_CLKS => 1,
            RST_PERIODR::_2_29_CLKS => 2,
            RST_PERIODR::_2_28_CLKS => 3,
            RST_PERIODR::_2_27_CLKS => 4,
            RST_PERIODR::_2_26_CLKS => 5,
            RST_PERIODR::_2_25_CLKS => 6,
            RST_PERIODR::_2_24_CLKS => 7,
            RST_PERIODR::_2_23_CLKS => 8,
            RST_PERIODR::_2_22_CLKS => 9,
            RST_PERIODR::_2_21_CLKS => 10,
            RST_PERIODR::_2_20_CLKS => 11,
            RST_PERIODR::_2_19_CLKS => 12,
            RST_PERIODR::_2_18_CLKS => 13,
            RST_PERIODR::_2_17_CLKS => 14,
            RST_PERIODR::_2_16_CLKS => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RST_PERIODR {
        match value {
            0 => RST_PERIODR::_2_31_CLKS,
            1 => RST_PERIODR::_2_30_CLKS,
            2 => RST_PERIODR::_2_29_CLKS,
            3 => RST_PERIODR::_2_28_CLKS,
            4 => RST_PERIODR::_2_27_CLKS,
            5 => RST_PERIODR::_2_26_CLKS,
            6 => RST_PERIODR::_2_25_CLKS,
            7 => RST_PERIODR::_2_24_CLKS,
            8 => RST_PERIODR::_2_23_CLKS,
            9 => RST_PERIODR::_2_22_CLKS,
            10 => RST_PERIODR::_2_21_CLKS,
            11 => RST_PERIODR::_2_20_CLKS,
            12 => RST_PERIODR::_2_19_CLKS,
            13 => RST_PERIODR::_2_18_CLKS,
            14 => RST_PERIODR::_2_17_CLKS,
            15 => RST_PERIODR::_2_16_CLKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2_31_CLKS`"]
    #[inline]
    pub fn is_2_31_clks(&self) -> bool {
        *self == RST_PERIODR::_2_31_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_30_CLKS`"]
    #[inline]
    pub fn is_2_30_clks(&self) -> bool {
        *self == RST_PERIODR::_2_30_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_29_CLKS`"]
    #[inline]
    pub fn is_2_29_clks(&self) -> bool {
        *self == RST_PERIODR::_2_29_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_28_CLKS`"]
    #[inline]
    pub fn is_2_28_clks(&self) -> bool {
        *self == RST_PERIODR::_2_28_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_27_CLKS`"]
    #[inline]
    pub fn is_2_27_clks(&self) -> bool {
        *self == RST_PERIODR::_2_27_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_26_CLKS`"]
    #[inline]
    pub fn is_2_26_clks(&self) -> bool {
        *self == RST_PERIODR::_2_26_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_25_CLKS`"]
    #[inline]
    pub fn is_2_25_clks(&self) -> bool {
        *self == RST_PERIODR::_2_25_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_24_CLKS`"]
    #[inline]
    pub fn is_2_24_clks(&self) -> bool {
        *self == RST_PERIODR::_2_24_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_23_CLKS`"]
    #[inline]
    pub fn is_2_23_clks(&self) -> bool {
        *self == RST_PERIODR::_2_23_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_22_CLKS`"]
    #[inline]
    pub fn is_2_22_clks(&self) -> bool {
        *self == RST_PERIODR::_2_22_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_21_CLKS`"]
    #[inline]
    pub fn is_2_21_clks(&self) -> bool {
        *self == RST_PERIODR::_2_21_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_20_CLKS`"]
    #[inline]
    pub fn is_2_20_clks(&self) -> bool {
        *self == RST_PERIODR::_2_20_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_19_CLKS`"]
    #[inline]
    pub fn is_2_19_clks(&self) -> bool {
        *self == RST_PERIODR::_2_19_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_18_CLKS`"]
    #[inline]
    pub fn is_2_18_clks(&self) -> bool {
        *self == RST_PERIODR::_2_18_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_17_CLKS`"]
    #[inline]
    pub fn is_2_17_clks(&self) -> bool {
        *self == RST_PERIODR::_2_17_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_16_CLKS`"]
    #[inline]
    pub fn is_2_16_clks(&self) -> bool {
        *self == RST_PERIODR::_2_16_CLKS
    }
}
#[doc = "Possible values of the field `en_timer`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_TIMERR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl EN_TIMERR {
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
            EN_TIMERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_TIMERR {
        match value {
            i => EN_TIMERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `en_clock`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_CLOCKR {
    #[doc = "WDT Clock Gate Control Disable"]
    DISABLE,
    #[doc = "WDT Clock Gate Control Enable"]
    ENABLE,
}
impl EN_CLOCKR {
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
            EN_CLOCKR::DISABLE => false,
            EN_CLOCKR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EN_CLOCKR {
        match value {
            false => EN_CLOCKR::DISABLE,
            true => EN_CLOCKR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == EN_CLOCKR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == EN_CLOCKR::ENABLE
    }
}
#[doc = "Possible values of the field `wait_period`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAIT_PERIODR {
    #[doc = "2^31 WDT clocks."]
    _2_31_CLKS,
    #[doc = "2^30 WDT clocks."]
    _2_30_CLKS,
    #[doc = "2^29 WDT clocks."]
    _2_29_CLKS,
    #[doc = "2^28 WDT clocks."]
    _2_28_CLKS,
    #[doc = "2^27 WDT clocks"]
    _2_27_CLKS,
    #[doc = "2^26 WDT clocks"]
    _2_26_CLKS,
    #[doc = "2^25 WDT clocks"]
    _2_25_CLKS,
    #[doc = "2^24 WDT clocks"]
    _2_24_CLKS,
    #[doc = "2^23 WDT clocks"]
    _2_23_CLKS,
    #[doc = "2^22 WDT clocks."]
    _2_22_CLKS,
    #[doc = "2^21 WDT clocks."]
    _2_21_CLKS,
    #[doc = "2^20 WDT clocks."]
    _2_20_CLKS,
    #[doc = "2^19 WDT clocks."]
    _2_19_CLKS,
    #[doc = "2^18 WDT clocks."]
    _2_18_CLKS,
    #[doc = "2^17 WDT clocks."]
    _2_17_CLKS,
    #[doc = "2^16 WDT clocks."]
    _2_16_CLKS,
}
impl WAIT_PERIODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAIT_PERIODR::_2_31_CLKS => 0,
            WAIT_PERIODR::_2_30_CLKS => 1,
            WAIT_PERIODR::_2_29_CLKS => 2,
            WAIT_PERIODR::_2_28_CLKS => 3,
            WAIT_PERIODR::_2_27_CLKS => 4,
            WAIT_PERIODR::_2_26_CLKS => 5,
            WAIT_PERIODR::_2_25_CLKS => 6,
            WAIT_PERIODR::_2_24_CLKS => 7,
            WAIT_PERIODR::_2_23_CLKS => 8,
            WAIT_PERIODR::_2_22_CLKS => 9,
            WAIT_PERIODR::_2_21_CLKS => 10,
            WAIT_PERIODR::_2_20_CLKS => 11,
            WAIT_PERIODR::_2_19_CLKS => 12,
            WAIT_PERIODR::_2_18_CLKS => 13,
            WAIT_PERIODR::_2_17_CLKS => 14,
            WAIT_PERIODR::_2_16_CLKS => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAIT_PERIODR {
        match value {
            0 => WAIT_PERIODR::_2_31_CLKS,
            1 => WAIT_PERIODR::_2_30_CLKS,
            2 => WAIT_PERIODR::_2_29_CLKS,
            3 => WAIT_PERIODR::_2_28_CLKS,
            4 => WAIT_PERIODR::_2_27_CLKS,
            5 => WAIT_PERIODR::_2_26_CLKS,
            6 => WAIT_PERIODR::_2_25_CLKS,
            7 => WAIT_PERIODR::_2_24_CLKS,
            8 => WAIT_PERIODR::_2_23_CLKS,
            9 => WAIT_PERIODR::_2_22_CLKS,
            10 => WAIT_PERIODR::_2_21_CLKS,
            11 => WAIT_PERIODR::_2_20_CLKS,
            12 => WAIT_PERIODR::_2_19_CLKS,
            13 => WAIT_PERIODR::_2_18_CLKS,
            14 => WAIT_PERIODR::_2_17_CLKS,
            15 => WAIT_PERIODR::_2_16_CLKS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2_31_CLKS`"]
    #[inline]
    pub fn is_2_31_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_31_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_30_CLKS`"]
    #[inline]
    pub fn is_2_30_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_30_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_29_CLKS`"]
    #[inline]
    pub fn is_2_29_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_29_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_28_CLKS`"]
    #[inline]
    pub fn is_2_28_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_28_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_27_CLKS`"]
    #[inline]
    pub fn is_2_27_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_27_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_26_CLKS`"]
    #[inline]
    pub fn is_2_26_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_26_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_25_CLKS`"]
    #[inline]
    pub fn is_2_25_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_25_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_24_CLKS`"]
    #[inline]
    pub fn is_2_24_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_24_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_23_CLKS`"]
    #[inline]
    pub fn is_2_23_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_23_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_22_CLKS`"]
    #[inline]
    pub fn is_2_22_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_22_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_21_CLKS`"]
    #[inline]
    pub fn is_2_21_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_21_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_20_CLKS`"]
    #[inline]
    pub fn is_2_20_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_20_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_19_CLKS`"]
    #[inline]
    pub fn is_2_19_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_19_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_18_CLKS`"]
    #[inline]
    pub fn is_2_18_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_18_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_17_CLKS`"]
    #[inline]
    pub fn is_2_17_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_17_CLKS
    }
    #[doc = "Checks if the value of the field is `_2_16_CLKS`"]
    #[inline]
    pub fn is_2_16_clks(&self) -> bool {
        *self == WAIT_PERIODR::_2_16_CLKS
    }
}
#[doc = "Values that can be written to the field `int_period`"]
pub enum INT_PERIODW {
    #[doc = "2^31 WDT clocks"]
    _2_31_CLKS,
    #[doc = "2^30 WDT clocks"]
    _2_30_CLKS,
    #[doc = "2^29 WDT clocks"]
    _2_29_CLKS,
    #[doc = "2^28 WDT clocks"]
    _2_28_CLKS,
    #[doc = "2^27 WDT clocks"]
    _2_27_CLKS,
    #[doc = "2^26 WDT clocks"]
    _2_26_CLKS,
    #[doc = "2^25 WDT clocks"]
    _2_25_CLKS,
    #[doc = "2^24 WDT clocks"]
    _2_24_CLKS,
    #[doc = "2^23 WDT clocks"]
    _2_23_CLKS,
    #[doc = "2^22 WDT clocks"]
    _2_22_CLKS,
    #[doc = "2^21 WDT clocks"]
    _2_21_CLKS,
    #[doc = "2^20 WDT clocks"]
    _2_20_CLKS,
    #[doc = "2^19 WDT clocks"]
    _2_19_CLKS,
    #[doc = "2^18 WDT clocks"]
    _2_18_CLKS,
    #[doc = "2^17 WDT clocks"]
    _2_17_CLKS,
    #[doc = "2^16 WDT clocks"]
    _2_16_CLKS,
}
impl INT_PERIODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INT_PERIODW::_2_31_CLKS => 0,
            INT_PERIODW::_2_30_CLKS => 1,
            INT_PERIODW::_2_29_CLKS => 2,
            INT_PERIODW::_2_28_CLKS => 3,
            INT_PERIODW::_2_27_CLKS => 4,
            INT_PERIODW::_2_26_CLKS => 5,
            INT_PERIODW::_2_25_CLKS => 6,
            INT_PERIODW::_2_24_CLKS => 7,
            INT_PERIODW::_2_23_CLKS => 8,
            INT_PERIODW::_2_22_CLKS => 9,
            INT_PERIODW::_2_21_CLKS => 10,
            INT_PERIODW::_2_20_CLKS => 11,
            INT_PERIODW::_2_19_CLKS => 12,
            INT_PERIODW::_2_18_CLKS => 13,
            INT_PERIODW::_2_17_CLKS => 14,
            INT_PERIODW::_2_16_CLKS => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INT_PERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _INT_PERIODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INT_PERIODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "2^31 WDT clocks"]
    #[inline]
    pub fn _2_31_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_31_CLKS)
    }
    #[doc = "2^30 WDT clocks"]
    #[inline]
    pub fn _2_30_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_30_CLKS)
    }
    #[doc = "2^29 WDT clocks"]
    #[inline]
    pub fn _2_29_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_29_CLKS)
    }
    #[doc = "2^28 WDT clocks"]
    #[inline]
    pub fn _2_28_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_28_CLKS)
    }
    #[doc = "2^27 WDT clocks"]
    #[inline]
    pub fn _2_27_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_27_CLKS)
    }
    #[doc = "2^26 WDT clocks"]
    #[inline]
    pub fn _2_26_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_26_CLKS)
    }
    #[doc = "2^25 WDT clocks"]
    #[inline]
    pub fn _2_25_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_25_CLKS)
    }
    #[doc = "2^24 WDT clocks"]
    #[inline]
    pub fn _2_24_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_24_CLKS)
    }
    #[doc = "2^23 WDT clocks"]
    #[inline]
    pub fn _2_23_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_23_CLKS)
    }
    #[doc = "2^22 WDT clocks"]
    #[inline]
    pub fn _2_22_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_22_CLKS)
    }
    #[doc = "2^21 WDT clocks"]
    #[inline]
    pub fn _2_21_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_21_CLKS)
    }
    #[doc = "2^20 WDT clocks"]
    #[inline]
    pub fn _2_20_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_20_CLKS)
    }
    #[doc = "2^19 WDT clocks"]
    #[inline]
    pub fn _2_19_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_19_CLKS)
    }
    #[doc = "2^18 WDT clocks"]
    #[inline]
    pub fn _2_18_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_18_CLKS)
    }
    #[doc = "2^17 WDT clocks"]
    #[inline]
    pub fn _2_17_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_17_CLKS)
    }
    #[doc = "2^16 WDT clocks"]
    #[inline]
    pub fn _2_16_clks(self) -> &'a mut W {
        self.variant(INT_PERIODW::_2_16_CLKS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `rst_period`"]
pub enum RST_PERIODW {
    #[doc = "2^31 WDT clocks."]
    _2_31_CLKS,
    #[doc = "2^30 WDT clocks."]
    _2_30_CLKS,
    #[doc = "2^29 WDT clocks."]
    _2_29_CLKS,
    #[doc = "2^28 WDT clocks."]
    _2_28_CLKS,
    #[doc = "2^27 WDT clocks."]
    _2_27_CLKS,
    #[doc = "2^26 WDT clocks."]
    _2_26_CLKS,
    #[doc = "2^25 WDT clocks."]
    _2_25_CLKS,
    #[doc = "2^24 WDT clocks."]
    _2_24_CLKS,
    #[doc = "2^23 WDT clocks."]
    _2_23_CLKS,
    #[doc = "2^22 WDT clocks."]
    _2_22_CLKS,
    #[doc = "2^21 WDT clocks."]
    _2_21_CLKS,
    #[doc = "2^20 WDT clocks."]
    _2_20_CLKS,
    #[doc = "2^19 WDT clocks."]
    _2_19_CLKS,
    #[doc = "2^18 WDT clocks."]
    _2_18_CLKS,
    #[doc = "2^17 WDT clocks."]
    _2_17_CLKS,
    #[doc = "2^16 WDT clocks."]
    _2_16_CLKS,
}
impl RST_PERIODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RST_PERIODW::_2_31_CLKS => 0,
            RST_PERIODW::_2_30_CLKS => 1,
            RST_PERIODW::_2_29_CLKS => 2,
            RST_PERIODW::_2_28_CLKS => 3,
            RST_PERIODW::_2_27_CLKS => 4,
            RST_PERIODW::_2_26_CLKS => 5,
            RST_PERIODW::_2_25_CLKS => 6,
            RST_PERIODW::_2_24_CLKS => 7,
            RST_PERIODW::_2_23_CLKS => 8,
            RST_PERIODW::_2_22_CLKS => 9,
            RST_PERIODW::_2_21_CLKS => 10,
            RST_PERIODW::_2_20_CLKS => 11,
            RST_PERIODW::_2_19_CLKS => 12,
            RST_PERIODW::_2_18_CLKS => 13,
            RST_PERIODW::_2_17_CLKS => 14,
            RST_PERIODW::_2_16_CLKS => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RST_PERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _RST_PERIODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RST_PERIODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "2^31 WDT clocks."]
    #[inline]
    pub fn _2_31_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_31_CLKS)
    }
    #[doc = "2^30 WDT clocks."]
    #[inline]
    pub fn _2_30_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_30_CLKS)
    }
    #[doc = "2^29 WDT clocks."]
    #[inline]
    pub fn _2_29_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_29_CLKS)
    }
    #[doc = "2^28 WDT clocks."]
    #[inline]
    pub fn _2_28_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_28_CLKS)
    }
    #[doc = "2^27 WDT clocks."]
    #[inline]
    pub fn _2_27_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_27_CLKS)
    }
    #[doc = "2^26 WDT clocks."]
    #[inline]
    pub fn _2_26_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_26_CLKS)
    }
    #[doc = "2^25 WDT clocks."]
    #[inline]
    pub fn _2_25_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_25_CLKS)
    }
    #[doc = "2^24 WDT clocks."]
    #[inline]
    pub fn _2_24_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_24_CLKS)
    }
    #[doc = "2^23 WDT clocks."]
    #[inline]
    pub fn _2_23_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_23_CLKS)
    }
    #[doc = "2^22 WDT clocks."]
    #[inline]
    pub fn _2_22_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_22_CLKS)
    }
    #[doc = "2^21 WDT clocks."]
    #[inline]
    pub fn _2_21_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_21_CLKS)
    }
    #[doc = "2^20 WDT clocks."]
    #[inline]
    pub fn _2_20_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_20_CLKS)
    }
    #[doc = "2^19 WDT clocks."]
    #[inline]
    pub fn _2_19_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_19_CLKS)
    }
    #[doc = "2^18 WDT clocks."]
    #[inline]
    pub fn _2_18_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_18_CLKS)
    }
    #[doc = "2^17 WDT clocks."]
    #[inline]
    pub fn _2_17_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_17_CLKS)
    }
    #[doc = "2^16 WDT clocks."]
    #[inline]
    pub fn _2_16_clks(self) -> &'a mut W {
        self.variant(RST_PERIODW::_2_16_CLKS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `en_timer`"]
pub enum EN_TIMERW {}
impl EN_TIMERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _EN_TIMERW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_TIMERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_TIMERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `en_clock`"]
pub enum EN_CLOCKW {
    #[doc = "WDT Clock Gate Control Disable"]
    DISABLE,
    #[doc = "WDT Clock Gate Control Enable"]
    ENABLE,
}
impl EN_CLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EN_CLOCKW::DISABLE => false,
            EN_CLOCKW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EN_CLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _EN_CLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EN_CLOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "WDT Clock Gate Control Disable"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_CLOCKW::DISABLE)
    }
    #[doc = "WDT Clock Gate Control Enable"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_CLOCKW::ENABLE)
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
#[doc = "Values that can be written to the field `wait_period`"]
pub enum WAIT_PERIODW {
    #[doc = "2^31 WDT clocks."]
    _2_31_CLKS,
    #[doc = "2^30 WDT clocks."]
    _2_30_CLKS,
    #[doc = "2^29 WDT clocks."]
    _2_29_CLKS,
    #[doc = "2^28 WDT clocks."]
    _2_28_CLKS,
    #[doc = "2^27 WDT clocks"]
    _2_27_CLKS,
    #[doc = "2^26 WDT clocks"]
    _2_26_CLKS,
    #[doc = "2^25 WDT clocks"]
    _2_25_CLKS,
    #[doc = "2^24 WDT clocks"]
    _2_24_CLKS,
    #[doc = "2^23 WDT clocks"]
    _2_23_CLKS,
    #[doc = "2^22 WDT clocks."]
    _2_22_CLKS,
    #[doc = "2^21 WDT clocks."]
    _2_21_CLKS,
    #[doc = "2^20 WDT clocks."]
    _2_20_CLKS,
    #[doc = "2^19 WDT clocks."]
    _2_19_CLKS,
    #[doc = "2^18 WDT clocks."]
    _2_18_CLKS,
    #[doc = "2^17 WDT clocks."]
    _2_17_CLKS,
    #[doc = "2^16 WDT clocks."]
    _2_16_CLKS,
}
impl WAIT_PERIODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAIT_PERIODW::_2_31_CLKS => 0,
            WAIT_PERIODW::_2_30_CLKS => 1,
            WAIT_PERIODW::_2_29_CLKS => 2,
            WAIT_PERIODW::_2_28_CLKS => 3,
            WAIT_PERIODW::_2_27_CLKS => 4,
            WAIT_PERIODW::_2_26_CLKS => 5,
            WAIT_PERIODW::_2_25_CLKS => 6,
            WAIT_PERIODW::_2_24_CLKS => 7,
            WAIT_PERIODW::_2_23_CLKS => 8,
            WAIT_PERIODW::_2_22_CLKS => 9,
            WAIT_PERIODW::_2_21_CLKS => 10,
            WAIT_PERIODW::_2_20_CLKS => 11,
            WAIT_PERIODW::_2_19_CLKS => 12,
            WAIT_PERIODW::_2_18_CLKS => 13,
            WAIT_PERIODW::_2_17_CLKS => 14,
            WAIT_PERIODW::_2_16_CLKS => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAIT_PERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _WAIT_PERIODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAIT_PERIODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "2^31 WDT clocks."]
    #[inline]
    pub fn _2_31_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_31_CLKS)
    }
    #[doc = "2^30 WDT clocks."]
    #[inline]
    pub fn _2_30_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_30_CLKS)
    }
    #[doc = "2^29 WDT clocks."]
    #[inline]
    pub fn _2_29_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_29_CLKS)
    }
    #[doc = "2^28 WDT clocks."]
    #[inline]
    pub fn _2_28_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_28_CLKS)
    }
    #[doc = "2^27 WDT clocks"]
    #[inline]
    pub fn _2_27_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_27_CLKS)
    }
    #[doc = "2^26 WDT clocks"]
    #[inline]
    pub fn _2_26_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_26_CLKS)
    }
    #[doc = "2^25 WDT clocks"]
    #[inline]
    pub fn _2_25_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_25_CLKS)
    }
    #[doc = "2^24 WDT clocks"]
    #[inline]
    pub fn _2_24_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_24_CLKS)
    }
    #[doc = "2^23 WDT clocks"]
    #[inline]
    pub fn _2_23_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_23_CLKS)
    }
    #[doc = "2^22 WDT clocks."]
    #[inline]
    pub fn _2_22_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_22_CLKS)
    }
    #[doc = "2^21 WDT clocks."]
    #[inline]
    pub fn _2_21_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_21_CLKS)
    }
    #[doc = "2^20 WDT clocks."]
    #[inline]
    pub fn _2_20_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_20_CLKS)
    }
    #[doc = "2^19 WDT clocks."]
    #[inline]
    pub fn _2_19_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_19_CLKS)
    }
    #[doc = "2^18 WDT clocks."]
    #[inline]
    pub fn _2_18_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_18_CLKS)
    }
    #[doc = "2^17 WDT clocks."]
    #[inline]
    pub fn _2_17_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_17_CLKS)
    }
    #[doc = "2^16 WDT clocks."]
    #[inline]
    pub fn _2_16_clks(self) -> &'a mut W {
        self.variant(WAIT_PERIODW::_2_16_CLKS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:3 - Period from WDT Clear to Interrupt Flag Set"]
    #[inline]
    pub fn int_period(&self) -> INT_PERIODR {
        INT_PERIODR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Period from WDT Clear to Reset Flag Set"]
    #[inline]
    pub fn rst_period(&self) -> RST_PERIODR {
        RST_PERIODR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Watchdg Timer Enable"]
    #[inline]
    pub fn en_timer(&self) -> EN_TIMERR {
        EN_TIMERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Watchdog Clock Gate"]
    #[inline]
    pub fn en_clock(&self) -> EN_CLOCKR {
        EN_CLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:15 - Period from WDT Clear to Clear Window Begin"]
    #[inline]
    pub fn wait_period(&self) -> WAIT_PERIODR {
        WAIT_PERIODR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:3 - Period from WDT Clear to Interrupt Flag Set"]
    #[inline]
    pub fn int_period(&mut self) -> _INT_PERIODW {
        _INT_PERIODW { w: self }
    }
    #[doc = "Bits 4:7 - Period from WDT Clear to Reset Flag Set"]
    #[inline]
    pub fn rst_period(&mut self) -> _RST_PERIODW {
        _RST_PERIODW { w: self }
    }
    #[doc = "Bit 8 - Watchdg Timer Enable"]
    #[inline]
    pub fn en_timer(&mut self) -> _EN_TIMERW {
        _EN_TIMERW { w: self }
    }
    #[doc = "Bit 9 - Watchdog Clock Gate"]
    #[inline]
    pub fn en_clock(&mut self) -> _EN_CLOCKW {
        _EN_CLOCKW { w: self }
    }
    #[doc = "Bits 12:15 - Period from WDT Clear to Clear Window Begin"]
    #[inline]
    pub fn wait_period(&mut self) -> _WAIT_PERIODW {
        _WAIT_PERIODW { w: self }
    }
}
