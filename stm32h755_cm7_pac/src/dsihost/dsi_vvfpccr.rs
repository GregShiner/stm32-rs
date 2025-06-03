#[doc = "Register `DSI_VVFPCCR` reader"]
pub type R = crate::R<DsiVvfpccrSpec>;
#[doc = "Field `VFP` reader - VFP"]
pub type VfpR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - VFP"]
    #[inline(always)]
    pub fn vfp(&self) -> VfpR {
        VfpR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host video VFP current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvfpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVvfpccrSpec;
impl crate::RegisterSpec for DsiVvfpccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvfpccr::R`](R) reader structure"]
impl crate::Readable for DsiVvfpccrSpec {}
#[doc = "`reset()` method sets DSI_VVFPCCR to value 0"]
impl crate::Resettable for DsiVvfpccrSpec {}
