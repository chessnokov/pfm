#[doc = "Register `DSI_VHSACR` reader"]
pub struct R(crate::R<DSI_VHSACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_VHSACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_VHSACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_VHSACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_VHSACR` writer"]
pub struct W(crate::W<DSI_VHSACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_VHSACR_SPEC>;
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
impl From<crate::W<DSI_VHSACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_VHSACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSA` reader - Horizontal Synchronism Active duration"]
pub struct HSA_R(crate::FieldReader<u16, u16>);
impl HSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        HSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSA` writer - Horizontal Synchronism Active duration"]
pub struct HSA_W<'a> {
    w: &'a mut W,
}
impl<'a> HSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Horizontal Synchronism Active duration"]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Horizontal Synchronism Active duration"]
    #[inline(always)]
    pub fn hsa(&mut self) -> HSA_W {
        HSA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Video HSA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vhsacr](index.html) module"]
pub struct DSI_VHSACR_SPEC;
impl crate::RegisterSpec for DSI_VHSACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_vhsacr::R](R) reader structure"]
impl crate::Readable for DSI_VHSACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_vhsacr::W](W) writer structure"]
impl crate::Writable for DSI_VHSACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSI_VHSACR to value 0"]
impl crate::Resettable for DSI_VHSACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
