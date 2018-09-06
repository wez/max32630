#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPCL_CTRL {
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
#[doc = "Possible values of the field `ss_sample_mode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_SAMPLE_MODER {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl SS_SAMPLE_MODER {
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
            SS_SAMPLE_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SS_SAMPLE_MODER {
        match value {
            i => SS_SAMPLE_MODER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `miso_fc_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISO_FC_ENR {
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl MISO_FC_ENR {
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
            MISO_FC_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MISO_FC_ENR {
        match value {
            i => MISO_FC_ENR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ss_sa_sdio_out`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_SA_SDIO_OUTR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SS_SA_SDIO_OUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SS_SA_SDIO_OUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SS_SA_SDIO_OUTR {
        match value {
            i => SS_SA_SDIO_OUTR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `ss_sa_sdio_dr_en`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_SA_SDIO_DR_ENR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SS_SA_SDIO_DR_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SS_SA_SDIO_DR_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SS_SA_SDIO_DR_ENR {
        match value {
            i => SS_SA_SDIO_DR_ENR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `ss_sample_mode`"]
pub enum SS_SAMPLE_MODEW {}
impl SS_SAMPLE_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SS_SAMPLE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SS_SAMPLE_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS_SAMPLE_MODEW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `miso_fc_en`"]
pub enum MISO_FC_ENW {}
impl MISO_FC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _MISO_FC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _MISO_FC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MISO_FC_ENW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `ss_sa_sdio_out`"]
pub enum SS_SA_SDIO_OUTW {}
impl SS_SA_SDIO_OUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SS_SA_SDIO_OUTW<'a> {
    w: &'a mut W,
}
impl<'a> _SS_SA_SDIO_OUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS_SA_SDIO_OUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ss_sa_sdio_dr_en`"]
pub enum SS_SA_SDIO_DR_ENW {}
impl SS_SA_SDIO_DR_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _SS_SA_SDIO_DR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SS_SA_SDIO_DR_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SS_SA_SDIO_DR_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - SS Sample Mode"]
    #[inline]
    pub fn ss_sample_mode(&self) -> SS_SAMPLE_MODER {
        SS_SAMPLE_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - SDIO(1) to SR(0) Mode"]
    #[inline]
    pub fn miso_fc_en(&self) -> MISO_FC_ENR {
        MISO_FC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:7 - SDIO Active Output Value"]
    #[inline]
    pub fn ss_sa_sdio_out(&self) -> SS_SA_SDIO_OUTR {
        SS_SA_SDIO_OUTR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - SDIO Active Drive Mode"]
    #[inline]
    pub fn ss_sa_sdio_dr_en(&self) -> SS_SA_SDIO_DR_ENR {
        SS_SA_SDIO_DR_ENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - SS Sample Mode"]
    #[inline]
    pub fn ss_sample_mode(&mut self) -> _SS_SAMPLE_MODEW {
        _SS_SAMPLE_MODEW { w: self }
    }
    #[doc = "Bit 1 - SDIO(1) to SR(0) Mode"]
    #[inline]
    pub fn miso_fc_en(&mut self) -> _MISO_FC_ENW {
        _MISO_FC_ENW { w: self }
    }
    #[doc = "Bits 4:7 - SDIO Active Output Value"]
    #[inline]
    pub fn ss_sa_sdio_out(&mut self) -> _SS_SA_SDIO_OUTW {
        _SS_SA_SDIO_OUTW { w: self }
    }
    #[doc = "Bits 8:11 - SDIO Active Drive Mode"]
    #[inline]
    pub fn ss_sa_sdio_dr_en(&mut self) -> _SS_SA_SDIO_DR_ENW {
        _SS_SA_SDIO_DR_ENW { w: self }
    }
}
