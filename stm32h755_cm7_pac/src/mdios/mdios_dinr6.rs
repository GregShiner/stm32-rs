#[doc = "Register `MDIOS_DINR6` reader"]
pub type R = crate::R<MdiosDinr6Spec>;
#[doc = "Field `DIN6` reader - Input data received from MDIO Master during write frames"]
pub type Din6R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din6(&self) -> Din6R {
        Din6R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr6Spec;
impl crate::RegisterSpec for MdiosDinr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr6::R`](R) reader structure"]
impl crate::Readable for MdiosDinr6Spec {}
#[doc = "`reset()` method sets MDIOS_DINR6 to value 0"]
impl crate::Resettable for MdiosDinr6Spec {}
