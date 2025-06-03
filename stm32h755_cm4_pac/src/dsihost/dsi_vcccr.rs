#[doc = "Register `DSI_VCCCR` reader"]
pub type R = crate::R<DsiVcccrSpec>;
#[doc = "Field `NUMC` reader - NUMC"]
pub type NumcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - NUMC"]
    #[inline(always)]
    pub fn numc(&self) -> NumcR {
        NumcR::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "DSI Host video chunks current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vcccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVcccrSpec;
impl crate::RegisterSpec for DsiVcccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vcccr::R`](R) reader structure"]
impl crate::Readable for DsiVcccrSpec {}
#[doc = "`reset()` method sets DSI_VCCCR to value 0"]
impl crate::Resettable for DsiVcccrSpec {}
