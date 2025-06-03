#[doc = "Register `CPT1ER` reader"]
pub type R = crate::R<Cpt1erSpec>;
#[doc = "Field `CPT1x` reader - Timerx Capture 1 value"]
pub type Cpt1xR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 1 value"]
    #[inline(always)]
    pub fn cpt1x(&self) -> Cpt1xR {
        Cpt1xR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpt1er::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpt1erSpec;
impl crate::RegisterSpec for Cpt1erSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt1er::R`](R) reader structure"]
impl crate::Readable for Cpt1erSpec {}
#[doc = "`reset()` method sets CPT1ER to value 0"]
impl crate::Resettable for Cpt1erSpec {}
