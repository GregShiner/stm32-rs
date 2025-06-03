#[doc = "Register `DSI_VVBPCCR` reader"]
pub type R = crate::R<DsiVvbpccrSpec>;
#[doc = "Field `VBP` reader - VBP"]
pub type VbpR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - VBP"]
    #[inline(always)]
    pub fn vbp(&self) -> VbpR {
        VbpR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host video VBP current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvbpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVvbpccrSpec;
impl crate::RegisterSpec for DsiVvbpccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvbpccr::R`](R) reader structure"]
impl crate::Readable for DsiVvbpccrSpec {}
#[doc = "`reset()` method sets DSI_VVBPCCR to value 0"]
impl crate::Resettable for DsiVvbpccrSpec {}
