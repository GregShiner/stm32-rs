#[doc = "Register `MDIOS_DINR18` reader"]
pub type R = crate::R<MdiosDinr18Spec>;
#[doc = "Field `DIN18` reader - Input data received from MDIO Master during write frames"]
pub type Din18R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din18(&self) -> Din18R {
        Din18R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr18::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr18Spec;
impl crate::RegisterSpec for MdiosDinr18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr18::R`](R) reader structure"]
impl crate::Readable for MdiosDinr18Spec {}
#[doc = "`reset()` method sets MDIOS_DINR18 to value 0"]
impl crate::Resettable for MdiosDinr18Spec {}
