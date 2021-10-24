#[doc = "Register `DSI_VLCCR` reader"]
pub struct R(crate::R<DSI_VLCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_VLCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_VLCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_VLCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HLINE` reader - Horizontal Line duration"]
pub struct HLINE_R(crate::FieldReader<u16, u16>);
impl HLINE_R {
    pub(crate) fn new(bits: u16) -> Self {
        HLINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HLINE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:14 - Horizontal Line duration"]
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "DSI Host Video Line Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vlccr](index.html) module"]
pub struct DSI_VLCCR_SPEC;
impl crate::RegisterSpec for DSI_VLCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_vlccr::R](R) reader structure"]
impl crate::Readable for DSI_VLCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_VLCCR to value 0"]
impl crate::Resettable for DSI_VLCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
