#[doc = "Register `M5FDRH` reader"]
pub type R = crate::R<M5fdrhSpec>;
#[doc = "Field `FEC` reader - Failing error code"]
pub type FecR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FecR {
        FecR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5fdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5fdrhSpec;
impl crate::RegisterSpec for M5fdrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5fdrh::R`](R) reader structure"]
impl crate::Readable for M5fdrhSpec {}
#[doc = "`reset()` method sets M5FDRH to value 0"]
impl crate::Resettable for M5fdrhSpec {}
