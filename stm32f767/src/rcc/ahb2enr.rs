#[doc = "Register `AHB2ENR` reader"]
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2ENR` writer"]
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTGFSEN` reader - USB OTG FS clock enable"]
pub struct OTGFSEN_R(crate::FieldReader<bool, bool>);
impl OTGFSEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OTGFSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTGFSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTGFSEN` writer - USB OTG FS clock enable"]
pub struct OTGFSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGFSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RNGEN` reader - Random number generator clock enable"]
pub struct RNGEN_R(crate::FieldReader<bool, bool>);
impl RNGEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RNGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGEN` writer - Random number generator clock enable"]
pub struct RNGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `HASHEN` reader - Hash modules clock enable"]
pub struct HASHEN_R(crate::FieldReader<bool, bool>);
impl HASHEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HASHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASHEN` writer - Hash modules clock enable"]
pub struct HASHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CRYPEN` reader - Cryptographic modules clock enable"]
pub struct CRYPEN_R(crate::FieldReader<bool, bool>);
impl CRYPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRYPEN` writer - Cryptographic modules clock enable"]
pub struct CRYPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DCMIEN` reader - Camera interface enable"]
pub struct DCMIEN_R(crate::FieldReader<bool, bool>);
impl DCMIEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCMIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCMIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMIEN` writer - Camera interface enable"]
pub struct DCMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMIEN_W<'a> {
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
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Random number generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Hash modules clock enable"]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Cryptographic modules clock enable"]
    #[inline(always)]
    pub fn crypen(&self) -> CRYPEN_R {
        CRYPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Camera interface enable"]
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline(always)]
    pub fn otgfsen(&mut self) -> OTGFSEN_W {
        OTGFSEN_W { w: self }
    }
    #[doc = "Bit 6 - Random number generator clock enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W {
        RNGEN_W { w: self }
    }
    #[doc = "Bit 5 - Hash modules clock enable"]
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W {
        HASHEN_W { w: self }
    }
    #[doc = "Bit 4 - Cryptographic modules clock enable"]
    #[inline(always)]
    pub fn crypen(&mut self) -> CRYPEN_W {
        CRYPEN_W { w: self }
    }
    #[doc = "Bit 0 - Camera interface enable"]
    #[inline(always)]
    pub fn dcmien(&mut self) -> DCMIEN_W {
        DCMIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](index.html) module"]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2enr::R](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
