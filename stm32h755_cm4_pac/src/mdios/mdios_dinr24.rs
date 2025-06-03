#[doc = "Register `MDIOS_DINR24` reader"]
pub type R = crate::R<MdiosDinr24Spec>;
#[doc = "Field `DIN24` reader - Input data received from MDIO Master during write frames"]
pub type Din24R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din24(&self) -> Din24R {
        Din24R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 24\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr24::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr24Spec;
impl crate::RegisterSpec for MdiosDinr24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr24::R`](R) reader structure"]
impl crate::Readable for MdiosDinr24Spec {}
#[doc = "`reset()` method sets MDIOS_DINR24 to value 0"]
impl crate::Resettable for MdiosDinr24Spec {}
