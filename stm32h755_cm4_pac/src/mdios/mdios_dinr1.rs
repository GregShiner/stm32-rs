#[doc = "Register `MDIOS_DINR1` reader"]
pub type R = crate::R<MdiosDinr1Spec>;
#[doc = "Field `DIN1` reader - Input data received from MDIO Master during write frames"]
pub type Din1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din1(&self) -> Din1R {
        Din1R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr1Spec;
impl crate::RegisterSpec for MdiosDinr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr1::R`](R) reader structure"]
impl crate::Readable for MdiosDinr1Spec {}
#[doc = "`reset()` method sets MDIOS_DINR1 to value 0"]
impl crate::Resettable for MdiosDinr1Spec {}
