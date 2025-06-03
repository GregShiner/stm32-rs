#[doc = "Register `MDIOS_DINR10` reader"]
pub type R = crate::R<MdiosDinr10Spec>;
#[doc = "Field `DIN10` reader - Input data received from MDIO Master during write frames"]
pub type Din10R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din10(&self) -> Din10R {
        Din10R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr10Spec;
impl crate::RegisterSpec for MdiosDinr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr10::R`](R) reader structure"]
impl crate::Readable for MdiosDinr10Spec {}
#[doc = "`reset()` method sets MDIOS_DINR10 to value 0"]
impl crate::Resettable for MdiosDinr10Spec {}
