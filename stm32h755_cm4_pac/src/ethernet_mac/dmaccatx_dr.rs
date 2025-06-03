#[doc = "Register `DMACCATxDR` reader"]
pub type R = crate::R<DmaccatxDrSpec>;
#[doc = "Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer"]
pub type CurtdesaptrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CurtdesaptrR {
        CurtdesaptrR::new(self.bits)
    }
}
#[doc = "Channel current application transmit descriptor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaccatx_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaccatxDrSpec;
impl crate::RegisterSpec for DmaccatxDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccatx_dr::R`](R) reader structure"]
impl crate::Readable for DmaccatxDrSpec {}
#[doc = "`reset()` method sets DMACCATxDR to value 0"]
impl crate::Resettable for DmaccatxDrSpec {}
