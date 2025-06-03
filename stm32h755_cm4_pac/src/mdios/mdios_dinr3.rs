#[doc = "Register `MDIOS_DINR3` reader"]
pub type R = crate::R<MdiosDinr3Spec>;
#[doc = "Field `DIN3` reader - Input data received from MDIO Master during write frames"]
pub type Din3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din3(&self) -> Din3R {
        Din3R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr3Spec;
impl crate::RegisterSpec for MdiosDinr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr3::R`](R) reader structure"]
impl crate::Readable for MdiosDinr3Spec {}
#[doc = "`reset()` method sets MDIOS_DINR3 to value 0"]
impl crate::Resettable for MdiosDinr3Spec {}
