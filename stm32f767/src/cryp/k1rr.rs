#[doc = "Register `K1RR` writer"]
pub struct W(crate::W<K1RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<K1RR_SPEC>;
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
impl From<crate::W<K1RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<K1RR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `b128` writer - b128"]
pub struct B128_W<'a> {
    w: &'a mut W,
}
impl<'a> B128_W<'a> {
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
#[doc = "Field `b129` writer - b129"]
pub struct B129_W<'a> {
    w: &'a mut W,
}
impl<'a> B129_W<'a> {
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
#[doc = "Field `b130` writer - b130"]
pub struct B130_W<'a> {
    w: &'a mut W,
}
impl<'a> B130_W<'a> {
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
#[doc = "Field `b131` writer - b131"]
pub struct B131_W<'a> {
    w: &'a mut W,
}
impl<'a> B131_W<'a> {
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
#[doc = "Field `b132` writer - b132"]
pub struct B132_W<'a> {
    w: &'a mut W,
}
impl<'a> B132_W<'a> {
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
#[doc = "Field `b133` writer - b133"]
pub struct B133_W<'a> {
    w: &'a mut W,
}
impl<'a> B133_W<'a> {
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
#[doc = "Field `b134` writer - b134"]
pub struct B134_W<'a> {
    w: &'a mut W,
}
impl<'a> B134_W<'a> {
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
#[doc = "Field `b135` writer - b135"]
pub struct B135_W<'a> {
    w: &'a mut W,
}
impl<'a> B135_W<'a> {
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
#[doc = "Field `b136` writer - b136"]
pub struct B136_W<'a> {
    w: &'a mut W,
}
impl<'a> B136_W<'a> {
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
#[doc = "Field `b137` writer - b137"]
pub struct B137_W<'a> {
    w: &'a mut W,
}
impl<'a> B137_W<'a> {
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
#[doc = "Field `b138` writer - b138"]
pub struct B138_W<'a> {
    w: &'a mut W,
}
impl<'a> B138_W<'a> {
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
#[doc = "Field `b139` writer - b139"]
pub struct B139_W<'a> {
    w: &'a mut W,
}
impl<'a> B139_W<'a> {
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
#[doc = "Field `b140` writer - b140"]
pub struct B140_W<'a> {
    w: &'a mut W,
}
impl<'a> B140_W<'a> {
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
#[doc = "Field `b141` writer - b141"]
pub struct B141_W<'a> {
    w: &'a mut W,
}
impl<'a> B141_W<'a> {
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
#[doc = "Field `b142` writer - b142"]
pub struct B142_W<'a> {
    w: &'a mut W,
}
impl<'a> B142_W<'a> {
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
#[doc = "Field `b143` writer - b143"]
pub struct B143_W<'a> {
    w: &'a mut W,
}
impl<'a> B143_W<'a> {
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
#[doc = "Field `b144` writer - b144"]
pub struct B144_W<'a> {
    w: &'a mut W,
}
impl<'a> B144_W<'a> {
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
#[doc = "Field `b145` writer - b145"]
pub struct B145_W<'a> {
    w: &'a mut W,
}
impl<'a> B145_W<'a> {
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
#[doc = "Field `b146` writer - b146"]
pub struct B146_W<'a> {
    w: &'a mut W,
}
impl<'a> B146_W<'a> {
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
#[doc = "Field `b147` writer - b147"]
pub struct B147_W<'a> {
    w: &'a mut W,
}
impl<'a> B147_W<'a> {
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
#[doc = "Field `b148` writer - b148"]
pub struct B148_W<'a> {
    w: &'a mut W,
}
impl<'a> B148_W<'a> {
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
#[doc = "Field `b149` writer - b149"]
pub struct B149_W<'a> {
    w: &'a mut W,
}
impl<'a> B149_W<'a> {
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
#[doc = "Field `b150` writer - b150"]
pub struct B150_W<'a> {
    w: &'a mut W,
}
impl<'a> B150_W<'a> {
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
#[doc = "Field `b151` writer - b151"]
pub struct B151_W<'a> {
    w: &'a mut W,
}
impl<'a> B151_W<'a> {
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
#[doc = "Field `b152` writer - b152"]
pub struct B152_W<'a> {
    w: &'a mut W,
}
impl<'a> B152_W<'a> {
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
#[doc = "Field `b153` writer - b153"]
pub struct B153_W<'a> {
    w: &'a mut W,
}
impl<'a> B153_W<'a> {
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
#[doc = "Field `b154` writer - b154"]
pub struct B154_W<'a> {
    w: &'a mut W,
}
impl<'a> B154_W<'a> {
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
#[doc = "Field `b155` writer - b155"]
pub struct B155_W<'a> {
    w: &'a mut W,
}
impl<'a> B155_W<'a> {
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
#[doc = "Field `b156` writer - b156"]
pub struct B156_W<'a> {
    w: &'a mut W,
}
impl<'a> B156_W<'a> {
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
#[doc = "Field `b157` writer - b157"]
pub struct B157_W<'a> {
    w: &'a mut W,
}
impl<'a> B157_W<'a> {
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
#[doc = "Field `b158` writer - b158"]
pub struct B158_W<'a> {
    w: &'a mut W,
}
impl<'a> B158_W<'a> {
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
#[doc = "Field `b159` writer - b159"]
pub struct B159_W<'a> {
    w: &'a mut W,
}
impl<'a> B159_W<'a> {
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
    #[doc = "Bit 0 - b128"]
    #[inline(always)]
    pub fn b128(&mut self) -> B128_W {
        B128_W { w: self }
    }
    #[doc = "Bit 1 - b129"]
    #[inline(always)]
    pub fn b129(&mut self) -> B129_W {
        B129_W { w: self }
    }
    #[doc = "Bit 2 - b130"]
    #[inline(always)]
    pub fn b130(&mut self) -> B130_W {
        B130_W { w: self }
    }
    #[doc = "Bit 3 - b131"]
    #[inline(always)]
    pub fn b131(&mut self) -> B131_W {
        B131_W { w: self }
    }
    #[doc = "Bit 4 - b132"]
    #[inline(always)]
    pub fn b132(&mut self) -> B132_W {
        B132_W { w: self }
    }
    #[doc = "Bit 5 - b133"]
    #[inline(always)]
    pub fn b133(&mut self) -> B133_W {
        B133_W { w: self }
    }
    #[doc = "Bit 6 - b134"]
    #[inline(always)]
    pub fn b134(&mut self) -> B134_W {
        B134_W { w: self }
    }
    #[doc = "Bit 7 - b135"]
    #[inline(always)]
    pub fn b135(&mut self) -> B135_W {
        B135_W { w: self }
    }
    #[doc = "Bit 8 - b136"]
    #[inline(always)]
    pub fn b136(&mut self) -> B136_W {
        B136_W { w: self }
    }
    #[doc = "Bit 9 - b137"]
    #[inline(always)]
    pub fn b137(&mut self) -> B137_W {
        B137_W { w: self }
    }
    #[doc = "Bit 10 - b138"]
    #[inline(always)]
    pub fn b138(&mut self) -> B138_W {
        B138_W { w: self }
    }
    #[doc = "Bit 11 - b139"]
    #[inline(always)]
    pub fn b139(&mut self) -> B139_W {
        B139_W { w: self }
    }
    #[doc = "Bit 12 - b140"]
    #[inline(always)]
    pub fn b140(&mut self) -> B140_W {
        B140_W { w: self }
    }
    #[doc = "Bit 13 - b141"]
    #[inline(always)]
    pub fn b141(&mut self) -> B141_W {
        B141_W { w: self }
    }
    #[doc = "Bit 14 - b142"]
    #[inline(always)]
    pub fn b142(&mut self) -> B142_W {
        B142_W { w: self }
    }
    #[doc = "Bit 15 - b143"]
    #[inline(always)]
    pub fn b143(&mut self) -> B143_W {
        B143_W { w: self }
    }
    #[doc = "Bit 16 - b144"]
    #[inline(always)]
    pub fn b144(&mut self) -> B144_W {
        B144_W { w: self }
    }
    #[doc = "Bit 17 - b145"]
    #[inline(always)]
    pub fn b145(&mut self) -> B145_W {
        B145_W { w: self }
    }
    #[doc = "Bit 18 - b146"]
    #[inline(always)]
    pub fn b146(&mut self) -> B146_W {
        B146_W { w: self }
    }
    #[doc = "Bit 19 - b147"]
    #[inline(always)]
    pub fn b147(&mut self) -> B147_W {
        B147_W { w: self }
    }
    #[doc = "Bit 20 - b148"]
    #[inline(always)]
    pub fn b148(&mut self) -> B148_W {
        B148_W { w: self }
    }
    #[doc = "Bit 21 - b149"]
    #[inline(always)]
    pub fn b149(&mut self) -> B149_W {
        B149_W { w: self }
    }
    #[doc = "Bit 22 - b150"]
    #[inline(always)]
    pub fn b150(&mut self) -> B150_W {
        B150_W { w: self }
    }
    #[doc = "Bit 23 - b151"]
    #[inline(always)]
    pub fn b151(&mut self) -> B151_W {
        B151_W { w: self }
    }
    #[doc = "Bit 24 - b152"]
    #[inline(always)]
    pub fn b152(&mut self) -> B152_W {
        B152_W { w: self }
    }
    #[doc = "Bit 25 - b153"]
    #[inline(always)]
    pub fn b153(&mut self) -> B153_W {
        B153_W { w: self }
    }
    #[doc = "Bit 26 - b154"]
    #[inline(always)]
    pub fn b154(&mut self) -> B154_W {
        B154_W { w: self }
    }
    #[doc = "Bit 27 - b155"]
    #[inline(always)]
    pub fn b155(&mut self) -> B155_W {
        B155_W { w: self }
    }
    #[doc = "Bit 28 - b156"]
    #[inline(always)]
    pub fn b156(&mut self) -> B156_W {
        B156_W { w: self }
    }
    #[doc = "Bit 29 - b157"]
    #[inline(always)]
    pub fn b157(&mut self) -> B157_W {
        B157_W { w: self }
    }
    #[doc = "Bit 30 - b158"]
    #[inline(always)]
    pub fn b158(&mut self) -> B158_W {
        B158_W { w: self }
    }
    #[doc = "Bit 31 - b159"]
    #[inline(always)]
    pub fn b159(&mut self) -> B159_W {
        B159_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k1rr](index.html) module"]
pub struct K1RR_SPEC;
impl crate::RegisterSpec for K1RR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [k1rr::W](W) writer structure"]
impl crate::Writable for K1RR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets K1RR to value 0"]
impl crate::Resettable for K1RR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
