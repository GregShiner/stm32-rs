#[doc = "Register `HSEM_C1ISR` reader"]
pub type R = crate::R<HsemC1isrSpec>;
#[doc = "Field `ISF` reader - Interrupt semaphore x status bit before enable (mask)"]
pub type IsfR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt semaphore x status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf(&self) -> IsfR {
        IsfR::new(self.bits)
    }
}
#[doc = "HSEM Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsemC1isrSpec;
impl crate::RegisterSpec for HsemC1isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c1isr::R`](R) reader structure"]
impl crate::Readable for HsemC1isrSpec {}
#[doc = "`reset()` method sets HSEM_C1ISR to value 0"]
impl crate::Resettable for HsemC1isrSpec {}
