#[doc = "Register `MDIOS_DINR5` reader"]
pub type R = crate::R<MdiosDinr5Spec>;
#[doc = "Field `DIN5` reader - Input data received from MDIO Master during write frames"]
pub type Din5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din5(&self) -> Din5R {
        Din5R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr5Spec;
impl crate::RegisterSpec for MdiosDinr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr5::R`](R) reader structure"]
impl crate::Readable for MdiosDinr5Spec {}
#[doc = "`reset()` method sets MDIOS_DINR5 to value 0"]
impl crate::Resettable for MdiosDinr5Spec {}
