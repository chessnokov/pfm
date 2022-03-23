#[doc = "Register `DKCFGR2` reader"]
pub struct R(crate::R<DKCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DKCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DKCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DKCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DKCFGR2` writer"]
pub struct W(crate::W<DKCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DKCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DKCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DKCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART1SEL` reader - USART 1 clock source selection"]
pub struct USART1SEL_R(crate::FieldReader<u8, u8>);
impl USART1SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USART1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1SEL` writer - USART 1 clock source selection"]
pub struct USART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `USART2SEL` reader - USART 2 clock source selection"]
pub struct USART2SEL_R(crate::FieldReader<u8, u8>);
impl USART2SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USART2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2SEL` writer - USART 2 clock source selection"]
pub struct USART2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `USART3SEL` reader - USART 3 clock source selection"]
pub struct USART3SEL_R(crate::FieldReader<u8, u8>);
impl USART3SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USART3SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART3SEL` writer - USART 3 clock source selection"]
pub struct USART3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `UART4SEL` reader - UART 4 clock source selection"]
pub struct UART4SEL_R(crate::FieldReader<u8, u8>);
impl UART4SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART4SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART4SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4SEL` writer - UART 4 clock source selection"]
pub struct UART4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `UART5SEL` reader - UART 5 clock source selection"]
pub struct UART5SEL_R(crate::FieldReader<u8, u8>);
impl UART5SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART5SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART5SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART5SEL` writer - UART 5 clock source selection"]
pub struct UART5SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `USART6SEL` reader - USART 6 clock source selection"]
pub struct USART6SEL_R(crate::FieldReader<u8, u8>);
impl USART6SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USART6SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART6SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART6SEL` writer - USART 6 clock source selection"]
pub struct USART6SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART6SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `UART7SEL` reader - UART 7 clock source selection"]
pub struct UART7SEL_R(crate::FieldReader<u8, u8>);
impl UART7SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART7SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART7SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART7SEL` writer - UART 7 clock source selection"]
pub struct UART7SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART7SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `UART8SEL` reader - UART 8 clock source selection"]
pub struct UART8SEL_R(crate::FieldReader<u8, u8>);
impl UART8SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART8SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART8SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART8SEL` writer - UART 8 clock source selection"]
pub struct UART8SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART8SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub struct I2C1SEL_R(crate::FieldReader<u8, u8>);
impl I2C1SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2C1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub struct I2C1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `I2C2SEL` reader - I2C2 clock source selection"]
pub struct I2C2SEL_R(crate::FieldReader<u8, u8>);
impl I2C2SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2C2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2SEL` writer - I2C2 clock source selection"]
pub struct I2C2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection"]
pub struct I2C3SEL_R(crate::FieldReader<u8, u8>);
impl I2C3SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2C3SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C3SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection"]
pub struct I2C3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `I2C4SEL` reader - I2C4 clock source selection"]
pub struct I2C4SEL_R(crate::FieldReader<u8, u8>);
impl I2C4SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2C4SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C4SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4SEL` writer - I2C4 clock source selection"]
pub struct I2C4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `LPTIM1SEL` reader - Low power timer 1 clock source selection"]
pub struct LPTIM1SEL_R(crate::FieldReader<u8, u8>);
impl LPTIM1SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1SEL` writer - Low power timer 1 clock source selection"]
pub struct LPTIM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `CECSEL` reader - HDMI-CEC clock source selection"]
pub struct CECSEL_R(crate::FieldReader<bool, bool>);
impl CECSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CECSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CECSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CECSEL` writer - HDMI-CEC clock source selection"]
pub struct CECSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CK48MSEL` reader - 48MHz clock source selection"]
pub struct CK48MSEL_R(crate::FieldReader<bool, bool>);
impl CK48MSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CK48MSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CK48MSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CK48MSEL` writer - 48MHz clock source selection"]
pub struct CK48MSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CK48MSEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `SDMMCSEL` reader - SDMMC clock source selection"]
pub struct SDMMCSEL_R(crate::FieldReader<bool, bool>);
impl SDMMCSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMMCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMCSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMCSEL` writer - SDMMC clock source selection"]
pub struct SDMMCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMCSEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `SDMMC2SEL` reader - SDMMC2 clock source selection"]
pub struct SDMMC2SEL_R(crate::FieldReader<bool, bool>);
impl SDMMC2SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDMMC2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMMC2SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMC2SEL` writer - SDMMC2 clock source selection"]
pub struct SDMMC2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMC2SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `DSISEL` reader - DSI clock source selection"]
pub struct DSISEL_R(crate::FieldReader<bool, bool>);
impl DSISEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSISEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSISEL` writer - DSI clock source selection"]
pub struct DSISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSISEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USART 1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - USART 2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - USART 3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - UART 4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - UART 5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - USART 6 clock source selection"]
    #[inline(always)]
    pub fn usart6sel(&self) -> USART6SEL_R {
        USART6SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - UART 7 clock source selection"]
    #[inline(always)]
    pub fn uart7sel(&self) -> UART7SEL_R {
        UART7SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - UART 8 clock source selection"]
    #[inline(always)]
    pub fn uart8sel(&self) -> UART8SEL_R {
        UART8SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - HDMI-CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 48MHz clock source selection"]
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - SDMMC clock source selection"]
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - SDMMC2 clock source selection"]
    #[inline(always)]
    pub fn sdmmc2sel(&self) -> SDMMC2SEL_R {
        SDMMC2SEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DSI clock source selection"]
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART 1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W {
        USART1SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - USART 2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W {
        USART2SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - USART 3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&mut self) -> USART3SEL_W {
        USART3SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - UART 4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&mut self) -> UART4SEL_W {
        UART4SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - UART 5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&mut self) -> UART5SEL_W {
        UART5SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - USART 6 clock source selection"]
    #[inline(always)]
    pub fn usart6sel(&mut self) -> USART6SEL_W {
        USART6SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - UART 7 clock source selection"]
    #[inline(always)]
    pub fn uart7sel(&mut self) -> UART7SEL_W {
        UART7SEL_W { w: self }
    }
    #[doc = "Bits 14:15 - UART 8 clock source selection"]
    #[inline(always)]
    pub fn uart8sel(&mut self) -> UART8SEL_W {
        UART8SEL_W { w: self }
    }
    #[doc = "Bits 16:17 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W {
        I2C1SEL_W { w: self }
    }
    #[doc = "Bits 18:19 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W {
        I2C2SEL_W { w: self }
    }
    #[doc = "Bits 20:21 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W {
        I2C3SEL_W { w: self }
    }
    #[doc = "Bits 22:23 - I2C4 clock source selection"]
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W {
        I2C4SEL_W { w: self }
    }
    #[doc = "Bits 24:25 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W {
        LPTIM1SEL_W { w: self }
    }
    #[doc = "Bit 26 - HDMI-CEC clock source selection"]
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W {
        CECSEL_W { w: self }
    }
    #[doc = "Bit 27 - 48MHz clock source selection"]
    #[inline(always)]
    pub fn ck48msel(&mut self) -> CK48MSEL_W {
        CK48MSEL_W { w: self }
    }
    #[doc = "Bit 28 - SDMMC clock source selection"]
    #[inline(always)]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W {
        SDMMCSEL_W { w: self }
    }
    #[doc = "Bit 29 - SDMMC2 clock source selection"]
    #[inline(always)]
    pub fn sdmmc2sel(&mut self) -> SDMMC2SEL_W {
        SDMMC2SEL_W { w: self }
    }
    #[doc = "Bit 30 - DSI clock source selection"]
    #[inline(always)]
    pub fn dsisel(&mut self) -> DSISEL_W {
        DSISEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dedicated clocks configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dkcfgr2](index.html) module"]
pub struct DKCFGR2_SPEC;
impl crate::RegisterSpec for DKCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dkcfgr2::R](R) reader structure"]
impl crate::Readable for DKCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dkcfgr2::W](W) writer structure"]
impl crate::Writable for DKCFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DKCFGR2 to value 0x2000_3000"]
impl crate::Resettable for DKCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_3000
    }
}
