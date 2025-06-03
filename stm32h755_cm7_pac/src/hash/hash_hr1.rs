#[doc = "Register `HASH_HR1` reader"]
pub type R = crate::R<HashHr1Spec>;
#[doc = "Field `H1` reader - H1"]
pub type H1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H1"]
    #[inline(always)]
    pub fn h1(&self) -> H1R {
        H1R::new(self.bits)
    }
}
#[doc = "read-only\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_hr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashHr1Spec;
impl crate::RegisterSpec for HashHr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hr1::R`](R) reader structure"]
impl crate::Readable for HashHr1Spec {}
#[doc = "`reset()` method sets HASH_HR1 to value 0"]
impl crate::Resettable for HashHr1Spec {}
