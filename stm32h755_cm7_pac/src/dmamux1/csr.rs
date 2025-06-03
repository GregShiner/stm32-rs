#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Field `SOF` reader - Synchronization overrun event flag"]
pub type SofR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Synchronization overrun event flag"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "DMAMUX request line multiplexer interrupt channel status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {}
