#[doc = "Register `DMACCARxDR` reader"]
pub type R = crate::R<DmaccarxDrSpec>;
#[doc = "Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer"]
pub type CurrdesaptrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Application Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CurrdesaptrR {
        CurrdesaptrR::new(self.bits)
    }
}
#[doc = "Channel current application receive descriptor register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaccarx_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaccarxDrSpec;
impl crate::RegisterSpec for DmaccarxDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaccarx_dr::R`](R) reader structure"]
impl crate::Readable for DmaccarxDrSpec {}
#[doc = "`reset()` method sets DMACCARxDR to value 0"]
impl crate::Resettable for DmaccarxDrSpec {}
