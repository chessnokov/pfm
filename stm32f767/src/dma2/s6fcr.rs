#[doc = "Register `S6FCR` reader"]
pub struct R(crate::R<S6FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S6FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S6FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S6FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S6FCR` writer"]
pub struct W(crate::W<S6FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S6FCR_SPEC>;
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
impl From<crate::W<S6FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S6FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEIE` reader - FIFO error interrupt enable"]
pub struct FEIE_R(crate::FieldReader<bool, bool>);
impl FEIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIE` writer - FIFO error interrupt enable"]
pub struct FEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIE_W<'a> {
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
#[doc = "Field `FS` reader - FIFO status"]
pub struct FS_R(crate::FieldReader<u8, u8>);
impl FS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMDIS` reader - Direct mode disable"]
pub struct DMDIS_R(crate::FieldReader<bool, bool>);
impl DMDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMDIS` writer - Direct mode disable"]
pub struct DMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `FTH` reader - FIFO threshold selection"]
pub struct FTH_R(crate::FieldReader<u8, u8>);
impl FTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTH` writer - FIFO threshold selection"]
pub struct FTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&mut self) -> FEIE_W {
        FEIE_W { w: self }
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&mut self) -> DMDIS_W {
        DMDIS_W { w: self }
    }
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W {
        FTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s6fcr](index.html) module"]
pub struct S6FCR_SPEC;
impl crate::RegisterSpec for S6FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s6fcr::R](R) reader structure"]
impl crate::Readable for S6FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s6fcr::W](W) writer structure"]
impl crate::Writable for S6FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S6FCR to value 0x21"]
impl crate::Resettable for S6FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x21
    }
}
