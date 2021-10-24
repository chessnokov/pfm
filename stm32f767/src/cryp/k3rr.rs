#[doc = "Register `K3RR` writer"]
pub struct W(crate::W<K3RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<K3RR_SPEC>;
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
impl From<crate::W<K3RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<K3RR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `b0` writer - b0"]
pub struct B0_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_W<'a> {
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
#[doc = "Field `b1` writer - b1"]
pub struct B1_W<'a> {
    w: &'a mut W,
}
impl<'a> B1_W<'a> {
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
#[doc = "Field `b2` writer - b2"]
pub struct B2_W<'a> {
    w: &'a mut W,
}
impl<'a> B2_W<'a> {
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
#[doc = "Field `b3` writer - b3"]
pub struct B3_W<'a> {
    w: &'a mut W,
}
impl<'a> B3_W<'a> {
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
#[doc = "Field `b4` writer - b4"]
pub struct B4_W<'a> {
    w: &'a mut W,
}
impl<'a> B4_W<'a> {
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
#[doc = "Field `b5` writer - b5"]
pub struct B5_W<'a> {
    w: &'a mut W,
}
impl<'a> B5_W<'a> {
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
#[doc = "Field `b6` writer - b6"]
pub struct B6_W<'a> {
    w: &'a mut W,
}
impl<'a> B6_W<'a> {
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
#[doc = "Field `b7` writer - b7"]
pub struct B7_W<'a> {
    w: &'a mut W,
}
impl<'a> B7_W<'a> {
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
#[doc = "Field `b8` writer - b8"]
pub struct B8_W<'a> {
    w: &'a mut W,
}
impl<'a> B8_W<'a> {
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
#[doc = "Field `b9` writer - b9"]
pub struct B9_W<'a> {
    w: &'a mut W,
}
impl<'a> B9_W<'a> {
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
#[doc = "Field `b10` writer - b10"]
pub struct B10_W<'a> {
    w: &'a mut W,
}
impl<'a> B10_W<'a> {
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
#[doc = "Field `b11` writer - b11"]
pub struct B11_W<'a> {
    w: &'a mut W,
}
impl<'a> B11_W<'a> {
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
#[doc = "Field `b12` writer - b12"]
pub struct B12_W<'a> {
    w: &'a mut W,
}
impl<'a> B12_W<'a> {
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
#[doc = "Field `b13` writer - b13"]
pub struct B13_W<'a> {
    w: &'a mut W,
}
impl<'a> B13_W<'a> {
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
#[doc = "Field `b14` writer - b14"]
pub struct B14_W<'a> {
    w: &'a mut W,
}
impl<'a> B14_W<'a> {
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
#[doc = "Field `b15` writer - b15"]
pub struct B15_W<'a> {
    w: &'a mut W,
}
impl<'a> B15_W<'a> {
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
#[doc = "Field `b16` writer - b16"]
pub struct B16_W<'a> {
    w: &'a mut W,
}
impl<'a> B16_W<'a> {
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
#[doc = "Field `b17` writer - b17"]
pub struct B17_W<'a> {
    w: &'a mut W,
}
impl<'a> B17_W<'a> {
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
#[doc = "Field `b18` writer - b18"]
pub struct B18_W<'a> {
    w: &'a mut W,
}
impl<'a> B18_W<'a> {
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
#[doc = "Field `b19` writer - b19"]
pub struct B19_W<'a> {
    w: &'a mut W,
}
impl<'a> B19_W<'a> {
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
#[doc = "Field `b20` writer - b20"]
pub struct B20_W<'a> {
    w: &'a mut W,
}
impl<'a> B20_W<'a> {
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
#[doc = "Field `b21` writer - b21"]
pub struct B21_W<'a> {
    w: &'a mut W,
}
impl<'a> B21_W<'a> {
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
#[doc = "Field `b22` writer - b22"]
pub struct B22_W<'a> {
    w: &'a mut W,
}
impl<'a> B22_W<'a> {
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
#[doc = "Field `b23` writer - b23"]
pub struct B23_W<'a> {
    w: &'a mut W,
}
impl<'a> B23_W<'a> {
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
#[doc = "Field `b24` writer - b24"]
pub struct B24_W<'a> {
    w: &'a mut W,
}
impl<'a> B24_W<'a> {
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
#[doc = "Field `b25` writer - b25"]
pub struct B25_W<'a> {
    w: &'a mut W,
}
impl<'a> B25_W<'a> {
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
#[doc = "Field `b26` writer - b26"]
pub struct B26_W<'a> {
    w: &'a mut W,
}
impl<'a> B26_W<'a> {
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
#[doc = "Field `b27` writer - b27"]
pub struct B27_W<'a> {
    w: &'a mut W,
}
impl<'a> B27_W<'a> {
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
#[doc = "Field `b28` writer - b28"]
pub struct B28_W<'a> {
    w: &'a mut W,
}
impl<'a> B28_W<'a> {
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
#[doc = "Field `b29` writer - b29"]
pub struct B29_W<'a> {
    w: &'a mut W,
}
impl<'a> B29_W<'a> {
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
#[doc = "Field `b30` writer - b30"]
pub struct B30_W<'a> {
    w: &'a mut W,
}
impl<'a> B30_W<'a> {
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
#[doc = "Field `b31` writer - b31"]
pub struct B31_W<'a> {
    w: &'a mut W,
}
impl<'a> B31_W<'a> {
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
    #[doc = "Bit 0 - b0"]
    #[inline(always)]
    pub fn b0(&mut self) -> B0_W {
        B0_W { w: self }
    }
    #[doc = "Bit 1 - b1"]
    #[inline(always)]
    pub fn b1(&mut self) -> B1_W {
        B1_W { w: self }
    }
    #[doc = "Bit 2 - b2"]
    #[inline(always)]
    pub fn b2(&mut self) -> B2_W {
        B2_W { w: self }
    }
    #[doc = "Bit 3 - b3"]
    #[inline(always)]
    pub fn b3(&mut self) -> B3_W {
        B3_W { w: self }
    }
    #[doc = "Bit 4 - b4"]
    #[inline(always)]
    pub fn b4(&mut self) -> B4_W {
        B4_W { w: self }
    }
    #[doc = "Bit 5 - b5"]
    #[inline(always)]
    pub fn b5(&mut self) -> B5_W {
        B5_W { w: self }
    }
    #[doc = "Bit 6 - b6"]
    #[inline(always)]
    pub fn b6(&mut self) -> B6_W {
        B6_W { w: self }
    }
    #[doc = "Bit 7 - b7"]
    #[inline(always)]
    pub fn b7(&mut self) -> B7_W {
        B7_W { w: self }
    }
    #[doc = "Bit 8 - b8"]
    #[inline(always)]
    pub fn b8(&mut self) -> B8_W {
        B8_W { w: self }
    }
    #[doc = "Bit 9 - b9"]
    #[inline(always)]
    pub fn b9(&mut self) -> B9_W {
        B9_W { w: self }
    }
    #[doc = "Bit 10 - b10"]
    #[inline(always)]
    pub fn b10(&mut self) -> B10_W {
        B10_W { w: self }
    }
    #[doc = "Bit 11 - b11"]
    #[inline(always)]
    pub fn b11(&mut self) -> B11_W {
        B11_W { w: self }
    }
    #[doc = "Bit 12 - b12"]
    #[inline(always)]
    pub fn b12(&mut self) -> B12_W {
        B12_W { w: self }
    }
    #[doc = "Bit 13 - b13"]
    #[inline(always)]
    pub fn b13(&mut self) -> B13_W {
        B13_W { w: self }
    }
    #[doc = "Bit 14 - b14"]
    #[inline(always)]
    pub fn b14(&mut self) -> B14_W {
        B14_W { w: self }
    }
    #[doc = "Bit 15 - b15"]
    #[inline(always)]
    pub fn b15(&mut self) -> B15_W {
        B15_W { w: self }
    }
    #[doc = "Bit 16 - b16"]
    #[inline(always)]
    pub fn b16(&mut self) -> B16_W {
        B16_W { w: self }
    }
    #[doc = "Bit 17 - b17"]
    #[inline(always)]
    pub fn b17(&mut self) -> B17_W {
        B17_W { w: self }
    }
    #[doc = "Bit 18 - b18"]
    #[inline(always)]
    pub fn b18(&mut self) -> B18_W {
        B18_W { w: self }
    }
    #[doc = "Bit 19 - b19"]
    #[inline(always)]
    pub fn b19(&mut self) -> B19_W {
        B19_W { w: self }
    }
    #[doc = "Bit 20 - b20"]
    #[inline(always)]
    pub fn b20(&mut self) -> B20_W {
        B20_W { w: self }
    }
    #[doc = "Bit 21 - b21"]
    #[inline(always)]
    pub fn b21(&mut self) -> B21_W {
        B21_W { w: self }
    }
    #[doc = "Bit 22 - b22"]
    #[inline(always)]
    pub fn b22(&mut self) -> B22_W {
        B22_W { w: self }
    }
    #[doc = "Bit 23 - b23"]
    #[inline(always)]
    pub fn b23(&mut self) -> B23_W {
        B23_W { w: self }
    }
    #[doc = "Bit 24 - b24"]
    #[inline(always)]
    pub fn b24(&mut self) -> B24_W {
        B24_W { w: self }
    }
    #[doc = "Bit 25 - b25"]
    #[inline(always)]
    pub fn b25(&mut self) -> B25_W {
        B25_W { w: self }
    }
    #[doc = "Bit 26 - b26"]
    #[inline(always)]
    pub fn b26(&mut self) -> B26_W {
        B26_W { w: self }
    }
    #[doc = "Bit 27 - b27"]
    #[inline(always)]
    pub fn b27(&mut self) -> B27_W {
        B27_W { w: self }
    }
    #[doc = "Bit 28 - b28"]
    #[inline(always)]
    pub fn b28(&mut self) -> B28_W {
        B28_W { w: self }
    }
    #[doc = "Bit 29 - b29"]
    #[inline(always)]
    pub fn b29(&mut self) -> B29_W {
        B29_W { w: self }
    }
    #[doc = "Bit 30 - b30"]
    #[inline(always)]
    pub fn b30(&mut self) -> B30_W {
        B30_W { w: self }
    }
    #[doc = "Bit 31 - b31"]
    #[inline(always)]
    pub fn b31(&mut self) -> B31_W {
        B31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k3rr](index.html) module"]
pub struct K3RR_SPEC;
impl crate::RegisterSpec for K3RR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [k3rr::W](W) writer structure"]
impl crate::Writable for K3RR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets K3RR to value 0"]
impl crate::Resettable for K3RR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
