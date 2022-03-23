#[doc = "Register `DSI_VVSACCR` reader"]
pub struct R(crate::R<DSI_VVSACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_VVSACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_VVSACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_VVSACCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VSA` reader - Vertical Synchronism Active duration"]
pub struct VSA_R(crate::FieldReader<u16, u16>);
impl VSA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Vertical Synchronism Active duration"]
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host Video VSA Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vvsaccr](index.html) module"]
pub struct DSI_VVSACCR_SPEC;
impl crate::RegisterSpec for DSI_VVSACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_vvsaccr::R](R) reader structure"]
impl crate::Readable for DSI_VVSACCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_VVSACCR to value 0"]
impl crate::Resettable for DSI_VVSACCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
