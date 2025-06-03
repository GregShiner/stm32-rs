#[doc = "Register `MDIOS_DINR26` reader"]
pub type R = crate::R<MdiosDinr26Spec>;
#[doc = "Field `DIN26` reader - Input data received from MDIO Master during write frames"]
pub type Din26R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din26(&self) -> Din26R {
        Din26R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 26\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr26::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr26Spec;
impl crate::RegisterSpec for MdiosDinr26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr26::R`](R) reader structure"]
impl crate::Readable for MdiosDinr26Spec {}
#[doc = "`reset()` method sets MDIOS_DINR26 to value 0"]
impl crate::Resettable for MdiosDinr26Spec {}
