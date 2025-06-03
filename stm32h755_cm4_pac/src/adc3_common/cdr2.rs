#[doc = "Register `CDR2` reader"]
pub type R = crate::R<Cdr2Spec>;
#[doc = "Field `RDATA_ALT` reader - Regular data of the master/slave alternated ADCs"]
pub type RdataAltR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Regular data of the master/slave alternated ADCs"]
    #[inline(always)]
    pub fn rdata_alt(&self) -> RdataAltR {
        RdataAltR::new(self.bits)
    }
}
#[doc = "ADC x common regular data register for 32-bit dual mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cdr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cdr2Spec;
impl crate::RegisterSpec for Cdr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdr2::R`](R) reader structure"]
impl crate::Readable for Cdr2Spec {}
#[doc = "`reset()` method sets CDR2 to value 0"]
impl crate::Resettable for Cdr2Spec {}
