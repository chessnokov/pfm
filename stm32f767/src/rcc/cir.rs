#[doc = "Register `CIR` reader"]
pub struct R(crate::R<CIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR` writer"]
pub struct W(crate::W<CIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_SPEC>;
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
impl From<crate::W<CIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub struct CSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PLLSAIRDYC` writer - PLLSAI Ready Interrupt Clear"]
pub struct PLLSAIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIRDYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PLLI2SRDYC` writer - PLLI2S ready interrupt clear"]
pub struct PLLI2SRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SRDYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PLLRDYC` writer - Main PLL(PLL) ready interrupt clear"]
pub struct PLLRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRDYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub struct HSERDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub struct HSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear"]
pub struct LSERDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub struct LSIRDYC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PLLSAIRDYIE` reader - PLLSAI Ready Interrupt Enable"]
pub struct PLLSAIRDYIE_R(crate::FieldReader<bool, bool>);
impl PLLSAIRDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAIRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAIRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAIRDYIE` writer - PLLSAI Ready Interrupt Enable"]
pub struct PLLSAIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSAIRDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PLLI2SRDYIE` reader - PLLI2S ready interrupt enable"]
pub struct PLLI2SRDYIE_R(crate::FieldReader<bool, bool>);
impl PLLI2SRDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLI2SRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLI2SRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLI2SRDYIE` writer - PLLI2S ready interrupt enable"]
pub struct PLLI2SRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLI2SRDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PLLRDYIE` reader - Main PLL (PLL) ready interrupt enable"]
pub struct PLLRDYIE_R(crate::FieldReader<bool, bool>);
impl PLLRDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLRDYIE` writer - Main PLL (PLL) ready interrupt enable"]
pub struct PLLRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLRDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub struct HSERDYIE_R(crate::FieldReader<bool, bool>);
impl HSERDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSERDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSERDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub struct HSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSERDYIE_W<'a> {
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
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub struct HSIRDYIE_R(crate::FieldReader<bool, bool>);
impl HSIRDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub struct HSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIRDYIE_W<'a> {
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
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable"]
pub struct LSERDYIE_R(crate::FieldReader<bool, bool>);
impl LSERDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSERDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSERDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub struct LSERDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub struct LSIRDYIE_R(crate::FieldReader<bool, bool>);
impl LSIRDYIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSIRDYIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub struct LSIRDYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSIRDYIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub struct CSSF_R(crate::FieldReader<bool, bool>);
impl CSSF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSSF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSAIRDYF` reader - PLLSAI ready interrupt flag"]
pub struct PLLSAIRDYF_R(crate::FieldReader<bool, bool>);
impl PLLSAIRDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLSAIRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSAIRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLI2SRDYF` reader - PLLI2S ready interrupt flag"]
pub struct PLLI2SRDYF_R(crate::FieldReader<bool, bool>);
impl PLLI2SRDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLI2SRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLI2SRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLRDYF` reader - Main PLL (PLL) ready interrupt flag"]
pub struct PLLRDYF_R(crate::FieldReader<bool, bool>);
impl PLLRDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PLLRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub struct HSERDYF_R(crate::FieldReader<bool, bool>);
impl HSERDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSERDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSERDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub struct HSIRDYF_R(crate::FieldReader<bool, bool>);
impl HSIRDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSIRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSIRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub struct LSERDYF_R(crate::FieldReader<bool, bool>);
impl LSERDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSERDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSERDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub struct LSIRDYF_R(crate::FieldReader<bool, bool>);
impl LSIRDYF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LSIRDYF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSIRDYF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 14 - PLLSAI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn pllsairdyie(&self) -> PLLSAIRDYIE_R {
        PLLSAIRDYIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PLLI2S ready interrupt enable"]
    #[inline(always)]
    pub fn plli2srdyie(&self) -> PLLI2SRDYIE_R {
        PLLI2SRDYIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Main PLL (PLL) ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PLLSAI ready interrupt flag"]
    #[inline(always)]
    pub fn pllsairdyf(&self) -> PLLSAIRDYF_R {
        PLLSAIRDYF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PLLI2S ready interrupt flag"]
    #[inline(always)]
    pub fn plli2srdyf(&self) -> PLLI2SRDYF_R {
        PLLI2SRDYF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Main PLL (PLL) ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    pub fn cssc(&mut self) -> CSSC_W {
        CSSC_W { w: self }
    }
    #[doc = "Bit 22 - PLLSAI Ready Interrupt Clear"]
    #[inline(always)]
    pub fn pllsairdyc(&mut self) -> PLLSAIRDYC_W {
        PLLSAIRDYC_W { w: self }
    }
    #[doc = "Bit 21 - PLLI2S ready interrupt clear"]
    #[inline(always)]
    pub fn plli2srdyc(&mut self) -> PLLI2SRDYC_W {
        PLLI2SRDYC_W { w: self }
    }
    #[doc = "Bit 20 - Main PLL(PLL) ready interrupt clear"]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W {
        PLLRDYC_W { w: self }
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HSERDYC_W {
        HSERDYC_W { w: self }
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W {
        HSIRDYC_W { w: self }
    }
    #[doc = "Bit 17 - LSE ready interrupt clear"]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LSERDYC_W {
        LSERDYC_W { w: self }
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W {
        LSIRDYC_W { w: self }
    }
    #[doc = "Bit 14 - PLLSAI Ready Interrupt Enable"]
    #[inline(always)]
    pub fn pllsairdyie(&mut self) -> PLLSAIRDYIE_W {
        PLLSAIRDYIE_W { w: self }
    }
    #[doc = "Bit 13 - PLLI2S ready interrupt enable"]
    #[inline(always)]
    pub fn plli2srdyie(&mut self) -> PLLI2SRDYIE_W {
        PLLI2SRDYIE_W { w: self }
    }
    #[doc = "Bit 12 - Main PLL (PLL) ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W {
        PLLRDYIE_W { w: self }
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W {
        HSERDYIE_W { w: self }
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W {
        HSIRDYIE_W { w: self }
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W {
        LSERDYIE_W { w: self }
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W {
        LSIRDYIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir](index.html) module"]
pub struct CIR_SPEC;
impl crate::RegisterSpec for CIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir::R](R) reader structure"]
impl crate::Readable for CIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir::W](W) writer structure"]
impl crate::Writable for CIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
