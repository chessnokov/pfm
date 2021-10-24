#[doc = "Register `K2LR` writer"]
pub struct W(crate::W<K2LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<K2LR_SPEC>;
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
impl From<crate::W<K2LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<K2LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `b96` writer - b96"]
pub struct B96_W<'a> {
    w: &'a mut W,
}
impl<'a> B96_W<'a> {
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
#[doc = "Field `b97` writer - b97"]
pub struct B97_W<'a> {
    w: &'a mut W,
}
impl<'a> B97_W<'a> {
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
#[doc = "Field `b98` writer - b98"]
pub struct B98_W<'a> {
    w: &'a mut W,
}
impl<'a> B98_W<'a> {
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
#[doc = "Field `b99` writer - b99"]
pub struct B99_W<'a> {
    w: &'a mut W,
}
impl<'a> B99_W<'a> {
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
#[doc = "Field `b100` writer - b100"]
pub struct B100_W<'a> {
    w: &'a mut W,
}
impl<'a> B100_W<'a> {
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
#[doc = "Field `b101` writer - b101"]
pub struct B101_W<'a> {
    w: &'a mut W,
}
impl<'a> B101_W<'a> {
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
#[doc = "Field `b102` writer - b102"]
pub struct B102_W<'a> {
    w: &'a mut W,
}
impl<'a> B102_W<'a> {
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
#[doc = "Field `b103` writer - b103"]
pub struct B103_W<'a> {
    w: &'a mut W,
}
impl<'a> B103_W<'a> {
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
#[doc = "Field `b104` writer - b104"]
pub struct B104_W<'a> {
    w: &'a mut W,
}
impl<'a> B104_W<'a> {
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
#[doc = "Field `b105` writer - b105"]
pub struct B105_W<'a> {
    w: &'a mut W,
}
impl<'a> B105_W<'a> {
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
#[doc = "Field `b106` writer - b106"]
pub struct B106_W<'a> {
    w: &'a mut W,
}
impl<'a> B106_W<'a> {
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
#[doc = "Field `b107` writer - b107"]
pub struct B107_W<'a> {
    w: &'a mut W,
}
impl<'a> B107_W<'a> {
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
#[doc = "Field `b108` writer - b108"]
pub struct B108_W<'a> {
    w: &'a mut W,
}
impl<'a> B108_W<'a> {
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
#[doc = "Field `b109` writer - b109"]
pub struct B109_W<'a> {
    w: &'a mut W,
}
impl<'a> B109_W<'a> {
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
#[doc = "Field `b110` writer - b110"]
pub struct B110_W<'a> {
    w: &'a mut W,
}
impl<'a> B110_W<'a> {
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
#[doc = "Field `b111` writer - b111"]
pub struct B111_W<'a> {
    w: &'a mut W,
}
impl<'a> B111_W<'a> {
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
#[doc = "Field `b112` writer - b112"]
pub struct B112_W<'a> {
    w: &'a mut W,
}
impl<'a> B112_W<'a> {
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
#[doc = "Field `b113` writer - b113"]
pub struct B113_W<'a> {
    w: &'a mut W,
}
impl<'a> B113_W<'a> {
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
#[doc = "Field `b114` writer - b114"]
pub struct B114_W<'a> {
    w: &'a mut W,
}
impl<'a> B114_W<'a> {
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
#[doc = "Field `b115` writer - b115"]
pub struct B115_W<'a> {
    w: &'a mut W,
}
impl<'a> B115_W<'a> {
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
#[doc = "Field `b116` writer - b116"]
pub struct B116_W<'a> {
    w: &'a mut W,
}
impl<'a> B116_W<'a> {
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
#[doc = "Field `b117` writer - b117"]
pub struct B117_W<'a> {
    w: &'a mut W,
}
impl<'a> B117_W<'a> {
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
#[doc = "Field `b118` writer - b118"]
pub struct B118_W<'a> {
    w: &'a mut W,
}
impl<'a> B118_W<'a> {
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
#[doc = "Field `b119` writer - b119"]
pub struct B119_W<'a> {
    w: &'a mut W,
}
impl<'a> B119_W<'a> {
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
#[doc = "Field `b120` writer - b120"]
pub struct B120_W<'a> {
    w: &'a mut W,
}
impl<'a> B120_W<'a> {
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
#[doc = "Field `b121` writer - b121"]
pub struct B121_W<'a> {
    w: &'a mut W,
}
impl<'a> B121_W<'a> {
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
#[doc = "Field `b122` writer - b122"]
pub struct B122_W<'a> {
    w: &'a mut W,
}
impl<'a> B122_W<'a> {
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
#[doc = "Field `b123` writer - b123"]
pub struct B123_W<'a> {
    w: &'a mut W,
}
impl<'a> B123_W<'a> {
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
#[doc = "Field `b124` writer - b124"]
pub struct B124_W<'a> {
    w: &'a mut W,
}
impl<'a> B124_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `b125` writer - b125"]
pub struct B125_W<'a> {
    w: &'a mut W,
}
impl<'a> B125_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `b126` writer - b126"]
pub struct B126_W<'a> {
    w: &'a mut W,
}
impl<'a> B126_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `b127` writer - b127"]
pub struct B127_W<'a> {
    w: &'a mut W,
}
impl<'a> B127_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - b96"]
    #[inline(always)]
    pub fn b96(&mut self) -> B96_W {
        B96_W { w: self }
    }
    #[doc = "Bit 1 - b97"]
    #[inline(always)]
    pub fn b97(&mut self) -> B97_W {
        B97_W { w: self }
    }
    #[doc = "Bit 2 - b98"]
    #[inline(always)]
    pub fn b98(&mut self) -> B98_W {
        B98_W { w: self }
    }
    #[doc = "Bit 3 - b99"]
    #[inline(always)]
    pub fn b99(&mut self) -> B99_W {
        B99_W { w: self }
    }
    #[doc = "Bit 4 - b100"]
    #[inline(always)]
    pub fn b100(&mut self) -> B100_W {
        B100_W { w: self }
    }
    #[doc = "Bit 5 - b101"]
    #[inline(always)]
    pub fn b101(&mut self) -> B101_W {
        B101_W { w: self }
    }
    #[doc = "Bit 6 - b102"]
    #[inline(always)]
    pub fn b102(&mut self) -> B102_W {
        B102_W { w: self }
    }
    #[doc = "Bit 7 - b103"]
    #[inline(always)]
    pub fn b103(&mut self) -> B103_W {
        B103_W { w: self }
    }
    #[doc = "Bit 8 - b104"]
    #[inline(always)]
    pub fn b104(&mut self) -> B104_W {
        B104_W { w: self }
    }
    #[doc = "Bit 9 - b105"]
    #[inline(always)]
    pub fn b105(&mut self) -> B105_W {
        B105_W { w: self }
    }
    #[doc = "Bit 10 - b106"]
    #[inline(always)]
    pub fn b106(&mut self) -> B106_W {
        B106_W { w: self }
    }
    #[doc = "Bit 11 - b107"]
    #[inline(always)]
    pub fn b107(&mut self) -> B107_W {
        B107_W { w: self }
    }
    #[doc = "Bit 12 - b108"]
    #[inline(always)]
    pub fn b108(&mut self) -> B108_W {
        B108_W { w: self }
    }
    #[doc = "Bit 13 - b109"]
    #[inline(always)]
    pub fn b109(&mut self) -> B109_W {
        B109_W { w: self }
    }
    #[doc = "Bit 14 - b110"]
    #[inline(always)]
    pub fn b110(&mut self) -> B110_W {
        B110_W { w: self }
    }
    #[doc = "Bit 15 - b111"]
    #[inline(always)]
    pub fn b111(&mut self) -> B111_W {
        B111_W { w: self }
    }
    #[doc = "Bit 16 - b112"]
    #[inline(always)]
    pub fn b112(&mut self) -> B112_W {
        B112_W { w: self }
    }
    #[doc = "Bit 17 - b113"]
    #[inline(always)]
    pub fn b113(&mut self) -> B113_W {
        B113_W { w: self }
    }
    #[doc = "Bit 18 - b114"]
    #[inline(always)]
    pub fn b114(&mut self) -> B114_W {
        B114_W { w: self }
    }
    #[doc = "Bit 19 - b115"]
    #[inline(always)]
    pub fn b115(&mut self) -> B115_W {
        B115_W { w: self }
    }
    #[doc = "Bit 20 - b116"]
    #[inline(always)]
    pub fn b116(&mut self) -> B116_W {
        B116_W { w: self }
    }
    #[doc = "Bit 21 - b117"]
    #[inline(always)]
    pub fn b117(&mut self) -> B117_W {
        B117_W { w: self }
    }
    #[doc = "Bit 22 - b118"]
    #[inline(always)]
    pub fn b118(&mut self) -> B118_W {
        B118_W { w: self }
    }
    #[doc = "Bit 23 - b119"]
    #[inline(always)]
    pub fn b119(&mut self) -> B119_W {
        B119_W { w: self }
    }
    #[doc = "Bit 24 - b120"]
    #[inline(always)]
    pub fn b120(&mut self) -> B120_W {
        B120_W { w: self }
    }
    #[doc = "Bit 25 - b121"]
    #[inline(always)]
    pub fn b121(&mut self) -> B121_W {
        B121_W { w: self }
    }
    #[doc = "Bit 26 - b122"]
    #[inline(always)]
    pub fn b122(&mut self) -> B122_W {
        B122_W { w: self }
    }
    #[doc = "Bit 27 - b123"]
    #[inline(always)]
    pub fn b123(&mut self) -> B123_W {
        B123_W { w: self }
    }
    #[doc = "Bit 28 - b124"]
    #[inline(always)]
    pub fn b124(&mut self) -> B124_W {
        B124_W { w: self }
    }
    #[doc = "Bit 29 - b125"]
    #[inline(always)]
    pub fn b125(&mut self) -> B125_W {
        B125_W { w: self }
    }
    #[doc = "Bit 30 - b126"]
    #[inline(always)]
    pub fn b126(&mut self) -> B126_W {
        B126_W { w: self }
    }
    #[doc = "Bit 31 - b127"]
    #[inline(always)]
    pub fn b127(&mut self) -> B127_W {
        B127_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k2lr](index.html) module"]
pub struct K2LR_SPEC;
impl crate::RegisterSpec for K2LR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [k2lr::W](W) writer structure"]
impl crate::Writable for K2LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets K2LR to value 0"]
impl crate::Resettable for K2LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
