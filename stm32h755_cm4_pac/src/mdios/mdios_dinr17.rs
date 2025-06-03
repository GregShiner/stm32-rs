#[doc = "Register `MDIOS_DINR17` reader"]
pub type R = crate::R<MdiosDinr17Spec>;
#[doc = "Field `DIN17` reader - Input data received from MDIO Master during write frames"]
pub type Din17R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din17(&self) -> Din17R {
        Din17R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr17::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr17Spec;
impl crate::RegisterSpec for MdiosDinr17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr17::R`](R) reader structure"]
impl crate::Readable for MdiosDinr17Spec {}
#[doc = "`reset()` method sets MDIOS_DINR17 to value 0"]
impl crate::Resettable for MdiosDinr17Spec {}
