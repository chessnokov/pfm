#[doc = "Register `DSI_VPCCR` reader"]
pub struct R(crate::R<DSI_VPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_VPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_VPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_VPCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VPSIZE` reader - Video Packet Size"]
pub struct VPSIZE_R(crate::FieldReader<u16, u16>);
impl VPSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        VPSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VPSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:13 - Video Packet Size"]
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "DSI Host Video Packet Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vpccr](index.html) module"]
pub struct DSI_VPCCR_SPEC;
impl crate::RegisterSpec for DSI_VPCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_vpccr::R](R) reader structure"]
impl crate::Readable for DSI_VPCCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_VPCCR to value 0"]
impl crate::Resettable for DSI_VPCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
