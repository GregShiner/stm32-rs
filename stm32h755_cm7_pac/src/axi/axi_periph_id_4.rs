#[doc = "Register `AXI_PERIPH_ID_4` reader"]
pub type R = crate::R<AxiPeriphId4Spec>;
#[doc = "Field `JEP106CON` reader - JEP106 continuation code"]
pub type Jep106conR = crate::FieldReader;
#[doc = "Field `KCOUNT4` reader - Register file size"]
pub type Kcount4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - JEP106 continuation code"]
    #[inline(always)]
    pub fn jep106con(&self) -> Jep106conR {
        Jep106conR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Register file size"]
    #[inline(always)]
    pub fn kcount4(&self) -> Kcount4R {
        Kcount4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_periph_id_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiPeriphId4Spec;
impl crate::RegisterSpec for AxiPeriphId4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_periph_id_4::R`](R) reader structure"]
impl crate::Readable for AxiPeriphId4Spec {}
#[doc = "`reset()` method sets AXI_PERIPH_ID_4 to value 0x04"]
impl crate::Resettable for AxiPeriphId4Spec {
    const RESET_VALUE: u32 = 0x04;
}
