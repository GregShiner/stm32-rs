#[doc = "Register `UR5` reader"]
pub type R = crate::R<Ur5Spec>;
#[doc = "Field `MESAD_1` reader - Mass erase secured area disabled for bank 1"]
pub type Mesad1R = crate::BitReader;
#[doc = "Field `WRPS_1` reader - Write protection for flash bank 1"]
pub type Wrps1R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Mass erase secured area disabled for bank 1"]
    #[inline(always)]
    pub fn mesad_1(&self) -> Mesad1R {
        Mesad1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Write protection for flash bank 1"]
    #[inline(always)]
    pub fn wrps_1(&self) -> Wrps1R {
        Wrps1R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "SYSCFG user register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`ur5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur5Spec;
impl crate::RegisterSpec for Ur5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur5::R`](R) reader structure"]
impl crate::Readable for Ur5Spec {}
#[doc = "`reset()` method sets UR5 to value 0"]
impl crate::Resettable for Ur5Spec {}
