#[doc = "Register `AXI_PERIPH_ID_3` reader"]
pub type R = crate::R<AxiPeriphId3Spec>;
#[doc = "Field `CUST_MOD_NUM` reader - Customer modification"]
pub type CustModNumR = crate::FieldReader;
#[doc = "Field `REV_AND` reader - Customer version"]
pub type RevAndR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Customer modification"]
    #[inline(always)]
    pub fn cust_mod_num(&self) -> CustModNumR {
        CustModNumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Customer version"]
    #[inline(always)]
    pub fn rev_and(&self) -> RevAndR {
        RevAndR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXI interconnect - peripheral ID3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_periph_id_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiPeriphId3Spec;
impl crate::RegisterSpec for AxiPeriphId3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_periph_id_3::R`](R) reader structure"]
impl crate::Readable for AxiPeriphId3Spec {}
#[doc = "`reset()` method sets AXI_PERIPH_ID_3 to value 0x04"]
impl crate::Resettable for AxiPeriphId3Spec {
    const RESET_VALUE: u32 = 0x04;
}
