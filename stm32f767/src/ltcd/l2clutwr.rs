#[doc = "Register `L2CLUTWR` writer"]
pub struct W(crate::W<L2CLUTWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2CLUTWR_SPEC>;
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
impl From<crate::W<L2CLUTWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2CLUTWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLUTADD` writer - CLUT Address"]
pub struct CLUTADD_W<'a> {
    w: &'a mut W,
}
impl<'a> CLUTADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `RED` writer - Red value"]
pub struct RED_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `GREEN` writer - Green value"]
pub struct GREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BLUE` writer - Blue value"]
pub struct BLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 24:31 - CLUT Address"]
    #[inline(always)]
    pub fn clutadd(&mut self) -> CLUTADD_W {
        CLUTADD_W { w: self }
    }
    #[doc = "Bits 16:23 - Red value"]
    #[inline(always)]
    pub fn red(&mut self) -> RED_W {
        RED_W { w: self }
    }
    #[doc = "Bits 8:15 - Green value"]
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W {
        GREEN_W { w: self }
    }
    #[doc = "Bits 0:7 - Blue value"]
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W {
        BLUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layerx CLUT Write Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2clutwr](index.html) module"]
pub struct L2CLUTWR_SPEC;
impl crate::RegisterSpec for L2CLUTWR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [l2clutwr::W](W) writer structure"]
impl crate::Writable for L2CLUTWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L2CLUTWR to value 0"]
impl crate::Resettable for L2CLUTWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
