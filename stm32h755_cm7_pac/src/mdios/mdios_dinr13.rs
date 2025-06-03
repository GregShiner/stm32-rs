#[doc = "Register `MDIOS_DINR13` reader"]
pub type R = crate::R<MdiosDinr13Spec>;
#[doc = "Field `DIN13` reader - Input data received from MDIO Master during write frames"]
pub type Din13R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din13(&self) -> Din13R {
        Din13R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr13Spec;
impl crate::RegisterSpec for MdiosDinr13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr13::R`](R) reader structure"]
impl crate::Readable for MdiosDinr13Spec {}
#[doc = "`reset()` method sets MDIOS_DINR13 to value 0"]
impl crate::Resettable for MdiosDinr13Spec {}
