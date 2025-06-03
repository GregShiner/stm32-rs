#[doc = "Register `AXI_PERIPH_ID_1` reader"]
pub type R = crate::R<AxiPeriphId1Spec>;
#[doc = "Field `PARTNUM` reader - Peripheral part number bits 8 to 11"]
pub type PartnumR = crate::FieldReader;
#[doc = "Field `JEP106I` reader - JEP106 identity bits 0 to 3"]
pub type Jep106iR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Peripheral part number bits 8 to 11"]
    #[inline(always)]
    pub fn partnum(&self) -> PartnumR {
        PartnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - JEP106 identity bits 0 to 3"]
    #[inline(always)]
    pub fn jep106i(&self) -> Jep106iR {
        Jep106iR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_periph_id_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiPeriphId1Spec;
impl crate::RegisterSpec for AxiPeriphId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_periph_id_1::R`](R) reader structure"]
impl crate::Readable for AxiPeriphId1Spec {}
#[doc = "`reset()` method sets AXI_PERIPH_ID_1 to value 0x04"]
impl crate::Resettable for AxiPeriphId1Spec {
    const RESET_VALUE: u32 = 0x04;
}
