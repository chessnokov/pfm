#[doc = "Register `LIFCR` reader"]
pub struct R(crate::R<LIFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIFCR` writer"]
pub struct W(crate::W<LIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIFCR_SPEC>;
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
impl From<crate::W<LIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTCIF3` reader - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF3_R(crate::FieldReader<bool, bool>);
impl CTCIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF3` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF3_W<'a> {
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
#[doc = "Field `CHTIF3` reader - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF3_R(crate::FieldReader<bool, bool>);
impl CHTIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTIF3` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF3_W<'a> {
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
#[doc = "Field `CTEIF3` reader - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF3_R(crate::FieldReader<bool, bool>);
impl CTEIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF3` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF3_W<'a> {
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
#[doc = "Field `CDMEIF3` reader - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF3_R(crate::FieldReader<bool, bool>);
impl CDMEIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDMEIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMEIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMEIF3` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF3_W<'a> {
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
#[doc = "Field `CFEIF3` reader - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF3_R(crate::FieldReader<bool, bool>);
impl CFEIF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFEIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFEIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFEIF3` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF3_W<'a> {
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
#[doc = "Field `CTCIF2` reader - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF2_R(crate::FieldReader<bool, bool>);
impl CTCIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF2` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF2_W<'a> {
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
#[doc = "Field `CHTIF2` reader - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF2_R(crate::FieldReader<bool, bool>);
impl CHTIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTIF2` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF2_W<'a> {
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
#[doc = "Field `CTEIF2` reader - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF2_R(crate::FieldReader<bool, bool>);
impl CTEIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF2` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF2_W<'a> {
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
#[doc = "Field `CDMEIF2` reader - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF2_R(crate::FieldReader<bool, bool>);
impl CDMEIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDMEIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMEIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMEIF2` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF2_W<'a> {
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
#[doc = "Field `CFEIF2` reader - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF2_R(crate::FieldReader<bool, bool>);
impl CFEIF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFEIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFEIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFEIF2` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF2_W<'a> {
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
#[doc = "Field `CTCIF1` reader - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF1_R(crate::FieldReader<bool, bool>);
impl CTCIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF1` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF1_W<'a> {
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
#[doc = "Field `CHTIF1` reader - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF1_R(crate::FieldReader<bool, bool>);
impl CHTIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTIF1` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF1_W<'a> {
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
#[doc = "Field `CTEIF1` reader - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF1_R(crate::FieldReader<bool, bool>);
impl CTEIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF1` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF1_W<'a> {
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
#[doc = "Field `CDMEIF1` reader - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF1_R(crate::FieldReader<bool, bool>);
impl CDMEIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDMEIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMEIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMEIF1` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF1_W<'a> {
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
#[doc = "Field `CFEIF1` reader - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF1_R(crate::FieldReader<bool, bool>);
impl CFEIF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFEIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFEIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFEIF1` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF1_W<'a> {
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
#[doc = "Field `CTCIF0` reader - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF0_R(crate::FieldReader<bool, bool>);
impl CTCIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTCIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTCIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTCIF0` writer - Stream x clear transfer complete interrupt flag (x = 3..0)"]
pub struct CTCIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTCIF0_W<'a> {
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
#[doc = "Field `CHTIF0` reader - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF0_R(crate::FieldReader<bool, bool>);
impl CHTIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHTIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHTIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHTIF0` writer - Stream x clear half transfer interrupt flag (x = 3..0)"]
pub struct CHTIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHTIF0_W<'a> {
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
#[doc = "Field `CTEIF0` reader - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF0_R(crate::FieldReader<bool, bool>);
impl CTEIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTEIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTEIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTEIF0` writer - Stream x clear transfer error interrupt flag (x = 3..0)"]
pub struct CTEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEIF0_W<'a> {
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
#[doc = "Field `CDMEIF0` reader - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF0_R(crate::FieldReader<bool, bool>);
impl CDMEIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDMEIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CDMEIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDMEIF0` writer - Stream x clear direct mode error interrupt flag (x = 3..0)"]
pub struct CDMEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CDMEIF0_W<'a> {
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
#[doc = "Field `CFEIF0` reader - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF0_R(crate::FieldReader<bool, bool>);
impl CFEIF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFEIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFEIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFEIF0` writer - Stream x clear FIFO error interrupt flag (x = 3..0)"]
pub struct CFEIF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFEIF0_W<'a> {
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
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif3(&self) -> CTCIF3_R {
        CTCIF3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif3(&self) -> CHTIF3_R {
        CHTIF3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif3(&self) -> CTEIF3_R {
        CTEIF3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif3(&self) -> CDMEIF3_R {
        CDMEIF3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif3(&self) -> CFEIF3_R {
        CFEIF3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif2(&self) -> CTCIF2_R {
        CTCIF2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif2(&self) -> CHTIF2_R {
        CHTIF2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif2(&self) -> CTEIF2_R {
        CTEIF2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif2(&self) -> CDMEIF2_R {
        CDMEIF2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif2(&self) -> CFEIF2_R {
        CFEIF2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif1(&self) -> CTCIF1_R {
        CTCIF1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif1(&self) -> CHTIF1_R {
        CHTIF1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif1(&self) -> CTEIF1_R {
        CTEIF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif1(&self) -> CDMEIF1_R {
        CDMEIF1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif1(&self) -> CFEIF1_R {
        CFEIF1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif0(&self) -> CTCIF0_R {
        CTCIF0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif0(&self) -> CHTIF0_R {
        CHTIF0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif0(&self) -> CTEIF0_R {
        CTEIF0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif0(&self) -> CDMEIF0_R {
        CDMEIF0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif0(&self) -> CFEIF0_R {
        CFEIF0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W {
        CTCIF3_W { w: self }
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W {
        CHTIF3_W { w: self }
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W {
        CTEIF3_W { w: self }
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif3(&mut self) -> CDMEIF3_W {
        CDMEIF3_W { w: self }
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif3(&mut self) -> CFEIF3_W {
        CFEIF3_W { w: self }
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W {
        CTCIF2_W { w: self }
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W {
        CHTIF2_W { w: self }
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W {
        CTEIF2_W { w: self }
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif2(&mut self) -> CDMEIF2_W {
        CDMEIF2_W { w: self }
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif2(&mut self) -> CFEIF2_W {
        CFEIF2_W { w: self }
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W {
        CTCIF1_W { w: self }
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W {
        CHTIF1_W { w: self }
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W {
        CTEIF1_W { w: self }
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif1(&mut self) -> CDMEIF1_W {
        CDMEIF1_W { w: self }
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif1(&mut self) -> CFEIF1_W {
        CFEIF1_W { w: self }
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn ctcif0(&mut self) -> CTCIF0_W {
        CTCIF0_W { w: self }
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn chtif0(&mut self) -> CHTIF0_W {
        CHTIF0_W { w: self }
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF0_W {
        CTEIF0_W { w: self }
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cdmeif0(&mut self) -> CDMEIF0_W {
        CDMEIF0_W { w: self }
    }
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn cfeif0(&mut self) -> CFEIF0_W {
        CFEIF0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "low interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lifcr](index.html) module"]
pub struct LIFCR_SPEC;
impl crate::RegisterSpec for LIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lifcr::R](R) reader structure"]
impl crate::Readable for LIFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lifcr::W](W) writer structure"]
impl crate::Writable for LIFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIFCR to value 0"]
impl crate::Resettable for LIFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
