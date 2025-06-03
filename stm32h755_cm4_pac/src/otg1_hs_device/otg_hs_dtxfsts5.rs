#[doc = "Register `OTG_HS_DTXFSTS5` reader"]
pub type R = crate::R<OtgHsDtxfsts5Spec>;
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space avail"]
pub type IneptfsavR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> IneptfsavR {
        IneptfsavR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_dtxfsts5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDtxfsts5Spec;
impl crate::RegisterSpec for OtgHsDtxfsts5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_dtxfsts5::R`](R) reader structure"]
impl crate::Readable for OtgHsDtxfsts5Spec {}
#[doc = "`reset()` method sets OTG_HS_DTXFSTS5 to value 0"]
impl crate::Resettable for OtgHsDtxfsts5Spec {}
