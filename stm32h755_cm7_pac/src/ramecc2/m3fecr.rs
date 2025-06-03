#[doc = "Register `M3FECR` reader"]
pub type R = crate::R<M3fecrSpec>;
#[doc = "Field `FEC` reader - Failing error code"]
pub type FecR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FecR {
        FecR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3fecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3fecrSpec;
impl crate::RegisterSpec for M3fecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3fecr::R`](R) reader structure"]
impl crate::Readable for M3fecrSpec {}
#[doc = "`reset()` method sets M3FECR to value 0"]
impl crate::Resettable for M3fecrSpec {}
