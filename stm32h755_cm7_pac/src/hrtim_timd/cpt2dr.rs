#[doc = "Register `CPT2DR` reader"]
pub type R = crate::R<Cpt2drSpec>;
#[doc = "Field `CPT2x` reader - Timerx Capture 2 value"]
pub type Cpt2xR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 2 value"]
    #[inline(always)]
    pub fn cpt2x(&self) -> Cpt2xR {
        Cpt2xR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpt2dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpt2drSpec;
impl crate::RegisterSpec for Cpt2drSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt2dr::R`](R) reader structure"]
impl crate::Readable for Cpt2drSpec {}
#[doc = "`reset()` method sets CPT2DR to value 0"]
impl crate::Resettable for Cpt2drSpec {}
