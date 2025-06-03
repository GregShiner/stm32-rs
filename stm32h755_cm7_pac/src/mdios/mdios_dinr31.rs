#[doc = "Register `MDIOS_DINR31` reader"]
pub type R = crate::R<MdiosDinr31Spec>;
#[doc = "Field `DIN31` reader - Input data received from MDIO Master during write frames"]
pub type Din31R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din31(&self) -> Din31R {
        Din31R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 31\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr31::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr31Spec;
impl crate::RegisterSpec for MdiosDinr31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr31::R`](R) reader structure"]
impl crate::Readable for MdiosDinr31Spec {}
#[doc = "`reset()` method sets MDIOS_DINR31 to value 0"]
impl crate::Resettable for MdiosDinr31Spec {}
