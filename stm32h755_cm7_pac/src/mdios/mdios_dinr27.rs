#[doc = "Register `MDIOS_DINR27` reader"]
pub type R = crate::R<MdiosDinr27Spec>;
#[doc = "Field `DIN27` reader - Input data received from MDIO Master during write frames"]
pub type Din27R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din27(&self) -> Din27R {
        Din27R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 27\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr27::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr27Spec;
impl crate::RegisterSpec for MdiosDinr27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr27::R`](R) reader structure"]
impl crate::Readable for MdiosDinr27Spec {}
#[doc = "`reset()` method sets MDIOS_DINR27 to value 0"]
impl crate::Resettable for MdiosDinr27Spec {}
