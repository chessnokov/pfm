#[doc = "Register `DKCFGR1` reader"]
pub struct R(crate::R<DKCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DKCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DKCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DKCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DKCFGR1` writer"]
pub struct W(crate::W<DKCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DKCFGR1_SPEC>;
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
impl From<crate::W<DKCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DKCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLI2SDIV` reader - PLLI2S division factor for SAI1 clock"]
pub struct PLLI2SDIV_R(crate::FieldReader<u8, u8>);
impl PLLI2SDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLI2SDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLI2SDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLI2SDIV` writer - PLLI2S division factor for SAI1 clock"]
pub struct PLLI2SDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `PLLSAIDIVQ` reader - PLLSAI division factor for SAI1 clock"]
pub struct PLLSAIDIVQ_R(crate::FieldReader<u8, u8>);
impl PLLSAIDIVQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSAIDIVQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAIDIVQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAIDIVQ` writer - PLLSAI division factor for SAI1 clock"]
pub struct PLLSAIDIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIDIVQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `PLLSAIDIVR` reader - division factor for LCD_CLK"]
pub struct PLLSAIDIVR_R(crate::FieldReader<u8, u8>);
impl PLLSAIDIVR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSAIDIVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAIDIVR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAIDIVR` writer - division factor for LCD_CLK"]
pub struct PLLSAIDIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIDIVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `SAI1SEL` reader - SAI1 clock source selection"]
pub struct SAI1SEL_R(crate::FieldReader<u8, u8>);
impl SAI1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAI1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1SEL` writer - SAI1 clock source selection"]
pub struct SAI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `SAI2SEL` reader - SAI2 clock source selection"]
pub struct SAI2SEL_R(crate::FieldReader<u8, u8>);
impl SAI2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAI2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAI2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI2SEL` writer - SAI2 clock source selection"]
pub struct SAI2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `TIMPRE` reader - Timers clocks prescalers selection"]
pub struct TIMPRE_R(crate::FieldReader<bool, bool>);
impl TIMPRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMPRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMPRE` writer - Timers clocks prescalers selection"]
pub struct TIMPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sdiv(&self) -> PLLI2SDIV_R {
        PLLI2SDIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaidivq(&self) -> PLLSAIDIVQ_R {
        PLLSAIDIVQ_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline(always)]
    pub fn pllsaidivr(&self) -> PLLSAIDIVR_R {
        PLLSAIDIVR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLLI2S division factor for SAI1 clock"]
    #[inline(always)]
    pub fn plli2sdiv(&mut self) -> PLLI2SDIV_W {
        PLLI2SDIV_W { w: self }
    }
    #[doc = "Bits 8:12 - PLLSAI division factor for SAI1 clock"]
    #[inline(always)]
    pub fn pllsaidivq(&mut self) -> PLLSAIDIVQ_W {
        PLLSAIDIVQ_W { w: self }
    }
    #[doc = "Bits 16:17 - division factor for LCD_CLK"]
    #[inline(always)]
    pub fn pllsaidivr(&mut self) -> PLLSAIDIVR_W {
        PLLSAIDIVR_W { w: self }
    }
    #[doc = "Bits 20:21 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W {
        SAI1SEL_W { w: self }
    }
    #[doc = "Bits 22:23 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn sai2sel(&mut self) -> SAI2SEL_W {
        SAI2SEL_W { w: self }
    }
    #[doc = "Bit 24 - Timers clocks prescalers selection"]
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W {
        TIMPRE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dedicated clocks configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dkcfgr1](index.html) module"]
pub struct DKCFGR1_SPEC;
impl crate::RegisterSpec for DKCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dkcfgr1::R](R) reader structure"]
impl crate::Readable for DKCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dkcfgr1::W](W) writer structure"]
impl crate::Writable for DKCFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DKCFGR1 to value 0x2000_3000"]
impl crate::Resettable for DKCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_3000
    }
}
