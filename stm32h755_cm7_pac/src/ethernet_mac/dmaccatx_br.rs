#[doc = "Register `DMACCATxBR` reader"]
pub type R = crate::R<DmaccatxBrSpec>;
#[doc = "Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer"]
pub type CurtbufaptrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CurtbufaptrR {
        CurtbufaptrR::new(self.bits)
    }
}
#[doc = "Channel current application transmit buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaccatx_br::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaccatxBrSpec;
impl crate::RegisterSpec for DmaccatxBrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccatx_br::R`](R) reader structure"]
impl crate::Readable for DmaccatxBrSpec {}
#[doc = "`reset()` method sets DMACCATxBR to value 0"]
impl crate::Resettable for DmaccatxBrSpec {}
