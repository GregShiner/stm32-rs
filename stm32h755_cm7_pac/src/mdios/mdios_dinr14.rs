#[doc = "Register `MDIOS_DINR14` reader"]
pub type R = crate::R<MdiosDinr14Spec>;
#[doc = "Field `DIN14` reader - Input data received from MDIO Master during write frames"]
pub type Din14R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din14(&self) -> Din14R {
        Din14R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr14Spec;
impl crate::RegisterSpec for MdiosDinr14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr14::R`](R) reader structure"]
impl crate::Readable for MdiosDinr14Spec {}
#[doc = "`reset()` method sets MDIOS_DINR14 to value 0"]
impl crate::Resettable for MdiosDinr14Spec {}
