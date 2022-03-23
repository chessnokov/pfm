#[doc = "Register `DSI_TCCR0` reader"]
pub struct R(crate::R<DSI_TCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_TCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_TCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_TCCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_TCCR0` writer"]
pub struct W(crate::W<DSI_TCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_TCCR0_SPEC>;
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
impl From<crate::W<DSI_TCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_TCCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPRX_TOCNT` reader - Low-power Reception Timeout Counter"]
pub struct LPRX_TOCNT_R(crate::FieldReader<u16, u16>);
impl LPRX_TOCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LPRX_TOCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPRX_TOCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPRX_TOCNT` writer - Low-power Reception Timeout Counter"]
pub struct LPRX_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRX_TOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `HSTX_TOCNT` reader - High-Speed Transmission Timeout Counter"]
pub struct HSTX_TOCNT_R(crate::FieldReader<u16, u16>);
impl HSTX_TOCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        HSTX_TOCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTX_TOCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTX_TOCNT` writer - High-Speed Transmission Timeout Counter"]
pub struct HSTX_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTX_TOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low-power Reception Timeout Counter"]
    #[inline(always)]
    pub fn lprx_tocnt(&self) -> LPRX_TOCNT_R {
        LPRX_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High-Speed Transmission Timeout Counter"]
    #[inline(always)]
    pub fn hstx_tocnt(&self) -> HSTX_TOCNT_R {
        HSTX_TOCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low-power Reception Timeout Counter"]
    #[inline(always)]
    pub fn lprx_tocnt(&mut self) -> LPRX_TOCNT_W {
        LPRX_TOCNT_W { w: self }
    }
    #[doc = "Bits 16:31 - High-Speed Transmission Timeout Counter"]
    #[inline(always)]
    pub fn hstx_tocnt(&mut self) -> HSTX_TOCNT_W {
        HSTX_TOCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host Timeout Counter Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_tccr0](index.html) module"]
pub struct DSI_TCCR0_SPEC;
impl crate::RegisterSpec for DSI_TCCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_tccr0::R](R) reader structure"]
impl crate::Readable for DSI_TCCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_tccr0::W](W) writer structure"]
impl crate::Writable for DSI_TCCR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSI_TCCR0 to value 0"]
impl crate::Resettable for DSI_TCCR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
