#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYS_CLK_CTRL_2_SPIX {
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
#[doc = "Possible values of the field `spix_clk_scale`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIX_CLK_SCALER {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPIX_CLK_SCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPIX_CLK_SCALER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPIX_CLK_SCALER {
        match value {
            i => SPIX_CLK_SCALER::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `spix_clk_scale`"]
pub enum SPIX_CLK_SCALEW {}
impl SPIX_CLK_SCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SPIX_CLK_SCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIX_CLK_SCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIX_CLK_SCALEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - Control Settings for CLK2 - SPI XIP Clock"]
    #[inline]
    pub fn spix_clk_scale(&self) -> SPIX_CLK_SCALER {
        SPIX_CLK_SCALER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:3 - Control Settings for CLK2 - SPI XIP Clock"]
    #[inline]
    pub fn spix_clk_scale(&mut self) -> _SPIX_CLK_SCALEW {
        _SPIX_CLK_SCALEW { w: self }
    }
}
