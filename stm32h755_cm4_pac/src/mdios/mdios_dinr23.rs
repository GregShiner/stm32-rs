#[doc = "Register `MDIOS_DINR23` reader"]
pub type R = crate::R<MdiosDinr23Spec>;
#[doc = "Field `DIN23` reader - Input data received from MDIO Master during write frames"]
pub type Din23R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din23(&self) -> Din23R {
        Din23R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 23\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr23::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr23Spec;
impl crate::RegisterSpec for MdiosDinr23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr23::R`](R) reader structure"]
impl crate::Readable for MdiosDinr23Spec {}
#[doc = "`reset()` method sets MDIOS_DINR23 to value 0"]
impl crate::Resettable for MdiosDinr23Spec {}
