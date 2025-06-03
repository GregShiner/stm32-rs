#[doc = "Register `DSI_VNPCCR` reader"]
pub type R = crate::R<DsiVnpccrSpec>;
#[doc = "Field `NPSIZE` reader - NPSIZE"]
pub type NpsizeR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - NPSIZE"]
    #[inline(always)]
    pub fn npsize(&self) -> NpsizeR {
        NpsizeR::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "DSI Host video null packet current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vnpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVnpccrSpec;
impl crate::RegisterSpec for DsiVnpccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vnpccr::R`](R) reader structure"]
impl crate::Readable for DsiVnpccrSpec {}
#[doc = "`reset()` method sets DSI_VNPCCR to value 0"]
impl crate::Resettable for DsiVnpccrSpec {}
