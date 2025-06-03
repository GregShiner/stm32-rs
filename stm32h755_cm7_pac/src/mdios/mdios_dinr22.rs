#[doc = "Register `MDIOS_DINR22` reader"]
pub type R = crate::R<MdiosDinr22Spec>;
#[doc = "Field `DIN22` reader - Input data received from MDIO Master during write frames"]
pub type Din22R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din22(&self) -> Din22R {
        Din22R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 22\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr22::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr22Spec;
impl crate::RegisterSpec for MdiosDinr22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr22::R`](R) reader structure"]
impl crate::Readable for MdiosDinr22Spec {}
#[doc = "`reset()` method sets MDIOS_DINR22 to value 0"]
impl crate::Resettable for MdiosDinr22Spec {}
