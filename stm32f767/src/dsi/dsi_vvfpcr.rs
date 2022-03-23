#[doc = "Register `DSI_VVFPCR` reader"]
pub struct R(crate::R<DSI_VVFPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_VVFPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_VVFPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_VVFPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_VVFPCR` writer"]
pub struct W(crate::W<DSI_VVFPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_VVFPCR_SPEC>;
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
impl From<crate::W<DSI_VVFPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_VVFPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VFP` reader - Vertical Front-Porch duration"]
pub struct VFP_R(crate::FieldReader<u16, u16>);
impl VFP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VFP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VFP` writer - Vertical Front-Porch duration"]
pub struct VFP_W<'a> {
    w: &'a mut W,
}
impl<'a> VFP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Vertical Front-Porch duration"]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Front-Porch duration"]
    #[inline(always)]
    pub fn vfp(&mut self) -> VFP_W {
        VFP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video VFP Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vvfpcr](index.html) module"]
pub struct DSI_VVFPCR_SPEC;
impl crate::RegisterSpec for DSI_VVFPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_vvfpcr::R](R) reader structure"]
impl crate::Readable for DSI_VVFPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_vvfpcr::W](W) writer structure"]
impl crate::Writable for DSI_VVFPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSI_VVFPCR to value 0"]
impl crate::Resettable for DSI_VVFPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
