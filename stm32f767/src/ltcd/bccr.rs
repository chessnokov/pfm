#[doc = "Register `BCCR` reader"]
pub struct R(crate::R<BCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCCR` writer"]
pub struct W(crate::W<BCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCCR_SPEC>;
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
impl From<crate::W<BCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BC` reader - Background Color Red value"]
pub struct BC_R(crate::FieldReader<u32, u32>);
impl BC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        BC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BC` writer - Background Color Red value"]
pub struct BC_W<'a> {
    w: &'a mut W,
}
impl<'a> BC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Background Color Red value"]
    #[inline(always)]
    pub fn bc(&self) -> BC_R {
        BC_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Background Color Red value"]
    #[inline(always)]
    pub fn bc(&mut self) -> BC_W {
        BC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Background Color Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bccr](index.html) module"]
pub struct BCCR_SPEC;
impl crate::RegisterSpec for BCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bccr::R](R) reader structure"]
impl crate::Readable for BCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bccr::W](W) writer structure"]
impl crate::Writable for BCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCCR to value 0"]
impl crate::Resettable for BCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
