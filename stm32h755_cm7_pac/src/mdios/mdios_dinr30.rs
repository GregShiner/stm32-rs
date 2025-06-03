#[doc = "Register `MDIOS_DINR30` reader"]
pub type R = crate::R<MdiosDinr30Spec>;
#[doc = "Field `DIN30` reader - Input data received from MDIO Master during write frames"]
pub type Din30R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din30(&self) -> Din30R {
        Din30R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 30\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr30::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr30Spec;
impl crate::RegisterSpec for MdiosDinr30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr30::R`](R) reader structure"]
impl crate::Readable for MdiosDinr30Spec {}
#[doc = "`reset()` method sets MDIOS_DINR30 to value 0"]
impl crate::Resettable for MdiosDinr30Spec {}
