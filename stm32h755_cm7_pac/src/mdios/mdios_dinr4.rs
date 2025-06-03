#[doc = "Register `MDIOS_DINR4` reader"]
pub type R = crate::R<MdiosDinr4Spec>;
#[doc = "Field `DIN4` reader - Input data received from MDIO Master during write frames"]
pub type Din4R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din4(&self) -> Din4R {
        Din4R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr4Spec;
impl crate::RegisterSpec for MdiosDinr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr4::R`](R) reader structure"]
impl crate::Readable for MdiosDinr4Spec {}
#[doc = "`reset()` method sets MDIOS_DINR4 to value 0"]
impl crate::Resettable for MdiosDinr4Spec {}
