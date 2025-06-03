#[doc = "Register `HR2` reader"]
pub type R = crate::R<Hr2Spec>;
#[doc = "Field `H2` reader - H2"]
pub type H2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H2"]
    #[inline(always)]
    pub fn h2(&self) -> H2R {
        H2R::new(self.bits)
    }
}
#[doc = "digest registers\n\nYou can [`read`](crate::Reg::read) this register and get [`hr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hr2Spec;
impl crate::RegisterSpec for Hr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr2::R`](R) reader structure"]
impl crate::Readable for Hr2Spec {}
#[doc = "`reset()` method sets HR2 to value 0"]
impl crate::Resettable for Hr2Spec {}
