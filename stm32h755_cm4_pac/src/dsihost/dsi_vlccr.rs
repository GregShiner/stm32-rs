#[doc = "Register `DSI_VLCCR` reader"]
pub type R = crate::R<DsiVlccrSpec>;
#[doc = "Field `HLINE` reader - HLINE"]
pub type HlineR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - HLINE"]
    #[inline(always)]
    pub fn hline(&self) -> HlineR {
        HlineR::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "DSI Host video line current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vlccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVlccrSpec;
impl crate::RegisterSpec for DsiVlccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vlccr::R`](R) reader structure"]
impl crate::Readable for DsiVlccrSpec {}
#[doc = "`reset()` method sets DSI_VLCCR to value 0"]
impl crate::Resettable for DsiVlccrSpec {}
