#[doc = "Register `DSI_VHBPCCR` reader"]
pub type R = crate::R<DsiVhbpccrSpec>;
#[doc = "Field `HBP` reader - HBP"]
pub type HbpR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - HBP"]
    #[inline(always)]
    pub fn hbp(&self) -> HbpR {
        HbpR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host video HBP current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vhbpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVhbpccrSpec;
impl crate::RegisterSpec for DsiVhbpccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vhbpccr::R`](R) reader structure"]
impl crate::Readable for DsiVhbpccrSpec {}
#[doc = "`reset()` method sets DSI_VHBPCCR to value 0"]
impl crate::Resettable for DsiVhbpccrSpec {}
