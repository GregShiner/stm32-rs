#[doc = "Register `MDIOS_DINR20` reader"]
pub type R = crate::R<MdiosDinr20Spec>;
#[doc = "Field `DIN20` reader - Input data received from MDIO Master during write frames"]
pub type Din20R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din20(&self) -> Din20R {
        Din20R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 20\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr20::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr20Spec;
impl crate::RegisterSpec for MdiosDinr20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr20::R`](R) reader structure"]
impl crate::Readable for MdiosDinr20Spec {}
#[doc = "`reset()` method sets MDIOS_DINR20 to value 0"]
impl crate::Resettable for MdiosDinr20Spec {}
