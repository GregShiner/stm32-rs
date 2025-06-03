#[doc = "Register `HSEM_C2ISR` reader"]
pub type R = crate::R<HsemC2isrSpec>;
#[doc = "Field `ISF` reader - Interrupt semaphore x status bit before enable (mask)"]
pub type IsfR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt semaphore x status bit before enable (mask)"]
    #[inline(always)]
    pub fn isf(&self) -> IsfR {
        IsfR::new(self.bits)
    }
}
#[doc = "HSEM Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c2isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsemC2isrSpec;
impl crate::RegisterSpec for HsemC2isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_c2isr::R`](R) reader structure"]
impl crate::Readable for HsemC2isrSpec {}
#[doc = "`reset()` method sets HSEM_C2ISR to value 0"]
impl crate::Resettable for HsemC2isrSpec {}
