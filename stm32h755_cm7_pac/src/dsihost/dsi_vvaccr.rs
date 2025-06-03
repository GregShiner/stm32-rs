#[doc = "Register `DSI_VVACCR` reader"]
pub type R = crate::R<DsiVvaccrSpec>;
#[doc = "Field `VA` reader - VA"]
pub type VaR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - VA"]
    #[inline(always)]
    pub fn va(&self) -> VaR {
        VaR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "DSI Host video VA current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvaccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVvaccrSpec;
impl crate::RegisterSpec for DsiVvaccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvaccr::R`](R) reader structure"]
impl crate::Readable for DsiVvaccrSpec {}
#[doc = "`reset()` method sets DSI_VVACCR to value 0"]
impl crate::Resettable for DsiVvaccrSpec {}
