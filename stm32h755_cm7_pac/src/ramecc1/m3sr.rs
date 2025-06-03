#[doc = "Register `M3SR` reader"]
pub type R = crate::R<M3srSpec>;
#[doc = "Field `FADD` reader - ECC error failing address"]
pub type FaddR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FaddR {
        FaddR::new(self.bits)
    }
}
#[doc = "RAMECC monitor x status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3srSpec;
impl crate::RegisterSpec for M3srSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3sr::R`](R) reader structure"]
impl crate::Readable for M3srSpec {}
#[doc = "`reset()` method sets M3SR to value 0"]
impl crate::Resettable for M3srSpec {}
