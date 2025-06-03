#[doc = "Register `MDIOS_DINR2` reader"]
pub type R = crate::R<MdiosDinr2Spec>;
#[doc = "Field `DIN2` reader - Input data received from MDIO Master during write frames"]
pub type Din2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din2(&self) -> Din2R {
        Din2R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr2Spec;
impl crate::RegisterSpec for MdiosDinr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr2::R`](R) reader structure"]
impl crate::Readable for MdiosDinr2Spec {}
#[doc = "`reset()` method sets MDIOS_DINR2 to value 0"]
impl crate::Resettable for MdiosDinr2Spec {}
