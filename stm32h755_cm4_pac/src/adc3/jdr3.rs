#[doc = "Register `JDR3` reader"]
pub type R = crate::R<Jdr3Spec>;
#[doc = "Field `JDATA3` reader - ADC group injected sequencer rank 3 conversion data"]
pub type Jdata3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ADC group injected sequencer rank 3 conversion data"]
    #[inline(always)]
    pub fn jdata3(&self) -> Jdata3R {
        Jdata3R::new(self.bits)
    }
}
#[doc = "ADC group injected sequencer rank 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr3Spec;
impl crate::RegisterSpec for Jdr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr3::R`](R) reader structure"]
impl crate::Readable for Jdr3Spec {}
#[doc = "`reset()` method sets JDR3 to value 0"]
impl crate::Resettable for Jdr3Spec {}
