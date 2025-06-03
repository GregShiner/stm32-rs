#[doc = "Register `CCVR` reader"]
pub type R = crate::R<CcvrSpec>;
#[doc = "Field `NCV` reader - NMOS compensation value"]
pub type NcvR = crate::FieldReader;
#[doc = "Field `PCV` reader - PMOS compensation value"]
pub type PcvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - NMOS compensation value"]
    #[inline(always)]
    pub fn ncv(&self) -> NcvR {
        NcvR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PMOS compensation value"]
    #[inline(always)]
    pub fn pcv(&self) -> PcvR {
        PcvR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SYSCFG compensation cell value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcvrSpec;
impl crate::RegisterSpec for CcvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccvr::R`](R) reader structure"]
impl crate::Readable for CcvrSpec {}
#[doc = "`reset()` method sets CCVR to value 0"]
impl crate::Resettable for CcvrSpec {}
