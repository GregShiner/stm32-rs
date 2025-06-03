#[doc = "Register `JDR4` reader"]
pub type R = crate::R<Jdr4Spec>;
#[doc = "Field `JDATA4` reader - ADC group injected sequencer rank 4 conversion data"]
pub type Jdata4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ADC group injected sequencer rank 4 conversion data"]
    #[inline(always)]
    pub fn jdata4(&self) -> Jdata4R {
        Jdata4R::new(self.bits)
    }
}
#[doc = "ADC group injected sequencer rank 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jdr4Spec;
impl crate::RegisterSpec for Jdr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jdr4::R`](R) reader structure"]
impl crate::Readable for Jdr4Spec {}
#[doc = "`reset()` method sets JDR4 to value 0"]
impl crate::Resettable for Jdr4Spec {}
