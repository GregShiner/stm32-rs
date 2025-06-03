#[doc = "Register `MDIOS_DINR7` reader"]
pub type R = crate::R<MdiosDinr7Spec>;
#[doc = "Field `DIN7` reader - Input data received from MDIO Master during write frames"]
pub type Din7R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din7(&self) -> Din7R {
        Din7R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr7Spec;
impl crate::RegisterSpec for MdiosDinr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr7::R`](R) reader structure"]
impl crate::Readable for MdiosDinr7Spec {}
#[doc = "`reset()` method sets MDIOS_DINR7 to value 0"]
impl crate::Resettable for MdiosDinr7Spec {}
