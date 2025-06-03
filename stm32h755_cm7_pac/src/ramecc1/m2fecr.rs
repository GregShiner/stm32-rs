#[doc = "Register `M2FECR` reader"]
pub type R = crate::R<M2fecrSpec>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FaddR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FaddR {
        FaddR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2fecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2fecrSpec;
impl crate::RegisterSpec for M2fecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2fecr::R`](R) reader structure"]
impl crate::Readable for M2fecrSpec {}
#[doc = "`reset()` method sets M2FECR to value 0"]
impl crate::Resettable for M2fecrSpec {}
