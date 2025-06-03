#[doc = "Register `MDIOS_DINR0` reader"]
pub type R = crate::R<MdiosDinr0Spec>;
#[doc = "Field `DIN0` reader - Input data received from MDIO Master during write frames"]
pub type Din0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din0(&self) -> Din0R {
        Din0R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr0Spec;
impl crate::RegisterSpec for MdiosDinr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr0::R`](R) reader structure"]
impl crate::Readable for MdiosDinr0Spec {}
#[doc = "`reset()` method sets MDIOS_DINR0 to value 0"]
impl crate::Resettable for MdiosDinr0Spec {}
