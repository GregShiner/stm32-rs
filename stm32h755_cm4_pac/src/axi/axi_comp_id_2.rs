#[doc = "Register `AXI_COMP_ID_2` reader"]
pub type R = crate::R<AxiCompId2Spec>;
#[doc = "Field `PREAMBLE` reader - Preamble bits 12 to 19"]
pub type PreambleR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble bits 12 to 19"]
    #[inline(always)]
    pub fn preamble(&self) -> PreambleR {
        PreambleR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - component ID2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_comp_id_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiCompId2Spec;
impl crate::RegisterSpec for AxiCompId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_comp_id_2::R`](R) reader structure"]
impl crate::Readable for AxiCompId2Spec {}
#[doc = "`reset()` method sets AXI_COMP_ID_2 to value 0x04"]
impl crate::Resettable for AxiCompId2Spec {
    const RESET_VALUE: u32 = 0x04;
}
