#[doc = "Register `PLLCFGR` reader"]
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFGR` writer"]
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
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
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLQ3` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub struct PLLQ3_R(crate::FieldReader<bool, bool>);
impl PLLQ3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLQ3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLQ3` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub struct PLLQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PLLQ2` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub struct PLLQ2_R(crate::FieldReader<bool, bool>);
impl PLLQ2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLQ2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLQ2` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub struct PLLQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PLLQ1` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub struct PLLQ1_R(crate::FieldReader<bool, bool>);
impl PLLQ1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLQ1` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub struct PLLQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PLLQ0` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub struct PLLQ0_R(crate::FieldReader<bool, bool>);
impl PLLQ0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLQ0` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub struct PLLQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLQ0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PLLSRC` reader - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
pub struct PLLSRC_R(crate::FieldReader<bool, bool>);
impl PLLSRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLSRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSRC` writer - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
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
#[doc = "Field `PLLP1` reader - Main PLL (PLL) division factor for main system clock"]
pub struct PLLP1_R(crate::FieldReader<bool, bool>);
impl PLLP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLP1` writer - Main PLL (PLL) division factor for main system clock"]
pub struct PLLP1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLP1_W<'a> {
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
#[doc = "Field `PLLP0` reader - Main PLL (PLL) division factor for main system clock"]
pub struct PLLP0_R(crate::FieldReader<bool, bool>);
impl PLLP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLP0` writer - Main PLL (PLL) division factor for main system clock"]
pub struct PLLP0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLP0_W<'a> {
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
#[doc = "Field `PLLN8` reader - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN8_R(crate::FieldReader<bool, bool>);
impl PLLN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN8` writer - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN8_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN8_W<'a> {
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
#[doc = "Field `PLLN7` reader - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN7_R(crate::FieldReader<bool, bool>);
impl PLLN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN7` writer - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN7_W<'a> {
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
#[doc = "Field `PLLN6` reader - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN6_R(crate::FieldReader<bool, bool>);
impl PLLN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN6` writer - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN6_W<'a> {
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
#[doc = "Field `PLLN5` reader - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN5_R(crate::FieldReader<bool, bool>);
impl PLLN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN5` writer - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN5_W<'a> {
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
#[doc = "Field `PLLN4` reader - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN4_R(crate::FieldReader<bool, bool>);
impl PLLN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN4` writer - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN4_W<'a> {
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
#[doc = "Field `PLLN3` reader - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN3_R(crate::FieldReader<bool, bool>);
impl PLLN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN3` writer - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN3_W<'a> {
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
#[doc = "Field `PLLN2` reader - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN2_R(crate::FieldReader<bool, bool>);
impl PLLN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN2` writer - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN2_W<'a> {
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
#[doc = "Field `PLLN1` reader - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN1_R(crate::FieldReader<bool, bool>);
impl PLLN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN1` writer - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN1_W<'a> {
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
#[doc = "Field `PLLN0` reader - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN0_R(crate::FieldReader<bool, bool>);
impl PLLN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLN0` writer - Main PLL (PLL) multiplication factor for VCO"]
pub struct PLLN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PLLM5` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM5_R(crate::FieldReader<bool, bool>);
impl PLLM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLM5` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM5_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PLLM4` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM4_R(crate::FieldReader<bool, bool>);
impl PLLM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLM4` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM4_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PLLM3` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM3_R(crate::FieldReader<bool, bool>);
impl PLLM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLM3` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM3_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM3_W<'a> {
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
#[doc = "Field `PLLM2` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM2_R(crate::FieldReader<bool, bool>);
impl PLLM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLM2` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM2_W<'a> {
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
#[doc = "Field `PLLM1` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM1_R(crate::FieldReader<bool, bool>);
impl PLLM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLM1` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PLLM0` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM0_R(crate::FieldReader<bool, bool>);
impl PLLM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLM0` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub struct PLLM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLM0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq3(&self) -> PLLQ3_R {
        PLLQ3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq2(&self) -> PLLQ2_R {
        PLLQ2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq1(&self) -> PLLQ1_R {
        PLLQ1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq0(&self) -> PLLQ0_R {
        PLLQ0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp1(&self) -> PLLP1_R {
        PLLP1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp0(&self) -> PLLP0_R {
        PLLP0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln8(&self) -> PLLN8_R {
        PLLN8_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln7(&self) -> PLLN7_R {
        PLLN7_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln6(&self) -> PLLN6_R {
        PLLN6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln5(&self) -> PLLN5_R {
        PLLN5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln4(&self) -> PLLN4_R {
        PLLN4_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln3(&self) -> PLLN3_R {
        PLLN3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln2(&self) -> PLLN2_R {
        PLLN2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln1(&self) -> PLLN1_R {
        PLLN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln0(&self) -> PLLN0_R {
        PLLN0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm5(&self) -> PLLM5_R {
        PLLM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm4(&self) -> PLLM4_R {
        PLLM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm3(&self) -> PLLM3_R {
        PLLM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm2(&self) -> PLLM2_R {
        PLLM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm1(&self) -> PLLM1_R {
        PLLM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm0(&self) -> PLLM0_R {
        PLLM0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq3(&mut self) -> PLLQ3_W {
        PLLQ3_W { w: self }
    }
    #[doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq2(&mut self) -> PLLQ2_W {
        PLLQ2_W { w: self }
    }
    #[doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq1(&mut self) -> PLLQ1_W {
        PLLQ1_W { w: self }
    }
    #[doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq0(&mut self) -> PLLQ0_W {
        PLLQ0_W { w: self }
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Bit 17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp1(&mut self) -> PLLP1_W {
        PLLP1_W { w: self }
    }
    #[doc = "Bit 16 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp0(&mut self) -> PLLP0_W {
        PLLP0_W { w: self }
    }
    #[doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln8(&mut self) -> PLLN8_W {
        PLLN8_W { w: self }
    }
    #[doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln7(&mut self) -> PLLN7_W {
        PLLN7_W { w: self }
    }
    #[doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln6(&mut self) -> PLLN6_W {
        PLLN6_W { w: self }
    }
    #[doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln5(&mut self) -> PLLN5_W {
        PLLN5_W { w: self }
    }
    #[doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln4(&mut self) -> PLLN4_W {
        PLLN4_W { w: self }
    }
    #[doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln3(&mut self) -> PLLN3_W {
        PLLN3_W { w: self }
    }
    #[doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln2(&mut self) -> PLLN2_W {
        PLLN2_W { w: self }
    }
    #[doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln1(&mut self) -> PLLN1_W {
        PLLN1_W { w: self }
    }
    #[doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln0(&mut self) -> PLLN0_W {
        PLLN0_W { w: self }
    }
    #[doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm5(&mut self) -> PLLM5_W {
        PLLM5_W { w: self }
    }
    #[doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm4(&mut self) -> PLLM4_W {
        PLLM4_W { w: self }
    }
    #[doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm3(&mut self) -> PLLM3_W {
        PLLM3_W { w: self }
    }
    #[doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm2(&mut self) -> PLLM2_W {
        PLLM2_W { w: self }
    }
    #[doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm1(&mut self) -> PLLM1_W {
        PLLM1_W { w: self }
    }
    #[doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm0(&mut self) -> PLLM0_W {
        PLLM0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](index.html) module"]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfgr::R](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x2400_3010"]
impl crate::Resettable for PLLCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2400_3010
    }
}
