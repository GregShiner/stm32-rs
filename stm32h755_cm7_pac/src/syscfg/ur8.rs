#[doc = "Register `UR8` reader"]
pub type R = crate::R<Ur8Spec>;
#[doc = "Field `MEPAD_2` reader - Mass erase protected area disabled for bank 2"]
pub type Mepad2R = crate::BitReader;
#[doc = "Field `MESAD_2` reader - Mass erase secured area disabled for bank 2"]
pub type Mesad2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mass erase protected area disabled for bank 2"]
    #[inline(always)]
    pub fn mepad_2(&self) -> Mepad2R {
        Mepad2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Mass erase secured area disabled for bank 2"]
    #[inline(always)]
    pub fn mesad_2(&self) -> Mesad2R {
        Mesad2R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`ur8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur8Spec;
impl crate::RegisterSpec for Ur8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur8::R`](R) reader structure"]
impl crate::Readable for Ur8Spec {}
#[doc = "`reset()` method sets UR8 to value 0"]
impl crate::Resettable for Ur8Spec {}
