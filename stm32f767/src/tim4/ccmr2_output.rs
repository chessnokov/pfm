#[doc = "Register `CCMR2_Output` reader"]
pub struct R(crate::R<CCMR2_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR2_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR2_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR2_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMR2_Output` writer"]
pub struct W(crate::W<CCMR2_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR2_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR2_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR2_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `O24CE` reader - O24CE"]
pub struct O24CE_R(crate::FieldReader<bool, bool>);
impl O24CE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        O24CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for O24CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `O24CE` writer - O24CE"]
pub struct O24CE_W<'a> {
    w: &'a mut W,
}
impl<'a> O24CE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `OC4M` reader - OC4M"]
pub struct OC4M_R(crate::FieldReader<u8, u8>);
impl OC4M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OC4M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC4M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC4M` writer - OC4M"]
pub struct OC4M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `OC4PE` reader - OC4PE"]
pub struct OC4PE_R(crate::FieldReader<bool, bool>);
impl OC4PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OC4PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC4PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC4PE` writer - OC4PE"]
pub struct OC4PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4PE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `OC4FE` reader - OC4FE"]
pub struct OC4FE_R(crate::FieldReader<bool, bool>);
impl OC4FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OC4FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC4FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC4FE` writer - OC4FE"]
pub struct OC4FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4FE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CC4S` reader - CC4S"]
pub struct CC4S_R(crate::FieldReader<u8, u8>);
impl CC4S_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CC4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC4S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC4S` writer - CC4S"]
pub struct CC4S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC4S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `OC3CE` reader - OC3CE"]
pub struct OC3CE_R(crate::FieldReader<bool, bool>);
impl OC3CE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OC3CE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC3CE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC3CE` writer - OC3CE"]
pub struct OC3CE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3CE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `OC3M` reader - OC3M"]
pub struct OC3M_R(crate::FieldReader<u8, u8>);
impl OC3M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OC3M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC3M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC3M` writer - OC3M"]
pub struct OC3M_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `OC3PE` reader - OC3PE"]
pub struct OC3PE_R(crate::FieldReader<bool, bool>);
impl OC3PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OC3PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC3PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC3PE` writer - OC3PE"]
pub struct OC3PE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3PE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `OC3FE` reader - OC3FE"]
pub struct OC3FE_R(crate::FieldReader<bool, bool>);
impl OC3FE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OC3FE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC3FE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC3FE` writer - OC3FE"]
pub struct OC3FE_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3FE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CC3S` reader - CC3S"]
pub struct CC3S_R(crate::FieldReader<u8, u8>);
impl CC3S_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CC3S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC3S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC3S` writer - CC3S"]
pub struct CC3S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - O24CE"]
    #[inline(always)]
    pub fn o24ce(&self) -> O24CE_R {
        O24CE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - OC4M"]
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11 - OC4PE"]
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - OC4FE"]
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - OC3CE"]
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - OC3M"]
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3 - OC3PE"]
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OC3FE"]
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 15 - O24CE"]
    #[inline(always)]
    pub fn o24ce(&mut self) -> O24CE_W {
        O24CE_W { w: self }
    }
    #[doc = "Bits 12:14 - OC4M"]
    #[inline(always)]
    pub fn oc4m(&mut self) -> OC4M_W {
        OC4M_W { w: self }
    }
    #[doc = "Bit 11 - OC4PE"]
    #[inline(always)]
    pub fn oc4pe(&mut self) -> OC4PE_W {
        OC4PE_W { w: self }
    }
    #[doc = "Bit 10 - OC4FE"]
    #[inline(always)]
    pub fn oc4fe(&mut self) -> OC4FE_W {
        OC4FE_W { w: self }
    }
    #[doc = "Bits 8:9 - CC4S"]
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W {
        CC4S_W { w: self }
    }
    #[doc = "Bit 7 - OC3CE"]
    #[inline(always)]
    pub fn oc3ce(&mut self) -> OC3CE_W {
        OC3CE_W { w: self }
    }
    #[doc = "Bits 4:6 - OC3M"]
    #[inline(always)]
    pub fn oc3m(&mut self) -> OC3M_W {
        OC3M_W { w: self }
    }
    #[doc = "Bit 3 - OC3PE"]
    #[inline(always)]
    pub fn oc3pe(&mut self) -> OC3PE_W {
        OC3PE_W { w: self }
    }
    #[doc = "Bit 2 - OC3FE"]
    #[inline(always)]
    pub fn oc3fe(&mut self) -> OC3FE_W {
        OC3FE_W { w: self }
    }
    #[doc = "Bits 0:1 - CC3S"]
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W {
        CC3S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 2 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr2_output](index.html) module"]
pub struct CCMR2_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR2_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmr2_output::R](R) reader structure"]
impl crate::Readable for CCMR2_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmr2_output::W](W) writer structure"]
impl crate::Writable for CCMR2_OUTPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCMR2_Output to value 0"]
impl crate::Resettable for CCMR2_OUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
