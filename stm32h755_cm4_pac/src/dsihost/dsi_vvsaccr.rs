#[doc = "Register `DSI_VVSACCR` reader"]
pub type R = crate::R<DsiVvsaccrSpec>;
#[doc = "Field `VSA` reader - VSA"]
pub type VsaR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - VSA"]
    #[inline(always)]
    pub fn vsa(&self) -> VsaR {
        VsaR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host video VSA current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvsaccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVvsaccrSpec;
impl crate::RegisterSpec for DsiVvsaccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvsaccr::R`](R) reader structure"]
impl crate::Readable for DsiVvsaccrSpec {}
#[doc = "`reset()` method sets DSI_VVSACCR to value 0"]
impl crate::Resettable for DsiVvsaccrSpec {}
