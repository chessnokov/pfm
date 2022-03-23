#[doc = "Register `HR2` reader"]
pub struct R(crate::R<HR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H2` reader - H2"]
pub struct H2_R(crate::FieldReader<u32, u32>);
impl H2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        H2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for H2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - H2"]
    #[inline(always)]
    pub fn h2(&self) -> H2_R {
        H2_R::new(self.bits)
    }
}
#[doc = "digest registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr2](index.html) module"]
pub struct HR2_SPEC;
impl crate::RegisterSpec for HR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hr2::R](R) reader structure"]
impl crate::Readable for HR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HR2 to value 0"]
impl crate::Resettable for HR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
