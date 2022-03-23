#[doc = "Register `DSI_VHSACCR` reader"]
pub struct R(crate::R<DSI_VHSACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_VHSACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_VHSACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_VHSACCR_SPEC>) -> Self {
        R(reader)
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
impl R {
    #[doc = "Bits 0:11 - Horizontal Synchronism Active duration"]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host Video HSA Current Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_vhsaccr](index.html) module"]
pub struct DSI_VHSACCR_SPEC;
impl crate::RegisterSpec for DSI_VHSACCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_vhsaccr::R](R) reader structure"]
impl crate::Readable for DSI_VHSACCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DSI_VHSACCR to value 0"]
impl crate::Resettable for DSI_VHSACCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
