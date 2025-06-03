#[doc = "Register `MDIOS_DINR15` reader"]
pub type R = crate::R<MdiosDinr15Spec>;
#[doc = "Field `DIN15` reader - Input data received from MDIO Master during write frames"]
pub type Din15R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din15(&self) -> Din15R {
        Din15R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr15Spec;
impl crate::RegisterSpec for MdiosDinr15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr15::R`](R) reader structure"]
impl crate::Readable for MdiosDinr15Spec {}
#[doc = "`reset()` method sets MDIOS_DINR15 to value 0"]
impl crate::Resettable for MdiosDinr15Spec {}
