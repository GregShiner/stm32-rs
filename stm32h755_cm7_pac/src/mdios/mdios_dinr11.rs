#[doc = "Register `MDIOS_DINR11` reader"]
pub type R = crate::R<MdiosDinr11Spec>;
#[doc = "Field `DIN11` reader - Input data received from MDIO Master during write frames"]
pub type Din11R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din11(&self) -> Din11R {
        Din11R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr11Spec;
impl crate::RegisterSpec for MdiosDinr11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr11::R`](R) reader structure"]
impl crate::Readable for MdiosDinr11Spec {}
#[doc = "`reset()` method sets MDIOS_DINR11 to value 0"]
impl crate::Resettable for MdiosDinr11Spec {}
