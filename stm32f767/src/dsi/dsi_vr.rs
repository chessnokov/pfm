#[doc = "Register `DSI_VR` reader"]
pub struct R(crate::R<DSI_VR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_VR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_VR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_VR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Version of the DSI Host"]
pub struct VERSION_R(crate::FieldReader<u32, u32>);
impl VERSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VERSION_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Version of the DSI Host"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(self.bits)
    }
}
#[doc = "DSI Host Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vr](index.html) module"]
pub struct DSI_VR_SPEC;
impl crate::RegisterSpec for DSI_VR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_vr::R](R) reader structure"]
impl crate::Readable for DSI_VR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_VR to value 0x3133_302a"]
impl crate::Resettable for DSI_VR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3133_302a
    }
}
