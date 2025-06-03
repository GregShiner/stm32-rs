#[doc = "Register `CPT2CR` reader"]
pub type R = crate::R<Cpt2crSpec>;
#[doc = "Field `CPT2x` reader - Timerx Capture 2 value"]
pub type Cpt2xR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 2 value"]
    #[inline(always)]
    pub fn cpt2x(&self) -> Cpt2xR {
        Cpt2xR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpt2cr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpt2crSpec;
impl crate::RegisterSpec for Cpt2crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt2cr::R`](R) reader structure"]
impl crate::Readable for Cpt2crSpec {}
#[doc = "`reset()` method sets CPT2CR to value 0"]
impl crate::Resettable for Cpt2crSpec {}
