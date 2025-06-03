#[doc = "Register `MDIOS_DINR16` reader"]
pub type R = crate::R<MdiosDinr16Spec>;
#[doc = "Field `DIN16` reader - Input data received from MDIO Master during write frames"]
pub type Din16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din16(&self) -> Din16R {
        Din16R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr16Spec;
impl crate::RegisterSpec for MdiosDinr16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr16::R`](R) reader structure"]
impl crate::Readable for MdiosDinr16Spec {}
#[doc = "`reset()` method sets MDIOS_DINR16 to value 0"]
impl crate::Resettable for MdiosDinr16Spec {}
