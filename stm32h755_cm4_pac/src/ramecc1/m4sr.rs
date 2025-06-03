#[doc = "Register `M4SR` reader"]
pub type R = crate::R<M4srSpec>;
#[doc = "Field `FDATAL` reader - Failing data low"]
pub type FdatalR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FdatalR {
        FdatalR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4srSpec;
impl crate::RegisterSpec for M4srSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4sr::R`](R) reader structure"]
impl crate::Readable for M4srSpec {}
#[doc = "`reset()` method sets M4SR to value 0"]
impl crate::Resettable for M4srSpec {}
