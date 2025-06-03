#[doc = "Register `DSI_GVCIDR` reader"]
pub type R = crate::R<DsiGvcidrSpec>;
#[doc = "Field `VCID` reader - VCID"]
pub type VcidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - VCID"]
    #[inline(always)]
    pub fn vcid(&self) -> VcidR {
        VcidR::new((self.bits & 3) as u8)
    }
}
#[doc = "DSI Host generic VCID register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_gvcidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiGvcidrSpec;
impl crate::RegisterSpec for DsiGvcidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_gvcidr::R`](R) reader structure"]
impl crate::Readable for DsiGvcidrSpec {}
#[doc = "`reset()` method sets DSI_GVCIDR to value 0"]
impl crate::Resettable for DsiGvcidrSpec {}
