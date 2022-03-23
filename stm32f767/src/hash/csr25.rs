#[doc = "Register `CSR25` reader"]
pub struct R(crate::R<CSR25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR25_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR25` writer"]
pub struct W(crate::W<CSR25_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR25_SPEC>;
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
impl From<crate::W<CSR25_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR25_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR25` reader - CSR25"]
pub struct CSR25_R(crate::FieldReader<u32, u32>);
impl CSR25_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CSR25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSR25_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSR25` writer - CSR25"]
pub struct CSR25_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR25"]
    #[inline(always)]
    pub fn csr25(&self) -> CSR25_R {
        CSR25_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR25"]
    #[inline(always)]
    pub fn csr25(&mut self) -> CSR25_W {
        CSR25_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr25](index.html) module"]
pub struct CSR25_SPEC;
impl crate::RegisterSpec for CSR25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr25::R](R) reader structure"]
impl crate::Readable for CSR25_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr25::W](W) writer structure"]
impl crate::Writable for CSR25_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR25 to value 0"]
impl crate::Resettable for CSR25_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
