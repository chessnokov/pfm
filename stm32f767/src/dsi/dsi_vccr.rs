#[doc = "Register `DSI_VCCR` reader"]
pub struct R(crate::R<DSI_VCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_VCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_VCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_VCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_VCCR` writer"]
pub struct W(crate::W<DSI_VCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_VCCR_SPEC>;
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
impl From<crate::W<DSI_VCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_VCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUMC` reader - Number of Chunks"]
pub struct NUMC_R(crate::FieldReader<u16, u16>);
impl NUMC_R {
    pub(crate) fn new(bits: u16) -> Self {
        NUMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUMC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUMC` writer - Number of Chunks"]
pub struct NUMC_W<'a> {
    w: &'a mut W,
}
impl<'a> NUMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Number of Chunks"]
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Number of Chunks"]
    #[inline(always)]
    pub fn numc(&mut self) -> NUMC_W {
        NUMC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video Chunks Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vccr](index.html) module"]
pub struct DSI_VCCR_SPEC;
impl crate::RegisterSpec for DSI_VCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_vccr::R](R) reader structure"]
impl crate::Readable for DSI_VCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_vccr::W](W) writer structure"]
impl crate::Writable for DSI_VCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSI_VCCR to value 0"]
impl crate::Resettable for DSI_VCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
