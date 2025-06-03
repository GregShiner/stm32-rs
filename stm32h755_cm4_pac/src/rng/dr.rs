#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Field `RNDATA` reader - Random data 32-bit random data which are valid when DRDY=1."]
pub type RndataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Random data 32-bit random data which are valid when DRDY=1."]
    #[inline(always)]
    pub fn rndata(&self) -> RndataR {
        RndataR::new(self.bits)
    }
}
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {}
