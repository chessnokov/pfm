#[doc = "Register `OTG_FS_PCGCCTL` reader"]
pub struct R(crate::R<OTG_FS_PCGCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_PCGCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_PCGCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_PCGCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_PCGCCTL` writer"]
pub struct W(crate::W<OTG_FS_PCGCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_PCGCCTL_SPEC>;
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
impl From<crate::W<OTG_FS_PCGCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_PCGCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STPPCLK` reader - Stop PHY clock"]
pub struct STPPCLK_R(crate::FieldReader<bool, bool>);
impl STPPCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STPPCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STPPCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPPCLK` writer - Stop PHY clock"]
pub struct STPPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> STPPCLK_W<'a> {
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
#[doc = "Field `GATEHCLK` reader - Gate HCLK"]
pub struct GATEHCLK_R(crate::FieldReader<bool, bool>);
impl GATEHCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GATEHCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GATEHCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GATEHCLK` writer - Gate HCLK"]
pub struct GATEHCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> GATEHCLK_W<'a> {
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
#[doc = "Field `PHYSUSP` reader - PHY Suspended"]
pub struct PHYSUSP_R(crate::FieldReader<bool, bool>);
impl PHYSUSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PHYSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PHYSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHYSUSP` writer - PHY Suspended"]
pub struct PHYSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYSUSP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&mut self) -> STPPCLK_W {
        STPPCLK_W { w: self }
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W {
        GATEHCLK_W { w: self }
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    pub fn physusp(&mut self) -> PHYSUSP_W {
        PHYSUSP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_pcgcctl](index.html) module"]
pub struct OTG_FS_PCGCCTL_SPEC;
impl crate::RegisterSpec for OTG_FS_PCGCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_pcgcctl::R](R) reader structure"]
impl crate::Readable for OTG_FS_PCGCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_pcgcctl::W](W) writer structure"]
impl crate::Writable for OTG_FS_PCGCCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_PCGCCTL to value 0"]
impl crate::Resettable for OTG_FS_PCGCCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
