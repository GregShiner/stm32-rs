#[doc = "Register `MDIOS_DINR21` reader"]
pub type R = crate::R<MdiosDinr21Spec>;
#[doc = "Field `DIN21` reader - Input data received from MDIO Master during write frames"]
pub type Din21R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din21(&self) -> Din21R {
        Din21R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 21\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr21::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr21Spec;
impl crate::RegisterSpec for MdiosDinr21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr21::R`](R) reader structure"]
impl crate::Readable for MdiosDinr21Spec {}
#[doc = "`reset()` method sets MDIOS_DINR21 to value 0"]
impl crate::Resettable for MdiosDinr21Spec {}
