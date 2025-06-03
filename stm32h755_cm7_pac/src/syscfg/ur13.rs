#[doc = "Register `UR13` reader"]
pub type R = crate::R<Ur13Spec>;
#[doc = "Field `SDRS` reader - Secured DTCM RAM Size"]
pub type SdrsR = crate::FieldReader;
#[doc = "Field `D1SBRST` reader - D1 Standby reset"]
pub type D1sbrstR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Secured DTCM RAM Size"]
    #[inline(always)]
    pub fn sdrs(&self) -> SdrsR {
        SdrsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - D1 Standby reset"]
    #[inline(always)]
    pub fn d1sbrst(&self) -> D1sbrstR {
        D1sbrstR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`ur13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur13Spec;
impl crate::RegisterSpec for Ur13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur13::R`](R) reader structure"]
impl crate::Readable for Ur13Spec {}
#[doc = "`reset()` method sets UR13 to value 0"]
impl crate::Resettable for Ur13Spec {}
