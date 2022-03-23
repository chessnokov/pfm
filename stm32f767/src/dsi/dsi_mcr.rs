#[doc = "Register `DSI_MCR` reader"]
pub struct R(crate::R<DSI_MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSI_MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSI_MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSI_MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSI_MCR` writer"]
pub struct W(crate::W<DSI_MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSI_MCR_SPEC>;
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
impl From<crate::W<DSI_MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSI_MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDM` reader - Command mode"]
pub struct CMDM_R(crate::FieldReader<bool, bool>);
impl CMDM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDM` writer - Command mode"]
pub struct CMDM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDM_W<'a> {
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
    #[doc = "Bit 0 - Command mode"]
    #[inline(always)]
    pub fn cmdm(&self) -> CMDM_R {
        CMDM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command mode"]
    #[inline(always)]
    pub fn cmdm(&mut self) -> CMDM_W {
        CMDM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host mode Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsi_mcr](index.html) module"]
pub struct DSI_MCR_SPEC;
impl crate::RegisterSpec for DSI_MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsi_mcr::R](R) reader structure"]
impl crate::Readable for DSI_MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsi_mcr::W](W) writer structure"]
impl crate::Writable for DSI_MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSI_MCR to value 0"]
impl crate::Resettable for DSI_MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
