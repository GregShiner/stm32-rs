#[doc = "Register `AXI_COMP_ID_0` reader"]
pub type R = crate::R<AxiCompId0Spec>;
#[doc = "Field `PREAMBLE` reader - Preamble bits 0 to 7"]
pub type PreambleR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble bits 0 to 7"]
    #[inline(always)]
    pub fn preamble(&self) -> PreambleR {
        PreambleR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - component ID0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_comp_id_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiCompId0Spec;
impl crate::RegisterSpec for AxiCompId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_comp_id_0::R`](R) reader structure"]
impl crate::Readable for AxiCompId0Spec {}
#[doc = "`reset()` method sets AXI_COMP_ID_0 to value 0x04"]
impl crate::Resettable for AxiCompId0Spec {
    const RESET_VALUE: u32 = 0x04;
}
