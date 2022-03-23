#[doc = "Register `MDIOS_DOUTR5` reader"]
pub struct R(crate::R<MDIOS_DOUTR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_DOUTR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_DOUTR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_DOUTR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDIOS_DOUTR5` writer"]
pub struct W(crate::W<MDIOS_DOUTR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIOS_DOUTR5_SPEC>;
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
impl From<crate::W<MDIOS_DOUTR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIOS_DOUTR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT5` reader - Output data sent to MDIO Master during read frames"]
pub struct DOUT5_R(crate::FieldReader<u16, u16>);
impl DOUT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DOUT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUT5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUT5` writer - Output data sent to MDIO Master during read frames"]
pub struct DOUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUT5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout5(&self) -> DOUT5_R {
        DOUT5_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout5(&mut self) -> DOUT5_W {
        DOUT5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS output data register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdios_doutr5](index.html) module"]
pub struct MDIOS_DOUTR5_SPEC;
impl crate::RegisterSpec for MDIOS_DOUTR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdios_doutr5::R](R) reader structure"]
impl crate::Readable for MDIOS_DOUTR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdios_doutr5::W](W) writer structure"]
impl crate::Writable for MDIOS_DOUTR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDIOS_DOUTR5 to value 0"]
impl crate::Resettable for MDIOS_DOUTR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
