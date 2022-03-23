#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCO2` reader - Microcontroller clock output 2"]
pub struct MCO2_R(crate::FieldReader<u8, u8>);
impl MCO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCO2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCO2` writer - Microcontroller clock output 2"]
pub struct MCO2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `MCO2PRE` reader - MCO2 prescaler"]
pub struct MCO2PRE_R(crate::FieldReader<u8, u8>);
impl MCO2PRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCO2PRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCO2PRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCO2PRE` writer - MCO2 prescaler"]
pub struct MCO2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Field `MCO1PRE` reader - MCO1 prescaler"]
pub struct MCO1PRE_R(crate::FieldReader<u8, u8>);
impl MCO1PRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCO1PRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCO1PRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCO1PRE` writer - MCO1 prescaler"]
pub struct MCO1PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `I2SSRC` reader - I2S clock selection"]
pub struct I2SSRC_R(crate::FieldReader<bool, bool>);
impl I2SSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2SSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2SSRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2SSRC` writer - I2S clock selection"]
pub struct I2SSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SSRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `MCO1` reader - Microcontroller clock output 1"]
pub struct MCO1_R(crate::FieldReader<u8, u8>);
impl MCO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MCO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCO1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCO1` writer - Microcontroller clock output 1"]
pub struct MCO1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `RTCPRE` reader - HSE division factor for RTC clock"]
pub struct RTCPRE_R(crate::FieldReader<u8, u8>);
impl RTCPRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTCPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCPRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCPRE` writer - HSE division factor for RTC clock"]
pub struct RTCPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `PPRE2` reader - APB high-speed prescaler (APB2)"]
pub struct PPRE2_R(crate::FieldReader<u8, u8>);
impl PPRE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PPRE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPRE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPRE2` writer - APB high-speed prescaler (APB2)"]
pub struct PPRE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `PPRE1` reader - APB Low speed prescaler (APB1)"]
pub struct PPRE1_R(crate::FieldReader<u8, u8>);
impl PPRE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PPRE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPRE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPRE1` writer - APB Low speed prescaler (APB1)"]
pub struct PPRE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | ((value as u32 & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub struct HPRE_R(crate::FieldReader<u8, u8>);
impl HPRE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        HPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub struct HPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `SWS1` reader - System clock switch status"]
pub struct SWS1_R(crate::FieldReader<bool, bool>);
impl SWS1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWS0` reader - System clock switch status"]
pub struct SWS0_R(crate::FieldReader<bool, bool>);
impl SWS0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWS0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW1` reader - System clock switch"]
pub struct SW1_R(crate::FieldReader<bool, bool>);
impl SW1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW1` writer - System clock switch"]
pub struct SW1_W<'a> {
    w: &'a mut W,
}
impl<'a> SW1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SW0` reader - System clock switch"]
pub struct SW0_R(crate::FieldReader<bool, bool>);
impl SW0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW0` writer - System clock switch"]
pub struct SW0_W<'a> {
    w: &'a mut W,
}
impl<'a> SW0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline(always)]
    pub fn mco2(&self) -> MCO2_R {
        MCO2_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline(always)]
    pub fn mco1(&self) -> MCO1_R {
        MCO1_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - System clock switch status"]
    #[inline(always)]
    pub fn sws1(&self) -> SWS1_R {
        SWS1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - System clock switch status"]
    #[inline(always)]
    pub fn sws0(&self) -> SWS0_R {
        SWS0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - System clock switch"]
    #[inline(always)]
    pub fn sw1(&self) -> SW1_R {
        SW1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - System clock switch"]
    #[inline(always)]
    pub fn sw0(&self) -> SW0_R {
        SW0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline(always)]
    pub fn mco2(&mut self) -> MCO2_W {
        MCO2_W { w: self }
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W {
        MCO2PRE_W { w: self }
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&mut self) -> MCO1PRE_W {
        MCO1PRE_W { w: self }
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline(always)]
    pub fn i2ssrc(&mut self) -> I2SSRC_W {
        I2SSRC_W { w: self }
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline(always)]
    pub fn mco1(&mut self) -> MCO1_W {
        MCO1_W { w: self }
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W {
        RTCPRE_W { w: self }
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W {
        PPRE2_W { w: self }
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W {
        PPRE1_W { w: self }
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W {
        HPRE_W { w: self }
    }
    #[doc = "Bit 1 - System clock switch"]
    #[inline(always)]
    pub fn sw1(&mut self) -> SW1_W {
        SW1_W { w: self }
    }
    #[doc = "Bit 0 - System clock switch"]
    #[inline(always)]
    pub fn sw0(&mut self) -> SW0_W {
        SW0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
