#[doc = "Register `M3FDRH` reader"]
pub type R = crate::R<M3fdrhSpec>;
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub type FdatahR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FdatahR {
        FdatahR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3fdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3fdrhSpec;
impl crate::RegisterSpec for M3fdrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3fdrh::R`](R) reader structure"]
impl crate::Readable for M3fdrhSpec {}
#[doc = "`reset()` method sets M3FDRH to value 0"]
impl crate::Resettable for M3fdrhSpec {}
