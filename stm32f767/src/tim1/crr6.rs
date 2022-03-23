#[doc = "Register `CRR6` reader"]
pub struct R(crate::R<CRR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRR6` writer"]
pub struct W(crate::W<CRR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRR6_SPEC>;
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
impl From<crate::W<CRR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR6` reader - Capture/Compare 6 value"]
pub struct CCR6_R(crate::FieldReader<u16, u16>);
impl CCR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CCR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR6_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR6` writer - Capture/Compare 6 value"]
pub struct CCR6_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Capture/Compare 6 value"]
    #[inline(always)]
    pub fn ccr6(&self) -> CCR6_R {
        CCR6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture/Compare 6 value"]
    #[inline(always)]
    pub fn ccr6(&mut self) -> CCR6_W {
        CCR6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crr6](index.html) module"]
pub struct CRR6_SPEC;
impl crate::RegisterSpec for CRR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crr6::R](R) reader structure"]
impl crate::Readable for CRR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crr6::W](W) writer structure"]
impl crate::Writable for CRR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRR6 to value 0"]
impl crate::Resettable for CRR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
