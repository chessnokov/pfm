#[doc = "Register `LISR` reader"]
pub struct R(crate::R<LISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TCIF3` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub struct TCIF3_R(crate::FieldReader<bool, bool>);
impl TCIF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF3` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub struct HTIF3_R(crate::FieldReader<bool, bool>);
impl HTIF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HTIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF3` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub struct TEIF3_R(crate::FieldReader<bool, bool>);
impl TEIF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMEIF3` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub struct DMEIF3_R(crate::FieldReader<bool, bool>);
impl DMEIF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMEIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMEIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIF3` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub struct FEIF3_R(crate::FieldReader<bool, bool>);
impl FEIF3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEIF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF2` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub struct TCIF2_R(crate::FieldReader<bool, bool>);
impl TCIF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF2` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub struct HTIF2_R(crate::FieldReader<bool, bool>);
impl HTIF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HTIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF2` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub struct TEIF2_R(crate::FieldReader<bool, bool>);
impl TEIF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMEIF2` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub struct DMEIF2_R(crate::FieldReader<bool, bool>);
impl DMEIF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMEIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMEIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIF2` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub struct FEIF2_R(crate::FieldReader<bool, bool>);
impl FEIF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEIF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF1` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub struct TCIF1_R(crate::FieldReader<bool, bool>);
impl TCIF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF1` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub struct HTIF1_R(crate::FieldReader<bool, bool>);
impl HTIF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HTIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF1` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub struct TEIF1_R(crate::FieldReader<bool, bool>);
impl TEIF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMEIF1` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub struct DMEIF1_R(crate::FieldReader<bool, bool>);
impl DMEIF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMEIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMEIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIF1` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub struct FEIF1_R(crate::FieldReader<bool, bool>);
impl FEIF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEIF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCIF0` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub struct TCIF0_R(crate::FieldReader<bool, bool>);
impl TCIF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTIF0` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub struct HTIF0_R(crate::FieldReader<bool, bool>);
impl HTIF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HTIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEIF0` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub struct TEIF0_R(crate::FieldReader<bool, bool>);
impl TEIF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMEIF0` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub struct DMEIF0_R(crate::FieldReader<bool, bool>);
impl DMEIF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMEIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMEIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIF0` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub struct FEIF0_R(crate::FieldReader<bool, bool>);
impl FEIF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEIF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif3(&self) -> DMEIF3_R {
        DMEIF3_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif3(&self) -> FEIF3_R {
        FEIF3_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif2(&self) -> DMEIF2_R {
        DMEIF2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif2(&self) -> FEIF2_R {
        FEIF2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif1(&self) -> DMEIF1_R {
        DMEIF1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif1(&self) -> FEIF1_R {
        FEIF1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif0(&self) -> HTIF0_R {
        HTIF0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif0(&self) -> DMEIF0_R {
        DMEIF0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif0(&self) -> FEIF0_R {
        FEIF0_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "low interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lisr](index.html) module"]
pub struct LISR_SPEC;
impl crate::RegisterSpec for LISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lisr::R](R) reader structure"]
impl crate::Readable for LISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LISR to value 0"]
impl crate::Resettable for LISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
