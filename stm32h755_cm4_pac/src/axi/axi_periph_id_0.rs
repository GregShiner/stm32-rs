#[doc = "Register `AXI_PERIPH_ID_0` reader"]
pub type R = crate::R<AxiPeriphId0Spec>;
#[doc = "Field `PARTNUM` reader - Peripheral part number bits 0 to 7"]
pub type PartnumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Peripheral part number bits 0 to 7"]
    #[inline(always)]
    pub fn partnum(&self) -> PartnumR {
        PartnumR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_periph_id_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiPeriphId0Spec;
impl crate::RegisterSpec for AxiPeriphId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_periph_id_0::R`](R) reader structure"]
impl crate::Readable for AxiPeriphId0Spec {}
#[doc = "`reset()` method sets AXI_PERIPH_ID_0 to value 0x04"]
impl crate::Resettable for AxiPeriphId0Spec {
    const RESET_VALUE: u32 = 0x04;
}
