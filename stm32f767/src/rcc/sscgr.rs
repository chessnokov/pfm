#[doc = "Register `SSCGR` reader"]
pub struct R(crate::R<SSCGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSCGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSCGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSCGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSCGR` writer"]
pub struct W(crate::W<SSCGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSCGR_SPEC>;
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
impl From<crate::W<SSCGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSCGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSCGEN` reader - Spread spectrum modulation enable"]
pub struct SSCGEN_R(crate::FieldReader<bool, bool>);
impl SSCGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSCGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSCGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSCGEN` writer - Spread spectrum modulation enable"]
pub struct SSCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSCGEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `SPREADSEL` reader - Spread Select"]
pub struct SPREADSEL_R(crate::FieldReader<bool, bool>);
impl SPREADSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPREADSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPREADSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPREADSEL` writer - Spread Select"]
pub struct SPREADSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPREADSEL_W<'a> {
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
#[doc = "Field `INCSTEP` reader - Incrementation step"]
pub struct INCSTEP_R(crate::FieldReader<u16, u16>);
impl INCSTEP_R {
    pub(crate) fn new(bits: u16) -> Self {
        INCSTEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INCSTEP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INCSTEP` writer - Incrementation step"]
pub struct INCSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INCSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 13)) | ((value as u32 & 0x7fff) << 13);
        self.w
    }
}
#[doc = "Field `MODPER` reader - Modulation period"]
pub struct MODPER_R(crate::FieldReader<u16, u16>);
impl MODPER_R {
    pub(crate) fn new(bits: u16) -> Self {
        MODPER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODPER_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODPER` writer - Modulation period"]
pub struct MODPER_W<'a> {
    w: &'a mut W,
}
impl<'a> MODPER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&self) -> SSCGEN_R {
        SSCGEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&self) -> SPREADSEL_R {
        SPREADSEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&self) -> INCSTEP_R {
        INCSTEP_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&self) -> MODPER_R {
        MODPER_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&mut self) -> SSCGEN_W {
        SSCGEN_W { w: self }
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&mut self) -> SPREADSEL_W {
        SPREADSEL_W { w: self }
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&mut self) -> INCSTEP_W {
        INCSTEP_W { w: self }
    }
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&mut self) -> MODPER_W {
        MODPER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spread spectrum clock generation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sscgr](index.html) module"]
pub struct SSCGR_SPEC;
impl crate::RegisterSpec for SSCGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sscgr::R](R) reader structure"]
impl crate::Readable for SSCGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sscgr::W](W) writer structure"]
impl crate::Writable for SSCGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSCGR to value 0"]
impl crate::Resettable for SSCGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
