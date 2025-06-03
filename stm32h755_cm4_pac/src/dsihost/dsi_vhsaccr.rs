#[doc = "Register `DSI_VHSACCR` reader"]
pub type R = crate::R<DsiVhsaccrSpec>;
#[doc = "Field `HSA` reader - HSA"]
pub type HsaR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - HSA"]
    #[inline(always)]
    pub fn hsa(&self) -> HsaR {
        HsaR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host video HSA current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vhsaccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVhsaccrSpec;
impl crate::RegisterSpec for DsiVhsaccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vhsaccr::R`](R) reader structure"]
impl crate::Readable for DsiVhsaccrSpec {}
#[doc = "`reset()` method sets DSI_VHSACCR to value 0"]
impl crate::Resettable for DsiVhsaccrSpec {}
