#[doc = "Register `K1LR` writer"]
pub struct W(crate::W<K1LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<K1LR_SPEC>;
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
impl From<crate::W<K1LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<K1LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `b160` writer - b160"]
pub struct B160_W<'a> {
    w: &'a mut W,
}
impl<'a> B160_W<'a> {
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
#[doc = "Field `b161` writer - b161"]
pub struct B161_W<'a> {
    w: &'a mut W,
}
impl<'a> B161_W<'a> {
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
#[doc = "Field `b162` writer - b162"]
pub struct B162_W<'a> {
    w: &'a mut W,
}
impl<'a> B162_W<'a> {
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
#[doc = "Field `b163` writer - b163"]
pub struct B163_W<'a> {
    w: &'a mut W,
}
impl<'a> B163_W<'a> {
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
#[doc = "Field `b164` writer - b164"]
pub struct B164_W<'a> {
    w: &'a mut W,
}
impl<'a> B164_W<'a> {
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
#[doc = "Field `b165` writer - b165"]
pub struct B165_W<'a> {
    w: &'a mut W,
}
impl<'a> B165_W<'a> {
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
#[doc = "Field `b166` writer - b166"]
pub struct B166_W<'a> {
    w: &'a mut W,
}
impl<'a> B166_W<'a> {
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
#[doc = "Field `b167` writer - b167"]
pub struct B167_W<'a> {
    w: &'a mut W,
}
impl<'a> B167_W<'a> {
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
#[doc = "Field `b168` writer - b168"]
pub struct B168_W<'a> {
    w: &'a mut W,
}
impl<'a> B168_W<'a> {
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
#[doc = "Field `b169` writer - b169"]
pub struct B169_W<'a> {
    w: &'a mut W,
}
impl<'a> B169_W<'a> {
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
#[doc = "Field `b170` writer - b170"]
pub struct B170_W<'a> {
    w: &'a mut W,
}
impl<'a> B170_W<'a> {
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
#[doc = "Field `b171` writer - b171"]
pub struct B171_W<'a> {
    w: &'a mut W,
}
impl<'a> B171_W<'a> {
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
#[doc = "Field `b172` writer - b172"]
pub struct B172_W<'a> {
    w: &'a mut W,
}
impl<'a> B172_W<'a> {
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
#[doc = "Field `b173` writer - b173"]
pub struct B173_W<'a> {
    w: &'a mut W,
}
impl<'a> B173_W<'a> {
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
#[doc = "Field `b174` writer - b174"]
pub struct B174_W<'a> {
    w: &'a mut W,
}
impl<'a> B174_W<'a> {
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
#[doc = "Field `b175` writer - b175"]
pub struct B175_W<'a> {
    w: &'a mut W,
}
impl<'a> B175_W<'a> {
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
#[doc = "Field `b176` writer - b176"]
pub struct B176_W<'a> {
    w: &'a mut W,
}
impl<'a> B176_W<'a> {
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
#[doc = "Field `b177` writer - b177"]
pub struct B177_W<'a> {
    w: &'a mut W,
}
impl<'a> B177_W<'a> {
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
#[doc = "Field `b178` writer - b178"]
pub struct B178_W<'a> {
    w: &'a mut W,
}
impl<'a> B178_W<'a> {
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
#[doc = "Field `b179` writer - b179"]
pub struct B179_W<'a> {
    w: &'a mut W,
}
impl<'a> B179_W<'a> {
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
#[doc = "Field `b180` writer - b180"]
pub struct B180_W<'a> {
    w: &'a mut W,
}
impl<'a> B180_W<'a> {
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
#[doc = "Field `b181` writer - b181"]
pub struct B181_W<'a> {
    w: &'a mut W,
}
impl<'a> B181_W<'a> {
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
#[doc = "Field `b182` writer - b182"]
pub struct B182_W<'a> {
    w: &'a mut W,
}
impl<'a> B182_W<'a> {
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
#[doc = "Field `b183` writer - b183"]
pub struct B183_W<'a> {
    w: &'a mut W,
}
impl<'a> B183_W<'a> {
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
#[doc = "Field `b184` writer - b184"]
pub struct B184_W<'a> {
    w: &'a mut W,
}
impl<'a> B184_W<'a> {
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
#[doc = "Field `b185` writer - b185"]
pub struct B185_W<'a> {
    w: &'a mut W,
}
impl<'a> B185_W<'a> {
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
#[doc = "Field `b186` writer - b186"]
pub struct B186_W<'a> {
    w: &'a mut W,
}
impl<'a> B186_W<'a> {
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
#[doc = "Field `b187` writer - b187"]
pub struct B187_W<'a> {
    w: &'a mut W,
}
impl<'a> B187_W<'a> {
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
#[doc = "Field `b188` writer - b188"]
pub struct B188_W<'a> {
    w: &'a mut W,
}
impl<'a> B188_W<'a> {
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
#[doc = "Field `b189` writer - b189"]
pub struct B189_W<'a> {
    w: &'a mut W,
}
impl<'a> B189_W<'a> {
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
#[doc = "Field `b190` writer - b190"]
pub struct B190_W<'a> {
    w: &'a mut W,
}
impl<'a> B190_W<'a> {
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
#[doc = "Field `b191` writer - b191"]
pub struct B191_W<'a> {
    w: &'a mut W,
}
impl<'a> B191_W<'a> {
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
    #[doc = "Bit 0 - b160"]
    #[inline(always)]
    pub fn b160(&mut self) -> B160_W {
        B160_W { w: self }
    }
    #[doc = "Bit 1 - b161"]
    #[inline(always)]
    pub fn b161(&mut self) -> B161_W {
        B161_W { w: self }
    }
    #[doc = "Bit 2 - b162"]
    #[inline(always)]
    pub fn b162(&mut self) -> B162_W {
        B162_W { w: self }
    }
    #[doc = "Bit 3 - b163"]
    #[inline(always)]
    pub fn b163(&mut self) -> B163_W {
        B163_W { w: self }
    }
    #[doc = "Bit 4 - b164"]
    #[inline(always)]
    pub fn b164(&mut self) -> B164_W {
        B164_W { w: self }
    }
    #[doc = "Bit 5 - b165"]
    #[inline(always)]
    pub fn b165(&mut self) -> B165_W {
        B165_W { w: self }
    }
    #[doc = "Bit 6 - b166"]
    #[inline(always)]
    pub fn b166(&mut self) -> B166_W {
        B166_W { w: self }
    }
    #[doc = "Bit 7 - b167"]
    #[inline(always)]
    pub fn b167(&mut self) -> B167_W {
        B167_W { w: self }
    }
    #[doc = "Bit 8 - b168"]
    #[inline(always)]
    pub fn b168(&mut self) -> B168_W {
        B168_W { w: self }
    }
    #[doc = "Bit 9 - b169"]
    #[inline(always)]
    pub fn b169(&mut self) -> B169_W {
        B169_W { w: self }
    }
    #[doc = "Bit 10 - b170"]
    #[inline(always)]
    pub fn b170(&mut self) -> B170_W {
        B170_W { w: self }
    }
    #[doc = "Bit 11 - b171"]
    #[inline(always)]
    pub fn b171(&mut self) -> B171_W {
        B171_W { w: self }
    }
    #[doc = "Bit 12 - b172"]
    #[inline(always)]
    pub fn b172(&mut self) -> B172_W {
        B172_W { w: self }
    }
    #[doc = "Bit 13 - b173"]
    #[inline(always)]
    pub fn b173(&mut self) -> B173_W {
        B173_W { w: self }
    }
    #[doc = "Bit 14 - b174"]
    #[inline(always)]
    pub fn b174(&mut self) -> B174_W {
        B174_W { w: self }
    }
    #[doc = "Bit 15 - b175"]
    #[inline(always)]
    pub fn b175(&mut self) -> B175_W {
        B175_W { w: self }
    }
    #[doc = "Bit 16 - b176"]
    #[inline(always)]
    pub fn b176(&mut self) -> B176_W {
        B176_W { w: self }
    }
    #[doc = "Bit 17 - b177"]
    #[inline(always)]
    pub fn b177(&mut self) -> B177_W {
        B177_W { w: self }
    }
    #[doc = "Bit 18 - b178"]
    #[inline(always)]
    pub fn b178(&mut self) -> B178_W {
        B178_W { w: self }
    }
    #[doc = "Bit 19 - b179"]
    #[inline(always)]
    pub fn b179(&mut self) -> B179_W {
        B179_W { w: self }
    }
    #[doc = "Bit 20 - b180"]
    #[inline(always)]
    pub fn b180(&mut self) -> B180_W {
        B180_W { w: self }
    }
    #[doc = "Bit 21 - b181"]
    #[inline(always)]
    pub fn b181(&mut self) -> B181_W {
        B181_W { w: self }
    }
    #[doc = "Bit 22 - b182"]
    #[inline(always)]
    pub fn b182(&mut self) -> B182_W {
        B182_W { w: self }
    }
    #[doc = "Bit 23 - b183"]
    #[inline(always)]
    pub fn b183(&mut self) -> B183_W {
        B183_W { w: self }
    }
    #[doc = "Bit 24 - b184"]
    #[inline(always)]
    pub fn b184(&mut self) -> B184_W {
        B184_W { w: self }
    }
    #[doc = "Bit 25 - b185"]
    #[inline(always)]
    pub fn b185(&mut self) -> B185_W {
        B185_W { w: self }
    }
    #[doc = "Bit 26 - b186"]
    #[inline(always)]
    pub fn b186(&mut self) -> B186_W {
        B186_W { w: self }
    }
    #[doc = "Bit 27 - b187"]
    #[inline(always)]
    pub fn b187(&mut self) -> B187_W {
        B187_W { w: self }
    }
    #[doc = "Bit 28 - b188"]
    #[inline(always)]
    pub fn b188(&mut self) -> B188_W {
        B188_W { w: self }
    }
    #[doc = "Bit 29 - b189"]
    #[inline(always)]
    pub fn b189(&mut self) -> B189_W {
        B189_W { w: self }
    }
    #[doc = "Bit 30 - b190"]
    #[inline(always)]
    pub fn b190(&mut self) -> B190_W {
        B190_W { w: self }
    }
    #[doc = "Bit 31 - b191"]
    #[inline(always)]
    pub fn b191(&mut self) -> B191_W {
        B191_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k1lr](index.html) module"]
pub struct K1LR_SPEC;
impl crate::RegisterSpec for K1LR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [k1lr::W](W) writer structure"]
impl crate::Writable for K1LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets K1LR to value 0"]
impl crate::Resettable for K1LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
