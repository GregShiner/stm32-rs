#[doc = "Register `HSEM_C2MISR` reader"]
pub type R = crate::R<HsemC2misrSpec>;
#[doc = "Field `MISF` reader - masked interrupt semaphore x status bit after enable (mask)"]
pub type MisfR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - masked interrupt semaphore x status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf(&self) -> MisfR {
        MisfR::new(self.bits)
    }
}
#[doc = "HSEM Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c2misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsemC2misrSpec;
impl crate::RegisterSpec for HsemC2misrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c2misr::R`](R) reader structure"]
impl crate::Readable for HsemC2misrSpec {}
#[doc = "`reset()` method sets HSEM_C2MISR to value 0"]
impl crate::Resettable for HsemC2misrSpec {}
