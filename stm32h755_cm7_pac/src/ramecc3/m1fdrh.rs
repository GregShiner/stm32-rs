#[doc = "Register `M1FDRH` reader"]
pub type R = crate::R<M1fdrhSpec>;
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub type FdatahR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FdatahR {
        FdatahR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1fdrh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1fdrhSpec;
impl crate::RegisterSpec for M1fdrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1fdrh::R`](R) reader structure"]
impl crate::Readable for M1fdrhSpec {}
#[doc = "`reset()` method sets M1FDRH to value 0"]
impl crate::Resettable for M1fdrhSpec {}
