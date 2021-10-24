#[doc = "Register `L2CKCR` reader"]
pub struct R(crate::R<L2CKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2CKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2CKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2CKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L2CKCR` writer"]
pub struct W(crate::W<L2CKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2CKCR_SPEC>;
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
impl From<crate::W<L2CKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2CKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKRED` reader - Color Key Red value"]
pub struct CKRED_R(crate::FieldReader<u16, u16>);
impl CKRED_R {
    pub(crate) fn new(bits: u16) -> Self {
        CKRED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKRED_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKRED` writer - Color Key Red value"]
pub struct CKRED_W<'a> {
    w: &'a mut W,
}
impl<'a> CKRED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 15)) | ((value as u32 & 0x01ff) << 15);
        self.w
    }
}
#[doc = "Field `CKGREEN` reader - Color Key Green value"]
pub struct CKGREEN_R(crate::FieldReader<u8, u8>);
impl CKGREEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKGREEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKGREEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKGREEN` writer - Color Key Green value"]
pub struct CKGREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKGREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `CKBLUE` reader - Color Key Blue value"]
pub struct CKBLUE_R(crate::FieldReader<u8, u8>);
impl CKBLUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKBLUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKBLUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKBLUE` writer - Color Key Blue value"]
pub struct CKBLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKBLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 15:23 - Color Key Red value"]
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 15) & 0x01ff) as u16)
    }
    #[doc = "Bits 8:14 - Color Key Green value"]
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:7 - Color Key Blue value"]
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 15:23 - Color Key Red value"]
    #[inline(always)]
    pub fn ckred(&mut self) -> CKRED_W {
        CKRED_W { w: self }
    }
    #[doc = "Bits 8:14 - Color Key Green value"]
    #[inline(always)]
    pub fn ckgreen(&mut self) -> CKGREEN_W {
        CKGREEN_W { w: self }
    }
    #[doc = "Bits 0:7 - Color Key Blue value"]
    #[inline(always)]
    pub fn ckblue(&mut self) -> CKBLUE_W {
        CKBLUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx Color Keying Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2ckcr](index.html) module"]
pub struct L2CKCR_SPEC;
impl crate::RegisterSpec for L2CKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2ckcr::R](R) reader structure"]
impl crate::Readable for L2CKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l2ckcr::W](W) writer structure"]
impl crate::Writable for L2CKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L2CKCR to value 0"]
impl crate::Resettable for L2CKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}