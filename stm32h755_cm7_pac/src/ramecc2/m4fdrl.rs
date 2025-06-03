#[doc = "Register `M4FDRL` reader"]
pub type R = crate::R<M4fdrlSpec>;
#[doc = "Field `FDATAL` reader - Failing data low"]
pub type FdatalR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data low"]
    #[inline(always)]
    pub fn fdatal(&self) -> FdatalR {
        FdatalR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4fdrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4fdrlSpec;
impl crate::RegisterSpec for M4fdrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4fdrl::R`](R) reader structure"]
impl crate::Readable for M4fdrlSpec {}
#[doc = "`reset()` method sets M4FDRL to value 0"]
impl crate::Resettable for M4fdrlSpec {}
