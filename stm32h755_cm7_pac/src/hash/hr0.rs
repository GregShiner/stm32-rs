#[doc = "Register `HR0` reader"]
pub type R = crate::R<Hr0Spec>;
#[doc = "Field `H0` reader - H0"]
pub type H0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H0"]
    #[inline(always)]
    pub fn h0(&self) -> H0R {
        H0R::new(self.bits)
    }
}
#[doc = "digest registers\n\nYou can [`read`](crate::Reg::read) this register and get [`hr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hr0Spec;
impl crate::RegisterSpec for Hr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr0::R`](R) reader structure"]
impl crate::Readable for Hr0Spec {}
#[doc = "`reset()` method sets HR0 to value 0"]
impl crate::Resettable for Hr0Spec {}
