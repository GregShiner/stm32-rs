#[doc = "Register `MDIOS_WRFR` reader"]
pub type R = crate::R<MdiosWrfrSpec>;
#[doc = "Field `WRF` reader - Write flags for MDIO registers 0 to 31"]
pub type WrfR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write flags for MDIO registers 0 to 31"]
    #[inline(always)]
    pub fn wrf(&self) -> WrfR {
        WrfR::new(self.bits)
    }
}
#[doc = "MDIOS write flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_wrfr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosWrfrSpec;
impl crate::RegisterSpec for MdiosWrfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_wrfr::R`](R) reader structure"]
impl crate::Readable for MdiosWrfrSpec {}
#[doc = "`reset()` method sets MDIOS_WRFR to value 0"]
impl crate::Resettable for MdiosWrfrSpec {}
