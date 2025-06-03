#[doc = "Register `MDIOS_DINR8` reader"]
pub type R = crate::R<MdiosDinr8Spec>;
#[doc = "Field `DIN8` reader - Input data received from MDIO Master during write frames"]
pub type Din8R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din8(&self) -> Din8R {
        Din8R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr8Spec;
impl crate::RegisterSpec for MdiosDinr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr8::R`](R) reader structure"]
impl crate::Readable for MdiosDinr8Spec {}
#[doc = "`reset()` method sets MDIOS_DINR8 to value 0"]
impl crate::Resettable for MdiosDinr8Spec {}
