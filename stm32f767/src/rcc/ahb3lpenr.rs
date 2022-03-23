#[doc = "Register `AHB3LPENR` reader"]
pub struct R(crate::R<AHB3LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB3LPENR` writer"]
pub struct W(crate::W<AHB3LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3LPENR_SPEC>;
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
impl From<crate::W<AHB3LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCLPEN` reader - Flexible memory controller module clock enable during Sleep mode"]
pub struct FMCLPEN_R(crate::FieldReader<bool, bool>);
impl FMCLPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FMCLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FMCLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMCLPEN` writer - Flexible memory controller module clock enable during Sleep mode"]
pub struct FMCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCLPEN_W<'a> {
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
#[doc = "Field `QSPILPEN` reader - Quand SPI memory controller clock enable during Sleep mode"]
pub struct QSPILPEN_R(crate::FieldReader<bool, bool>);
impl QSPILPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QSPILPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QSPILPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPILPEN` writer - Quand SPI memory controller clock enable during Sleep mode"]
pub struct QSPILPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPILPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Quand SPI memory controller clock enable during Sleep mode"]
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flexible memory controller module clock enable during Sleep mode"]
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FMCLPEN_W {
        FMCLPEN_W { w: self }
    }
    #[doc = "Bit 1 - Quand SPI memory controller clock enable during Sleep mode"]
    #[inline(always)]
    pub fn qspilpen(&mut self) -> QSPILPEN_W {
        QSPILPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB3 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3lpenr](index.html) module"]
pub struct AHB3LPENR_SPEC;
impl crate::RegisterSpec for AHB3LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3lpenr::R](R) reader structure"]
impl crate::Readable for AHB3LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb3lpenr::W](W) writer structure"]
impl crate::Writable for AHB3LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB3LPENR to value 0x01"]
impl crate::Resettable for AHB3LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
