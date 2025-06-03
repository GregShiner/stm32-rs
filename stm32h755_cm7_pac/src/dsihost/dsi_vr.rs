#[doc = "Register `DSI_VR` reader"]
pub type R = crate::R<DsiVrSpec>;
#[doc = "Field `VERSION` reader - VERSION"]
pub type VersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - VERSION"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(self.bits)
    }
}
#[doc = "DSI Host version register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVrSpec;
impl crate::RegisterSpec for DsiVrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vr::R`](R) reader structure"]
impl crate::Readable for DsiVrSpec {}
#[doc = "`reset()` method sets DSI_VR to value 0x3133_302a"]
impl crate::Resettable for DsiVrSpec {
    const RESET_VALUE: u32 = 0x3133_302a;
}
