#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLK_GATE_CTRL1 {
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
#[doc = "Possible values of the field `watchdog1_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WATCHDOG1_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WATCHDOG1_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WATCHDOG1_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WATCHDOG1_CLK_GATERR {
        match value {
            i => WATCHDOG1_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `gpio_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GPIO_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GPIO_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GPIO_CLK_GATERR {
        match value {
            i => GPIO_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `timer0_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER0_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMER0_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER0_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMER0_CLK_GATERR {
        match value {
            i => TIMER0_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `timer1_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER1_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMER1_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER1_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMER1_CLK_GATERR {
        match value {
            i => TIMER1_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `timer2_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER2_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMER2_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER2_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMER2_CLK_GATERR {
        match value {
            i => TIMER2_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `timer3_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER3_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMER3_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER3_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMER3_CLK_GATERR {
        match value {
            i => TIMER3_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `timer4_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER4_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMER4_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER4_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMER4_CLK_GATERR {
        match value {
            i => TIMER4_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `timer5_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMER5_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMER5_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMER5_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMER5_CLK_GATERR {
        match value {
            i => TIMER5_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `pulsetrain_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULSETRAIN_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PULSETRAIN_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PULSETRAIN_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PULSETRAIN_CLK_GATERR {
        match value {
            i => PULSETRAIN_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `uart0_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UART0_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART0_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART0_CLK_GATERR {
        match value {
            i => UART0_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `uart1_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UART1_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART1_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART1_CLK_GATERR {
        match value {
            i => UART1_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `uart2_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UART2_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART2_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART2_CLK_GATERR {
        match value {
            i => UART2_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `uart3_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART3_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UART3_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UART3_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UART3_CLK_GATERR {
        match value {
            i => UART3_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `i2cm0_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CM0_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl I2CM0_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            I2CM0_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> I2CM0_CLK_GATERR {
        match value {
            i => I2CM0_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `i2cm1_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CM1_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl I2CM1_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            I2CM1_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> I2CM1_CLK_GATERR {
        match value {
            i => I2CM1_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Possible values of the field `i2cm2_clk_gater`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CM2_CLK_GATERR {
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl I2CM2_CLK_GATERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            I2CM2_CLK_GATERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> I2CM2_CLK_GATERR {
        match value {
            i => I2CM2_CLK_GATERR::_Reserved(i),
        }
    }
}
#[doc = "Values that can be written to the field `watchdog1_clk_gater`"]
pub enum WATCHDOG1_CLK_GATERW {}
impl WATCHDOG1_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _WATCHDOG1_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _WATCHDOG1_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WATCHDOG1_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `gpio_clk_gater`"]
pub enum GPIO_CLK_GATERW {}
impl GPIO_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _GPIO_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _GPIO_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GPIO_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `timer0_clk_gater`"]
pub enum TIMER0_CLK_GATERW {}
impl TIMER0_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TIMER0_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER0_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER0_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `timer1_clk_gater`"]
pub enum TIMER1_CLK_GATERW {}
impl TIMER1_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TIMER1_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER1_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER1_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `timer2_clk_gater`"]
pub enum TIMER2_CLK_GATERW {}
impl TIMER2_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TIMER2_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER2_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER2_CLK_GATERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `timer3_clk_gater`"]
pub enum TIMER3_CLK_GATERW {}
impl TIMER3_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TIMER3_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER3_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER3_CLK_GATERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `timer4_clk_gater`"]
pub enum TIMER4_CLK_GATERW {}
impl TIMER4_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TIMER4_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER4_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER4_CLK_GATERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `timer5_clk_gater`"]
pub enum TIMER5_CLK_GATERW {}
impl TIMER5_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _TIMER5_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER5_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMER5_CLK_GATERW) -> &'a mut W {
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
#[doc = "Values that can be written to the field `pulsetrain_clk_gater`"]
pub enum PULSETRAIN_CLK_GATERW {}
impl PULSETRAIN_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _PULSETRAIN_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _PULSETRAIN_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PULSETRAIN_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `uart0_clk_gater`"]
pub enum UART0_CLK_GATERW {}
impl UART0_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UART0_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _UART0_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART0_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `uart1_clk_gater`"]
pub enum UART1_CLK_GATERW {}
impl UART1_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UART1_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _UART1_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART1_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `uart2_clk_gater`"]
pub enum UART2_CLK_GATERW {}
impl UART2_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UART2_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _UART2_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART2_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `uart3_clk_gater`"]
pub enum UART3_CLK_GATERW {}
impl UART3_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _UART3_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _UART3_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UART3_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `i2cm0_clk_gater`"]
pub enum I2CM0_CLK_GATERW {}
impl I2CM0_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _I2CM0_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CM0_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2CM0_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `i2cm1_clk_gater`"]
pub enum I2CM1_CLK_GATERW {}
impl I2CM1_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _I2CM1_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CM1_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2CM1_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `i2cm2_clk_gater`"]
pub enum I2CM2_CLK_GATERW {}
impl I2CM2_CLK_GATERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {}
    }
}
#[doc = r" Proxy"]
pub struct _I2CM2_CLK_GATERW<'a> {
    w: &'a mut W,
}
impl<'a> _I2CM2_CLK_GATERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2CM2_CLK_GATERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Clock Gating Control for Watchdog Timer 1"]
    #[inline]
    pub fn watchdog1_clk_gater(&self) -> WATCHDOG1_CLK_GATERR {
        WATCHDOG1_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Clock Gating Control for GPIO Ports"]
    #[inline]
    pub fn gpio_clk_gater(&self) -> GPIO_CLK_GATERR {
        GPIO_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Clock Gating Control for Timer/Counter Module 0"]
    #[inline]
    pub fn timer0_clk_gater(&self) -> TIMER0_CLK_GATERR {
        TIMER0_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Clock Gating Control for Timer/Counter Module 1"]
    #[inline]
    pub fn timer1_clk_gater(&self) -> TIMER1_CLK_GATERR {
        TIMER1_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Clock Gating Control for Timer/Counter Module 2"]
    #[inline]
    pub fn timer2_clk_gater(&self) -> TIMER2_CLK_GATERR {
        TIMER2_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Clock Gating Control for Timer/Counter Module 3"]
    #[inline]
    pub fn timer3_clk_gater(&self) -> TIMER3_CLK_GATERR {
        TIMER3_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Clock Gating Control for Timer/Counter Module 4"]
    #[inline]
    pub fn timer4_clk_gater(&self) -> TIMER4_CLK_GATERR {
        TIMER4_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Clock Gating Control for Timer/Counter Module 5"]
    #[inline]
    pub fn timer5_clk_gater(&self) -> TIMER5_CLK_GATERR {
        TIMER5_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Clock Gating Control for Pulse Train Generators"]
    #[inline]
    pub fn pulsetrain_clk_gater(&self) -> PULSETRAIN_CLK_GATERR {
        PULSETRAIN_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Clock Gating Control for UART 0"]
    #[inline]
    pub fn uart0_clk_gater(&self) -> UART0_CLK_GATERR {
        UART0_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Clock Gating Control for UART 1"]
    #[inline]
    pub fn uart1_clk_gater(&self) -> UART1_CLK_GATERR {
        UART1_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Clock Gating Control for UART 2"]
    #[inline]
    pub fn uart2_clk_gater(&self) -> UART2_CLK_GATERR {
        UART2_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Clock Gating Control for UART 3"]
    #[inline]
    pub fn uart3_clk_gater(&self) -> UART3_CLK_GATERR {
        UART3_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Clock Gating Control for I2C Master 0"]
    #[inline]
    pub fn i2cm0_clk_gater(&self) -> I2CM0_CLK_GATERR {
        I2CM0_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Clock Gating Control for I2C Master 1"]
    #[inline]
    pub fn i2cm1_clk_gater(&self) -> I2CM1_CLK_GATERR {
        I2CM1_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Clock Gating Control for I2C Master 2"]
    #[inline]
    pub fn i2cm2_clk_gater(&self) -> I2CM2_CLK_GATERR {
        I2CM2_CLK_GATERR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Clock Gating Control for Watchdog Timer 1"]
    #[inline]
    pub fn watchdog1_clk_gater(&mut self) -> _WATCHDOG1_CLK_GATERW {
        _WATCHDOG1_CLK_GATERW { w: self }
    }
    #[doc = "Bits 2:3 - Clock Gating Control for GPIO Ports"]
    #[inline]
    pub fn gpio_clk_gater(&mut self) -> _GPIO_CLK_GATERW {
        _GPIO_CLK_GATERW { w: self }
    }
    #[doc = "Bits 4:5 - Clock Gating Control for Timer/Counter Module 0"]
    #[inline]
    pub fn timer0_clk_gater(&mut self) -> _TIMER0_CLK_GATERW {
        _TIMER0_CLK_GATERW { w: self }
    }
    #[doc = "Bits 6:7 - Clock Gating Control for Timer/Counter Module 1"]
    #[inline]
    pub fn timer1_clk_gater(&mut self) -> _TIMER1_CLK_GATERW {
        _TIMER1_CLK_GATERW { w: self }
    }
    #[doc = "Bits 8:9 - Clock Gating Control for Timer/Counter Module 2"]
    #[inline]
    pub fn timer2_clk_gater(&mut self) -> _TIMER2_CLK_GATERW {
        _TIMER2_CLK_GATERW { w: self }
    }
    #[doc = "Bits 10:11 - Clock Gating Control for Timer/Counter Module 3"]
    #[inline]
    pub fn timer3_clk_gater(&mut self) -> _TIMER3_CLK_GATERW {
        _TIMER3_CLK_GATERW { w: self }
    }
    #[doc = "Bits 12:13 - Clock Gating Control for Timer/Counter Module 4"]
    #[inline]
    pub fn timer4_clk_gater(&mut self) -> _TIMER4_CLK_GATERW {
        _TIMER4_CLK_GATERW { w: self }
    }
    #[doc = "Bits 14:15 - Clock Gating Control for Timer/Counter Module 5"]
    #[inline]
    pub fn timer5_clk_gater(&mut self) -> _TIMER5_CLK_GATERW {
        _TIMER5_CLK_GATERW { w: self }
    }
    #[doc = "Bits 16:17 - Clock Gating Control for Pulse Train Generators"]
    #[inline]
    pub fn pulsetrain_clk_gater(&mut self) -> _PULSETRAIN_CLK_GATERW {
        _PULSETRAIN_CLK_GATERW { w: self }
    }
    #[doc = "Bits 18:19 - Clock Gating Control for UART 0"]
    #[inline]
    pub fn uart0_clk_gater(&mut self) -> _UART0_CLK_GATERW {
        _UART0_CLK_GATERW { w: self }
    }
    #[doc = "Bits 20:21 - Clock Gating Control for UART 1"]
    #[inline]
    pub fn uart1_clk_gater(&mut self) -> _UART1_CLK_GATERW {
        _UART1_CLK_GATERW { w: self }
    }
    #[doc = "Bits 22:23 - Clock Gating Control for UART 2"]
    #[inline]
    pub fn uart2_clk_gater(&mut self) -> _UART2_CLK_GATERW {
        _UART2_CLK_GATERW { w: self }
    }
    #[doc = "Bits 24:25 - Clock Gating Control for UART 3"]
    #[inline]
    pub fn uart3_clk_gater(&mut self) -> _UART3_CLK_GATERW {
        _UART3_CLK_GATERW { w: self }
    }
    #[doc = "Bits 26:27 - Clock Gating Control for I2C Master 0"]
    #[inline]
    pub fn i2cm0_clk_gater(&mut self) -> _I2CM0_CLK_GATERW {
        _I2CM0_CLK_GATERW { w: self }
    }
    #[doc = "Bits 28:29 - Clock Gating Control for I2C Master 1"]
    #[inline]
    pub fn i2cm1_clk_gater(&mut self) -> _I2CM1_CLK_GATERW {
        _I2CM1_CLK_GATERW { w: self }
    }
    #[doc = "Bits 30:31 - Clock Gating Control for I2C Master 2"]
    #[inline]
    pub fn i2cm2_clk_gater(&mut self) -> _I2CM2_CLK_GATERW {
        _I2CM2_CLK_GATERW { w: self }
    }
}
