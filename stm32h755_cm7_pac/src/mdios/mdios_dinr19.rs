#[doc = "Register `MDIOS_DINR19` reader"]
pub type R = crate::R<MdiosDinr19Spec>;
#[doc = "Field `DIN19` reader - Input data received from MDIO Master during write frames"]
pub type Din19R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din19(&self) -> Din19R {
        Din19R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 19\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr19::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr19Spec;
impl crate::RegisterSpec for MdiosDinr19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr19::R`](R) reader structure"]
impl crate::Readable for MdiosDinr19Spec {}
#[doc = "`reset()` method sets MDIOS_DINR19 to value 0"]
impl crate::Resettable for MdiosDinr19Spec {}
