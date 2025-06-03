#[doc = "Register `MDIOS_DINR12` reader"]
pub type R = crate::R<MdiosDinr12Spec>;
#[doc = "Field `DIN12` reader - Input data received from MDIO Master during write frames"]
pub type Din12R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din12(&self) -> Din12R {
        Din12R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr12Spec;
impl crate::RegisterSpec for MdiosDinr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr12::R`](R) reader structure"]
impl crate::Readable for MdiosDinr12Spec {}
#[doc = "`reset()` method sets MDIOS_DINR12 to value 0"]
impl crate::Resettable for MdiosDinr12Spec {}
