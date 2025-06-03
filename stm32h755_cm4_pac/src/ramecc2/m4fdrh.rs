#[doc = "Register `M4FDRH` reader"]
pub type R = crate::R<M4fdrhSpec>;
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub type FdatahR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FdatahR {
        FdatahR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4fdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4fdrhSpec;
impl crate::RegisterSpec for M4fdrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4fdrh::R`](R) reader structure"]
impl crate::Readable for M4fdrhSpec {}
#[doc = "`reset()` method sets M4FDRH to value 0"]
impl crate::Resettable for M4fdrhSpec {}
