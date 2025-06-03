#[doc = "Register `M4FAR` reader"]
pub type R = crate::R<M4farSpec>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FaddR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FaddR {
        FaddR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4far::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4farSpec;
impl crate::RegisterSpec for M4farSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4far::R`](R) reader structure"]
impl crate::Readable for M4farSpec {}
#[doc = "`reset()` method sets M4FAR to value 0"]
impl crate::Resettable for M4farSpec {}
