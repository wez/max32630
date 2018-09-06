#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ID {
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
#[doc = "Possible values of the field `rtl_version`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTL_VERSIONR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RTL_VERSIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTL_VERSIONR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTL_VERSIONR {
        match value {
            i => RTL_VERSIONR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `part_num`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PART_NUMR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PART_NUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PART_NUMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PART_NUMR {
        match value {
            i => PART_NUMR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `cache_id`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHE_IDR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CACHE_IDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CACHE_IDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CACHE_IDR {
        match value {
            i => CACHE_IDR::_Reserved(i),
        }
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - RTL Release Version"]
    #[inline]
    pub fn rtl_version(&self) -> RTL_VERSIONR {
        RTL_VERSIONR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:9 - Part Number ID"]
    #[inline]
    pub fn part_num(&self) -> PART_NUMR {
        PART_NUMR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:15 - Cache ID"]
    #[inline]
    pub fn cache_id(&self) -> CACHE_IDR {
        CACHE_IDR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 10;
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
}
