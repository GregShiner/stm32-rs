#[doc = "Register `DSI_LPMCCR` reader"]
pub type R = crate::R<DsiLpmccrSpec>;
#[doc = "Field `VLPSIZE` reader - VLPSIZE"]
pub type VlpsizeR = crate::FieldReader;
#[doc = "Field `LPSIZE` reader - LPSIZE"]
pub type LpsizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - VLPSIZE"]
    #[inline(always)]
    pub fn vlpsize(&self) -> VlpsizeR {
        VlpsizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LPSIZE"]
    #[inline(always)]
    pub fn lpsize(&self) -> LpsizeR {
        LpsizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "DSI Host low-power mode current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lpmccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiLpmccrSpec;
impl crate::RegisterSpec for DsiLpmccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lpmccr::R`](R) reader structure"]
impl crate::Readable for DsiLpmccrSpec {}
#[doc = "`reset()` method sets DSI_LPMCCR to value 0"]
impl crate::Resettable for DsiLpmccrSpec {}
