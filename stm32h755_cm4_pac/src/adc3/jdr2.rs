#[doc = "Register `JDR2` reader"]
pub type R = crate::R<Jdr2Spec>;
#[doc = "Field `JDATA2` reader - ADC group injected sequencer rank 2 conversion data"]
pub type Jdata2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ADC group injected sequencer rank 2 conversion data"]
    #[inline(always)]
    pub fn jdata2(&self) -> Jdata2R {
        Jdata2R::new(self.bits)
    }
}
#[doc = "ADC group injected sequencer rank 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr2Spec;
impl crate::RegisterSpec for Jdr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr2::R`](R) reader structure"]
impl crate::Readable for Jdr2Spec {}
#[doc = "`reset()` method sets JDR2 to value 0"]
impl crate::Resettable for Jdr2Spec {}
