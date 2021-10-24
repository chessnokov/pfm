#[doc = "Register `DSI_VVBPCR` reader"]
pub struct R(crate::R<DSI_VVBPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_VVBPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_VVBPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_VVBPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_VVBPCR` writer"]
pub struct W(crate::W<DSI_VVBPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_VVBPCR_SPEC>;
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
impl From<crate::W<DSI_VVBPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_VVBPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBP` reader - Vertical Back-Porch duration"]
pub struct VBP_R(crate::FieldReader<u16, u16>);
impl VBP_R {
    pub(crate) fn new(bits: u16) -> Self {
        VBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBP` writer - Vertical Back-Porch duration"]
pub struct VBP_W<'a> {
    w: &'a mut W,
}
impl<'a> VBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Vertical Back-Porch duration"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Back-Porch duration"]
    #[inline(always)]
    pub fn vbp(&mut self) -> VBP_W {
        VBP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video VBP Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vvbpcr](index.html) module"]
pub struct DSI_VVBPCR_SPEC;
impl crate::RegisterSpec for DSI_VVBPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_vvbpcr::R](R) reader structure"]
impl crate::Readable for DSI_VVBPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_vvbpcr::W](W) writer structure"]
impl crate::Writable for DSI_VVBPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSI_VVBPCR to value 0"]
impl crate::Resettable for DSI_VVBPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
