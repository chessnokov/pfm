#[doc = "Register `MDIOS_DINR1` reader"]
pub struct R(crate::R<MDIOS_DINR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_DINR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_DINR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_DINR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN1` reader - Input data received from MDIO Master during write frames"]
pub struct DIN1_R(crate::FieldReader<u16, u16>);
impl DIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIN1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din1(&self) -> DIN1_R {
        DIN1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_dinr1](index.html) module"]
pub struct MDIOS_DINR1_SPEC;
impl crate::RegisterSpec for MDIOS_DINR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_dinr1::R](R) reader structure"]
impl crate::Readable for MDIOS_DINR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDIOS_DINR1 to value 0"]
impl crate::Resettable for MDIOS_DINR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
