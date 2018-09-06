#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FETCH_CTRL {
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
#[doc = "Possible values of the field `cmd_value`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_VALUER {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMD_VALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMD_VALUER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMD_VALUER {
        match value {
            i => CMD_VALUER::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `cmd_width`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_WIDTHR {
    #[doc = "Single I/O used for Tx/Rx."]
    SINGLE,
    #[doc = "Dual I/O lines used for Tx/Rx."]
    DUAL_IO,
    #[doc = "Quad I/O lines used for Tx/Rx."]
    QUAD_IO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CMD_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMD_WIDTHR::SINGLE => 0,
            CMD_WIDTHR::DUAL_IO => 1,
            CMD_WIDTHR::QUAD_IO => 2,
            CMD_WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMD_WIDTHR {
        match value {
            0 => CMD_WIDTHR::SINGLE,
            1 => CMD_WIDTHR::DUAL_IO,
            2 => CMD_WIDTHR::QUAD_IO,
            i => CMD_WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == CMD_WIDTHR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline]
    pub fn is_dual_io(&self) -> bool {
        *self == CMD_WIDTHR::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline]
    pub fn is_quad_io(&self) -> bool {
        *self == CMD_WIDTHR::QUAD_IO
    }
}
#[doc = "Possible values of the field `addr_width`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR_WIDTHR {
    #[doc = "Single I/O used for Tx/Rx."]
    SINGLE,
    #[doc = "Dual I/O lines used for Tx/Rx."]
    DUAL_IO,
    #[doc = "Quad I/O lines used for Tx/Rx."]
    QUAD_IO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADDR_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADDR_WIDTHR::SINGLE => 0,
            ADDR_WIDTHR::DUAL_IO => 1,
            ADDR_WIDTHR::QUAD_IO => 2,
            ADDR_WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADDR_WIDTHR {
        match value {
            0 => ADDR_WIDTHR::SINGLE,
            1 => ADDR_WIDTHR::DUAL_IO,
            2 => ADDR_WIDTHR::QUAD_IO,
            i => ADDR_WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == ADDR_WIDTHR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline]
    pub fn is_dual_io(&self) -> bool {
        *self == ADDR_WIDTHR::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline]
    pub fn is_quad_io(&self) -> bool {
        *self == ADDR_WIDTHR::QUAD_IO
    }
}
#[doc = "Possible values of the field `data_width`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA_WIDTHR {
    #[doc = "Single I/O used for Tx/Rx."]
    SINGLE,
    #[doc = "Dual I/O lines used for Tx/Rx."]
    DUAL_IO,
    #[doc = "Quad I/O lines used for Tx/Rx."]
    QUAD_IO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DATA_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATA_WIDTHR::SINGLE => 0,
            DATA_WIDTHR::DUAL_IO => 1,
            DATA_WIDTHR::QUAD_IO => 2,
            DATA_WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATA_WIDTHR {
        match value {
            0 => DATA_WIDTHR::SINGLE,
            1 => DATA_WIDTHR::DUAL_IO,
            2 => DATA_WIDTHR::QUAD_IO,
            i => DATA_WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == DATA_WIDTHR::SINGLE
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline]
    pub fn is_dual_io(&self) -> bool {
        *self == DATA_WIDTHR::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline]
    pub fn is_quad_io(&self) -> bool {
        *self == DATA_WIDTHR::QUAD_IO
    }
}
#[doc = "Values that can be written to the field `cmd_value`"]
pub enum CMD_VALUEW {}
impl CMD_VALUEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _CMD_VALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_VALUEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_VALUEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `cmd_width`"]
pub enum CMD_WIDTHW {
    #[doc = "Single I/O used for Tx/Rx."]
    SINGLE,
    #[doc = "Dual I/O lines used for Tx/Rx."]
    DUAL_IO,
    #[doc = "Quad I/O lines used for Tx/Rx."]
    QUAD_IO,
}
impl CMD_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMD_WIDTHW::SINGLE => 0,
            CMD_WIDTHW::DUAL_IO => 1,
            CMD_WIDTHW::QUAD_IO => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single I/O used for Tx/Rx."]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(CMD_WIDTHW::SINGLE)
    }
    #[doc = "Dual I/O lines used for Tx/Rx."]
    #[inline]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(CMD_WIDTHW::DUAL_IO)
    }
    #[doc = "Quad I/O lines used for Tx/Rx."]
    #[inline]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(CMD_WIDTHW::QUAD_IO)
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
#[doc = "Values that can be written to the field `addr_width`"]
pub enum ADDR_WIDTHW {
    #[doc = "Single I/O used for Tx/Rx."]
    SINGLE,
    #[doc = "Dual I/O lines used for Tx/Rx."]
    DUAL_IO,
    #[doc = "Quad I/O lines used for Tx/Rx."]
    QUAD_IO,
}
impl ADDR_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADDR_WIDTHW::SINGLE => 0,
            ADDR_WIDTHW::DUAL_IO => 1,
            ADDR_WIDTHW::QUAD_IO => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDR_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDR_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDR_WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single I/O used for Tx/Rx."]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(ADDR_WIDTHW::SINGLE)
    }
    #[doc = "Dual I/O lines used for Tx/Rx."]
    #[inline]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(ADDR_WIDTHW::DUAL_IO)
    }
    #[doc = "Quad I/O lines used for Tx/Rx."]
    #[inline]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(ADDR_WIDTHW::QUAD_IO)
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
#[doc = "Values that can be written to the field `data_width`"]
pub enum DATA_WIDTHW {
    #[doc = "Single I/O used for Tx/Rx."]
    SINGLE,
    #[doc = "Dual I/O lines used for Tx/Rx."]
    DUAL_IO,
    #[doc = "Quad I/O lines used for Tx/Rx."]
    QUAD_IO,
}
impl DATA_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATA_WIDTHW::SINGLE => 0,
            DATA_WIDTHW::DUAL_IO => 1,
            DATA_WIDTHW::QUAD_IO => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATA_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATA_WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single I/O used for Tx/Rx."]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(DATA_WIDTHW::SINGLE)
    }
    #[doc = "Dual I/O lines used for Tx/Rx."]
    #[inline]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(DATA_WIDTHW::DUAL_IO)
    }
    #[doc = "Quad I/O lines used for Tx/Rx."]
    #[inline]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(DATA_WIDTHW::QUAD_IO)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Command Value"]
    #[inline]
    pub fn cmd_value(&self) -> CMD_VALUER {
        CMD_VALUER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Command Width"]
    #[inline]
    pub fn cmd_width(&self) -> CMD_WIDTHR {
        CMD_WIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Address Width"]
    #[inline]
    pub fn addr_width(&self) -> ADDR_WIDTHR {
        ADDR_WIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Data Width"]
    #[inline]
    pub fn data_width(&self) -> DATA_WIDTHR {
        DATA_WIDTHR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 0:7 - Command Value"]
    #[inline]
    pub fn cmd_value(&mut self) -> _CMD_VALUEW {
        _CMD_VALUEW { w: self }
    }
    #[doc = "Bits 8:9 - Command Width"]
    #[inline]
    pub fn cmd_width(&mut self) -> _CMD_WIDTHW {
        _CMD_WIDTHW { w: self }
    }
    #[doc = "Bits 10:11 - Address Width"]
    #[inline]
    pub fn addr_width(&mut self) -> _ADDR_WIDTHW {
        _ADDR_WIDTHW { w: self }
    }
    #[doc = "Bits 12:13 - Data Width"]
    #[inline]
    pub fn data_width(&mut self) -> _DATA_WIDTHW {
        _DATA_WIDTHW { w: self }
    }
}
