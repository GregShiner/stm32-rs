#[doc = "Register `DMACCARxBR` reader"]
pub type R = crate::R<DmaccarxBrSpec>;
#[doc = "Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer"]
pub type CurrbufaptrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CurrbufaptrR {
        CurrbufaptrR::new(self.bits)
    }
}
#[doc = "Channel current application receive buffer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaccarx_br::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaccarxBrSpec;
impl crate::RegisterSpec for DmaccarxBrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccarx_br::R`](R) reader structure"]
impl crate::Readable for DmaccarxBrSpec {}
#[doc = "`reset()` method sets DMACCARxBR to value 0"]
impl crate::Resettable for DmaccarxBrSpec {}
