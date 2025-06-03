#[doc = "Register `MDIOS_DINR9` reader"]
pub type R = crate::R<MdiosDinr9Spec>;
#[doc = "Field `DIN9` reader - Input data received from MDIO Master during write frames"]
pub type Din9R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din9(&self) -> Din9R {
        Din9R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr9Spec;
impl crate::RegisterSpec for MdiosDinr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr9::R`](R) reader structure"]
impl crate::Readable for MdiosDinr9Spec {}
#[doc = "`reset()` method sets MDIOS_DINR9 to value 0"]
impl crate::Resettable for MdiosDinr9Spec {}
