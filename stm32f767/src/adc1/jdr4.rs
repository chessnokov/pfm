#[doc = "Register `JDR4` reader"]
pub struct R(crate::R<JDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA` reader - Injected data"]
pub struct JDATA_R(crate::FieldReader<u16, u16>);
impl JDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        JDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Injected data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr4](index.html) module"]
pub struct JDR4_SPEC;
impl crate::RegisterSpec for JDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdr4::R](R) reader structure"]
impl crate::Readable for JDR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDR4 to value 0"]
impl crate::Resettable for JDR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
