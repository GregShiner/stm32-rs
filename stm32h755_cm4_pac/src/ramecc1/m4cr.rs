#[doc = "Register `M4CR` reader"]
pub type R = crate::R<M4crSpec>;
#[doc = "Field `FDATAL` reader - Failing data low"]
pub type FdatalR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FdatalR {
        FdatalR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4crSpec;
impl crate::RegisterSpec for M4crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4cr::R`](R) reader structure"]
impl crate::Readable for M4crSpec {}
#[doc = "`reset()` method sets M4CR to value 0"]
impl crate::Resettable for M4crSpec {}
