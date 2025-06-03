#[doc = "Register `AXI_COMP_ID_1` reader"]
pub type R = crate::R<AxiCompId1Spec>;
#[doc = "Field `PREAMBLE` reader - Preamble bits 8 to 11"]
pub type PreambleR = crate::FieldReader;
#[doc = "Field `CLASS` reader - Component class"]
pub type ClassR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Preamble bits 8 to 11"]
    #[inline(always)]
    pub fn preamble(&self) -> PreambleR {
        PreambleR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Component class"]
    #[inline(always)]
    pub fn class(&self) -> ClassR {
        ClassR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - component ID1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_comp_id_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiCompId1Spec;
impl crate::RegisterSpec for AxiCompId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_comp_id_1::R`](R) reader structure"]
impl crate::Readable for AxiCompId1Spec {}
#[doc = "`reset()` method sets AXI_COMP_ID_1 to value 0x04"]
impl crate::Resettable for AxiCompId1Spec {
    const RESET_VALUE: u32 = 0x04;
}
