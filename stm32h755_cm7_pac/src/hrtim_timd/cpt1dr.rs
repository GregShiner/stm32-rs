#[doc = "Register `CPT1DR` reader"]
pub type R = crate::R<Cpt1drSpec>;
#[doc = "Field `CPT1x` reader - Timerx Capture 1 value"]
pub type Cpt1xR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Capture 1 value"]
    #[inline(always)]
    pub fn cpt1x(&self) -> Cpt1xR {
        Cpt1xR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Timerx Capture 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpt1dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpt1drSpec;
impl crate::RegisterSpec for Cpt1drSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpt1dr::R`](R) reader structure"]
impl crate::Readable for Cpt1drSpec {}
#[doc = "`reset()` method sets CPT1DR to value 0"]
impl crate::Resettable for Cpt1drSpec {}
