#[doc = "Register `MDIOS_DINR28` reader"]
pub type R = crate::R<MdiosDinr28Spec>;
#[doc = "Field `DIN28` reader - Input data received from MDIO Master during write frames"]
pub type Din28R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din28(&self) -> Din28R {
        Din28R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 28\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr28::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr28Spec;
impl crate::RegisterSpec for MdiosDinr28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr28::R`](R) reader structure"]
impl crate::Readable for MdiosDinr28Spec {}
#[doc = "`reset()` method sets MDIOS_DINR28 to value 0"]
impl crate::Resettable for MdiosDinr28Spec {}
