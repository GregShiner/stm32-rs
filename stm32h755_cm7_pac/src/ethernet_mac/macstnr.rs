#[doc = "Register `MACSTNR` reader"]
pub type R = crate::R<MacstnrSpec>;
#[doc = "Field `TSSS` reader - TSSS"]
pub type TsssR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - TSSS"]
    #[inline(always)]
    pub fn tsss(&self) -> TsssR {
        TsssR::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "System time nanoseconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macstnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacstnrSpec;
impl crate::RegisterSpec for MacstnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstnr::R`](R) reader structure"]
impl crate::Readable for MacstnrSpec {}
#[doc = "`reset()` method sets MACSTNR to value 0"]
impl crate::Resettable for MacstnrSpec {}
