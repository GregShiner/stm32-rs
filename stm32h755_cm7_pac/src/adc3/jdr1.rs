#[doc = "Register `JDR1` reader"]
pub type R = crate::R<Jdr1Spec>;
#[doc = "Field `JDATA1` reader - ADC group injected sequencer rank 1 conversion data"]
pub type Jdata1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ADC group injected sequencer rank 1 conversion data"]
    #[inline(always)]
    pub fn jdata1(&self) -> Jdata1R {
        Jdata1R::new(self.bits)
    }
}
#[doc = "ADC group injected sequencer rank 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr1Spec;
impl crate::RegisterSpec for Jdr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr1::R`](R) reader structure"]
impl crate::Readable for Jdr1Spec {}
#[doc = "`reset()` method sets JDR1 to value 0"]
impl crate::Resettable for Jdr1Spec {}
