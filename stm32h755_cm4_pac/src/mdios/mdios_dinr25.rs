#[doc = "Register `MDIOS_DINR25` reader"]
pub type R = crate::R<MdiosDinr25Spec>;
#[doc = "Field `DIN25` reader - Input data received from MDIO Master during write frames"]
pub type Din25R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din25(&self) -> Din25R {
        Din25R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 25\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr25::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDinr25Spec;
impl crate::RegisterSpec for MdiosDinr25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_dinr25::R`](R) reader structure"]
impl crate::Readable for MdiosDinr25Spec {}
#[doc = "`reset()` method sets MDIOS_DINR25 to value 0"]
impl crate::Resettable for MdiosDinr25Spec {}
