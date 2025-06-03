#[doc = "Register `CPT2AR` reader"]
pub type R = crate::R<Cpt2arSpec>;
#[doc = "Field `CPT2x` reader - Timerx Capture 2 value"]
pub type Cpt2xR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 2 value"]
    #[inline(always)]
    pub fn cpt2x(&self) -> Cpt2xR {
        Cpt2xR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpt2ar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpt2arSpec;
impl crate::RegisterSpec for Cpt2arSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt2ar::R`](R) reader structure"]
impl crate::Readable for Cpt2arSpec {}
#[doc = "`reset()` method sets CPT2AR to value 0"]
impl crate::Resettable for Cpt2arSpec {}
