#[doc = "Register `AXI_COMP_ID_3` reader"]
pub type R = crate::R<AxiCompId3Spec>;
#[doc = "Field `PREAMBLE` reader - Preamble bits 20 to 27"]
pub type PreambleR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Preamble bits 20 to 27"]
    #[inline(always)]
    pub fn preamble(&self) -> PreambleR {
        PreambleR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - component ID3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_comp_id_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiCompId3Spec;
impl crate::RegisterSpec for AxiCompId3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_comp_id_3::R`](R) reader structure"]
impl crate::Readable for AxiCompId3Spec {}
#[doc = "`reset()` method sets AXI_COMP_ID_3 to value 0x04"]
impl crate::Resettable for AxiCompId3Spec {
    const RESET_VALUE: u32 = 0x04;
}
