#[doc = "Register `DSI_VPCCR` reader"]
pub type R = crate::R<DsiVpccrSpec>;
#[doc = "Field `VPSIZE` reader - VPSIZE"]
pub type VpsizeR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - VPSIZE"]
    #[inline(always)]
    pub fn vpsize(&self) -> VpsizeR {
        VpsizeR::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "DSI Host video packet current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVpccrSpec;
impl crate::RegisterSpec for DsiVpccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vpccr::R`](R) reader structure"]
impl crate::Readable for DsiVpccrSpec {}
#[doc = "`reset()` method sets DSI_VPCCR to value 0"]
impl crate::Resettable for DsiVpccrSpec {}
