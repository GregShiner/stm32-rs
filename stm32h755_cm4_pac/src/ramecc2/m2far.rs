#[doc = "Register `M2FAR` reader"]
pub type R = crate::R<M2farSpec>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FaddR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FaddR {
        FaddR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2far::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2farSpec;
impl crate::RegisterSpec for M2farSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2far::R`](R) reader structure"]
impl crate::Readable for M2farSpec {}
#[doc = "`reset()` method sets M2FAR to value 0"]
impl crate::Resettable for M2farSpec {}
