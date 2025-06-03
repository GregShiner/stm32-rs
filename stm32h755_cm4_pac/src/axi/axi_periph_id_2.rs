#[doc = "Register `AXI_PERIPH_ID_2` reader"]
pub type R = crate::R<AxiPeriphId2Spec>;
#[doc = "Field `JEP106ID` reader - JEP106 Identity bits 4 to 6"]
pub type Jep106idR = crate::FieldReader;
#[doc = "Field `JEDEC` reader - JEP106 code flag"]
pub type JedecR = crate::BitReader;
#[doc = "Field `REVISION` reader - Peripheral revision number"]
pub type RevisionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - JEP106 Identity bits 4 to 6"]
    #[inline(always)]
    pub fn jep106id(&self) -> Jep106idR {
        Jep106idR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - JEP106 code flag"]
    #[inline(always)]
    pub fn jedec(&self) -> JedecR {
        JedecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Peripheral revision number"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_periph_id_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiPeriphId2Spec;
impl crate::RegisterSpec for AxiPeriphId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_periph_id_2::R`](R) reader structure"]
impl crate::Readable for AxiPeriphId2Spec {}
#[doc = "`reset()` method sets AXI_PERIPH_ID_2 to value 0x04"]
impl crate::Resettable for AxiPeriphId2Spec {
    const RESET_VALUE: u32 = 0x04;
}
