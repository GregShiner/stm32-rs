#[doc = "Register `MDIOS_DINR29` reader"]
pub type R = crate::R<MdiosDinr29Spec>;
#[doc = "Field `DIN29` reader - Input data received from MDIO Master during write frames"]
pub type Din29R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din29(&self) -> Din29R {
        Din29R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 29\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr29::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr29Spec;
impl crate::RegisterSpec for MdiosDinr29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr29::R`](R) reader structure"]
impl crate::Readable for MdiosDinr29Spec {}
#[doc = "`reset()` method sets MDIOS_DINR29 to value 0"]
impl crate::Resettable for MdiosDinr29Spec {}
