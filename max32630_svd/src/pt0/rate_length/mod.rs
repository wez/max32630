#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RATE_LENGTH {
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
#[doc = "Possible values of the field `rate_control`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RATE_CONTROLR {
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl RATE_CONTROLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            RATE_CONTROLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> RATE_CONTROLR {
        match value {
            i => RATE_CONTROLR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Pulse train, 32 bit pattern."]
    _32_BIT,
    #[doc = "Square wave mode."]
    SQUARE_WAVE,
    #[doc = "Pulse train, 2 bit pattern."]
    _2_BIT,
    #[doc = "Pulse train, 3 bit pattern."]
    _3_BIT,
    #[doc = "Pulse train, 4 bit pattern."]
    _4_BIT,
    #[doc = "Pulse train, 5 bit pattern."]
    _5_BIT,
    #[doc = "Pulse train, 6 bit pattern."]
    _6_BIT,
    #[doc = "Pulse train, 7 bit pattern."]
    _7_BIT,
    #[doc = "Pulse train, 8 bit pattern."]
    _8_BIT,
    #[doc = "Pulse train, 9 bit pattern."]
    _9_BIT,
    #[doc = "Pulse train, 10 bit pattern."]
    _10_BIT,
    #[doc = "Pulse train, 11 bit pattern."]
    _11_BIT,
    #[doc = "Pulse train, 12 bit pattern."]
    _12_BIT,
    #[doc = "Pulse train, 13 bit pattern."]
    _13_BIT,
    #[doc = "Pulse train, 14 bit pattern."]
    _14_BIT,
    #[doc = "Pulse train, 15 bit pattern."]
    _15_BIT,
    #[doc = "Pulse train, 16 bit pattern."]
    _16_BIT,
    #[doc = "Pulse train, 17 bit pattern."]
    _17_BIT,
    #[doc = "Pulse train, 18 bit pattern."]
    _18_BIT,
    #[doc = "Pulse train, 19 bit pattern."]
    _19_BIT,
    #[doc = "Pulse train, 20 bit pattern."]
    _20_BIT,
    #[doc = "Pulse train, 21 bit pattern."]
    _21_BIT,
    #[doc = "Pulse train, 22 bit pattern."]
    _22_BIT,
    #[doc = "Pulse train, 23 bit pattern."]
    _23_BIT,
    #[doc = "Pulse train, 24 bit pattern."]
    _24_BIT,
    #[doc = "Pulse train, 25 bit pattern."]
    _25_BIT,
    #[doc = "Pulse train, 26 bit pattern."]
    _26_BIT,
    #[doc = "Pulse train, 27 bit pattern."]
    _27_BIT,
    #[doc = "Pulse train, 28 bit pattern."]
    _28_BIT,
    #[doc = "Pulse train, 29 bit pattern."]
    _29_BIT,
    #[doc = "Pulse train, 30 bit pattern."]
    _30_BIT,
    #[doc = "Pulse train, 31 bit pattern."]
    _31_BIT,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::_32_BIT => 0,
            MODER::SQUARE_WAVE => 1,
            MODER::_2_BIT => 2,
            MODER::_3_BIT => 3,
            MODER::_4_BIT => 4,
            MODER::_5_BIT => 5,
            MODER::_6_BIT => 6,
            MODER::_7_BIT => 7,
            MODER::_8_BIT => 8,
            MODER::_9_BIT => 9,
            MODER::_10_BIT => 10,
            MODER::_11_BIT => 11,
            MODER::_12_BIT => 12,
            MODER::_13_BIT => 13,
            MODER::_14_BIT => 14,
            MODER::_15_BIT => 15,
            MODER::_16_BIT => 16,
            MODER::_17_BIT => 17,
            MODER::_18_BIT => 18,
            MODER::_19_BIT => 19,
            MODER::_20_BIT => 20,
            MODER::_21_BIT => 21,
            MODER::_22_BIT => 22,
            MODER::_23_BIT => 23,
            MODER::_24_BIT => 24,
            MODER::_25_BIT => 25,
            MODER::_26_BIT => 26,
            MODER::_27_BIT => 27,
            MODER::_28_BIT => 28,
            MODER::_29_BIT => 29,
            MODER::_30_BIT => 30,
            MODER::_31_BIT => 31,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::_32_BIT,
            1 => MODER::SQUARE_WAVE,
            2 => MODER::_2_BIT,
            3 => MODER::_3_BIT,
            4 => MODER::_4_BIT,
            5 => MODER::_5_BIT,
            6 => MODER::_6_BIT,
            7 => MODER::_7_BIT,
            8 => MODER::_8_BIT,
            9 => MODER::_9_BIT,
            10 => MODER::_10_BIT,
            11 => MODER::_11_BIT,
            12 => MODER::_12_BIT,
            13 => MODER::_13_BIT,
            14 => MODER::_14_BIT,
            15 => MODER::_15_BIT,
            16 => MODER::_16_BIT,
            17 => MODER::_17_BIT,
            18 => MODER::_18_BIT,
            19 => MODER::_19_BIT,
            20 => MODER::_20_BIT,
            21 => MODER::_21_BIT,
            22 => MODER::_22_BIT,
            23 => MODER::_23_BIT,
            24 => MODER::_24_BIT,
            25 => MODER::_25_BIT,
            26 => MODER::_26_BIT,
            27 => MODER::_27_BIT,
            28 => MODER::_28_BIT,
            29 => MODER::_29_BIT,
            30 => MODER::_30_BIT,
            31 => MODER::_31_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32_BIT`"]
    #[inline]
    pub fn is_32_bit(&self) -> bool {
        *self == MODER::_32_BIT
    }
    #[doc = "Checks if the value of the field is `SQUARE_WAVE`"]
    #[inline]
    pub fn is_square_wave(&self) -> bool {
        *self == MODER::SQUARE_WAVE
    }
    #[doc = "Checks if the value of the field is `_2_BIT`"]
    #[inline]
    pub fn is_2_bit(&self) -> bool {
        *self == MODER::_2_BIT
    }
    #[doc = "Checks if the value of the field is `_3_BIT`"]
    #[inline]
    pub fn is_3_bit(&self) -> bool {
        *self == MODER::_3_BIT
    }
    #[doc = "Checks if the value of the field is `_4_BIT`"]
    #[inline]
    pub fn is_4_bit(&self) -> bool {
        *self == MODER::_4_BIT
    }
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline]
    pub fn is_5_bit(&self) -> bool {
        *self == MODER::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline]
    pub fn is_6_bit(&self) -> bool {
        *self == MODER::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline]
    pub fn is_7_bit(&self) -> bool {
        *self == MODER::_7_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == MODER::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_9_BIT`"]
    #[inline]
    pub fn is_9_bit(&self) -> bool {
        *self == MODER::_9_BIT
    }
    #[doc = "Checks if the value of the field is `_10_BIT`"]
    #[inline]
    pub fn is_10_bit(&self) -> bool {
        *self == MODER::_10_BIT
    }
    #[doc = "Checks if the value of the field is `_11_BIT`"]
    #[inline]
    pub fn is_11_bit(&self) -> bool {
        *self == MODER::_11_BIT
    }
    #[doc = "Checks if the value of the field is `_12_BIT`"]
    #[inline]
    pub fn is_12_bit(&self) -> bool {
        *self == MODER::_12_BIT
    }
    #[doc = "Checks if the value of the field is `_13_BIT`"]
    #[inline]
    pub fn is_13_bit(&self) -> bool {
        *self == MODER::_13_BIT
    }
    #[doc = "Checks if the value of the field is `_14_BIT`"]
    #[inline]
    pub fn is_14_bit(&self) -> bool {
        *self == MODER::_14_BIT
    }
    #[doc = "Checks if the value of the field is `_15_BIT`"]
    #[inline]
    pub fn is_15_bit(&self) -> bool {
        *self == MODER::_15_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline]
    pub fn is_16_bit(&self) -> bool {
        *self == MODER::_16_BIT
    }
    #[doc = "Checks if the value of the field is `_17_BIT`"]
    #[inline]
    pub fn is_17_bit(&self) -> bool {
        *self == MODER::_17_BIT
    }
    #[doc = "Checks if the value of the field is `_18_BIT`"]
    #[inline]
    pub fn is_18_bit(&self) -> bool {
        *self == MODER::_18_BIT
    }
    #[doc = "Checks if the value of the field is `_19_BIT`"]
    #[inline]
    pub fn is_19_bit(&self) -> bool {
        *self == MODER::_19_BIT
    }
    #[doc = "Checks if the value of the field is `_20_BIT`"]
    #[inline]
    pub fn is_20_bit(&self) -> bool {
        *self == MODER::_20_BIT
    }
    #[doc = "Checks if the value of the field is `_21_BIT`"]
    #[inline]
    pub fn is_21_bit(&self) -> bool {
        *self == MODER::_21_BIT
    }
    #[doc = "Checks if the value of the field is `_22_BIT`"]
    #[inline]
    pub fn is_22_bit(&self) -> bool {
        *self == MODER::_22_BIT
    }
    #[doc = "Checks if the value of the field is `_23_BIT`"]
    #[inline]
    pub fn is_23_bit(&self) -> bool {
        *self == MODER::_23_BIT
    }
    #[doc = "Checks if the value of the field is `_24_BIT`"]
    #[inline]
    pub fn is_24_bit(&self) -> bool {
        *self == MODER::_24_BIT
    }
    #[doc = "Checks if the value of the field is `_25_BIT`"]
    #[inline]
    pub fn is_25_bit(&self) -> bool {
        *self == MODER::_25_BIT
    }
    #[doc = "Checks if the value of the field is `_26_BIT`"]
    #[inline]
    pub fn is_26_bit(&self) -> bool {
        *self == MODER::_26_BIT
    }
    #[doc = "Checks if the value of the field is `_27_BIT`"]
    #[inline]
    pub fn is_27_bit(&self) -> bool {
        *self == MODER::_27_BIT
    }
    #[doc = "Checks if the value of the field is `_28_BIT`"]
    #[inline]
    pub fn is_28_bit(&self) -> bool {
        *self == MODER::_28_BIT
    }
    #[doc = "Checks if the value of the field is `_29_BIT`"]
    #[inline]
    pub fn is_29_bit(&self) -> bool {
        *self == MODER::_29_BIT
    }
    #[doc = "Checks if the value of the field is `_30_BIT`"]
    #[inline]
    pub fn is_30_bit(&self) -> bool {
        *self == MODER::_30_BIT
    }
    #[doc = "Checks if the value of the field is `_31_BIT`"]
    #[inline]
    pub fn is_31_bit(&self) -> bool {
        *self == MODER::_31_BIT
    }
}
#[doc = "Values that can be written to the field `rate_control`"]
pub enum RATE_CONTROLW {}
impl RATE_CONTROLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _RATE_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _RATE_CONTROLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RATE_CONTROLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 134217727;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `mode`"]
pub enum MODEW {
    #[doc = "Pulse train, 32 bit pattern."]
    _32_BIT,
    #[doc = "Square wave mode."]
    SQUARE_WAVE,
    #[doc = "Pulse train, 2 bit pattern."]
    _2_BIT,
    #[doc = "Pulse train, 3 bit pattern."]
    _3_BIT,
    #[doc = "Pulse train, 4 bit pattern."]
    _4_BIT,
    #[doc = "Pulse train, 5 bit pattern."]
    _5_BIT,
    #[doc = "Pulse train, 6 bit pattern."]
    _6_BIT,
    #[doc = "Pulse train, 7 bit pattern."]
    _7_BIT,
    #[doc = "Pulse train, 8 bit pattern."]
    _8_BIT,
    #[doc = "Pulse train, 9 bit pattern."]
    _9_BIT,
    #[doc = "Pulse train, 10 bit pattern."]
    _10_BIT,
    #[doc = "Pulse train, 11 bit pattern."]
    _11_BIT,
    #[doc = "Pulse train, 12 bit pattern."]
    _12_BIT,
    #[doc = "Pulse train, 13 bit pattern."]
    _13_BIT,
    #[doc = "Pulse train, 14 bit pattern."]
    _14_BIT,
    #[doc = "Pulse train, 15 bit pattern."]
    _15_BIT,
    #[doc = "Pulse train, 16 bit pattern."]
    _16_BIT,
    #[doc = "Pulse train, 17 bit pattern."]
    _17_BIT,
    #[doc = "Pulse train, 18 bit pattern."]
    _18_BIT,
    #[doc = "Pulse train, 19 bit pattern."]
    _19_BIT,
    #[doc = "Pulse train, 20 bit pattern."]
    _20_BIT,
    #[doc = "Pulse train, 21 bit pattern."]
    _21_BIT,
    #[doc = "Pulse train, 22 bit pattern."]
    _22_BIT,
    #[doc = "Pulse train, 23 bit pattern."]
    _23_BIT,
    #[doc = "Pulse train, 24 bit pattern."]
    _24_BIT,
    #[doc = "Pulse train, 25 bit pattern."]
    _25_BIT,
    #[doc = "Pulse train, 26 bit pattern."]
    _26_BIT,
    #[doc = "Pulse train, 27 bit pattern."]
    _27_BIT,
    #[doc = "Pulse train, 28 bit pattern."]
    _28_BIT,
    #[doc = "Pulse train, 29 bit pattern."]
    _29_BIT,
    #[doc = "Pulse train, 30 bit pattern."]
    _30_BIT,
    #[doc = "Pulse train, 31 bit pattern."]
    _31_BIT,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::_32_BIT => 0,
            MODEW::SQUARE_WAVE => 1,
            MODEW::_2_BIT => 2,
            MODEW::_3_BIT => 3,
            MODEW::_4_BIT => 4,
            MODEW::_5_BIT => 5,
            MODEW::_6_BIT => 6,
            MODEW::_7_BIT => 7,
            MODEW::_8_BIT => 8,
            MODEW::_9_BIT => 9,
            MODEW::_10_BIT => 10,
            MODEW::_11_BIT => 11,
            MODEW::_12_BIT => 12,
            MODEW::_13_BIT => 13,
            MODEW::_14_BIT => 14,
            MODEW::_15_BIT => 15,
            MODEW::_16_BIT => 16,
            MODEW::_17_BIT => 17,
            MODEW::_18_BIT => 18,
            MODEW::_19_BIT => 19,
            MODEW::_20_BIT => 20,
            MODEW::_21_BIT => 21,
            MODEW::_22_BIT => 22,
            MODEW::_23_BIT => 23,
            MODEW::_24_BIT => 24,
            MODEW::_25_BIT => 25,
            MODEW::_26_BIT => 26,
            MODEW::_27_BIT => 27,
            MODEW::_28_BIT => 28,
            MODEW::_29_BIT => 29,
            MODEW::_30_BIT => 30,
            MODEW::_31_BIT => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pulse train, 32 bit pattern."]
    #[inline]
    pub fn _32_bit(self) -> &'a mut W {
        self.variant(MODEW::_32_BIT)
    }
    #[doc = "Square wave mode."]
    #[inline]
    pub fn square_wave(self) -> &'a mut W {
        self.variant(MODEW::SQUARE_WAVE)
    }
    #[doc = "Pulse train, 2 bit pattern."]
    #[inline]
    pub fn _2_bit(self) -> &'a mut W {
        self.variant(MODEW::_2_BIT)
    }
    #[doc = "Pulse train, 3 bit pattern."]
    #[inline]
    pub fn _3_bit(self) -> &'a mut W {
        self.variant(MODEW::_3_BIT)
    }
    #[doc = "Pulse train, 4 bit pattern."]
    #[inline]
    pub fn _4_bit(self) -> &'a mut W {
        self.variant(MODEW::_4_BIT)
    }
    #[doc = "Pulse train, 5 bit pattern."]
    #[inline]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(MODEW::_5_BIT)
    }
    #[doc = "Pulse train, 6 bit pattern."]
    #[inline]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(MODEW::_6_BIT)
    }
    #[doc = "Pulse train, 7 bit pattern."]
    #[inline]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(MODEW::_7_BIT)
    }
    #[doc = "Pulse train, 8 bit pattern."]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(MODEW::_8_BIT)
    }
    #[doc = "Pulse train, 9 bit pattern."]
    #[inline]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(MODEW::_9_BIT)
    }
    #[doc = "Pulse train, 10 bit pattern."]
    #[inline]
    pub fn _10_bit(self) -> &'a mut W {
        self.variant(MODEW::_10_BIT)
    }
    #[doc = "Pulse train, 11 bit pattern."]
    #[inline]
    pub fn _11_bit(self) -> &'a mut W {
        self.variant(MODEW::_11_BIT)
    }
    #[doc = "Pulse train, 12 bit pattern."]
    #[inline]
    pub fn _12_bit(self) -> &'a mut W {
        self.variant(MODEW::_12_BIT)
    }
    #[doc = "Pulse train, 13 bit pattern."]
    #[inline]
    pub fn _13_bit(self) -> &'a mut W {
        self.variant(MODEW::_13_BIT)
    }
    #[doc = "Pulse train, 14 bit pattern."]
    #[inline]
    pub fn _14_bit(self) -> &'a mut W {
        self.variant(MODEW::_14_BIT)
    }
    #[doc = "Pulse train, 15 bit pattern."]
    #[inline]
    pub fn _15_bit(self) -> &'a mut W {
        self.variant(MODEW::_15_BIT)
    }
    #[doc = "Pulse train, 16 bit pattern."]
    #[inline]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(MODEW::_16_BIT)
    }
    #[doc = "Pulse train, 17 bit pattern."]
    #[inline]
    pub fn _17_bit(self) -> &'a mut W {
        self.variant(MODEW::_17_BIT)
    }
    #[doc = "Pulse train, 18 bit pattern."]
    #[inline]
    pub fn _18_bit(self) -> &'a mut W {
        self.variant(MODEW::_18_BIT)
    }
    #[doc = "Pulse train, 19 bit pattern."]
    #[inline]
    pub fn _19_bit(self) -> &'a mut W {
        self.variant(MODEW::_19_BIT)
    }
    #[doc = "Pulse train, 20 bit pattern."]
    #[inline]
    pub fn _20_bit(self) -> &'a mut W {
        self.variant(MODEW::_20_BIT)
    }
    #[doc = "Pulse train, 21 bit pattern."]
    #[inline]
    pub fn _21_bit(self) -> &'a mut W {
        self.variant(MODEW::_21_BIT)
    }
    #[doc = "Pulse train, 22 bit pattern."]
    #[inline]
    pub fn _22_bit(self) -> &'a mut W {
        self.variant(MODEW::_22_BIT)
    }
    #[doc = "Pulse train, 23 bit pattern."]
    #[inline]
    pub fn _23_bit(self) -> &'a mut W {
        self.variant(MODEW::_23_BIT)
    }
    #[doc = "Pulse train, 24 bit pattern."]
    #[inline]
    pub fn _24_bit(self) -> &'a mut W {
        self.variant(MODEW::_24_BIT)
    }
    #[doc = "Pulse train, 25 bit pattern."]
    #[inline]
    pub fn _25_bit(self) -> &'a mut W {
        self.variant(MODEW::_25_BIT)
    }
    #[doc = "Pulse train, 26 bit pattern."]
    #[inline]
    pub fn _26_bit(self) -> &'a mut W {
        self.variant(MODEW::_26_BIT)
    }
    #[doc = "Pulse train, 27 bit pattern."]
    #[inline]
    pub fn _27_bit(self) -> &'a mut W {
        self.variant(MODEW::_27_BIT)
    }
    #[doc = "Pulse train, 28 bit pattern."]
    #[inline]
    pub fn _28_bit(self) -> &'a mut W {
        self.variant(MODEW::_28_BIT)
    }
    #[doc = "Pulse train, 29 bit pattern."]
    #[inline]
    pub fn _29_bit(self) -> &'a mut W {
        self.variant(MODEW::_29_BIT)
    }
    #[doc = "Pulse train, 30 bit pattern."]
    #[inline]
    pub fn _30_bit(self) -> &'a mut W {
        self.variant(MODEW::_30_BIT)
    }
    #[doc = "Pulse train, 31 bit pattern."]
    #[inline]
    pub fn _31_bit(self) -> &'a mut W {
        self.variant(MODEW::_31_BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:26 - Pulse Train Enable/Rate Control"]
    #[inline]
    pub fn rate_control(&self) -> RATE_CONTROLR {
        RATE_CONTROLR::_from({
            const MASK: u32 = 134217727;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
    #[doc = "Bits 27:31 - Pulse Train Output Mode/Train Length"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:26 - Pulse Train Enable/Rate Control"]
    #[inline]
    pub fn rate_control(&mut self) -> _RATE_CONTROLW {
        _RATE_CONTROLW { w: self }
    }
    #[doc = "Bits 27:31 - Pulse Train Output Mode/Train Length"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
