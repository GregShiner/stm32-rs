#[doc = "Register `HASH_HR2` reader"]
pub type R = crate::R<HashHr2Spec>;
#[doc = "Field `H2` reader - H2"]
pub type H2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H2"]
    #[inline(always)]
    pub fn h2(&self) -> H2R {
        H2R::new(self.bits)
    }
}
#[doc = "read-only\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_hr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashHr2Spec;
impl crate::RegisterSpec for HashHr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hr2::R`](R) reader structure"]
impl crate::Readable for HashHr2Spec {}
#[doc = "`reset()` method sets HASH_HR2 to value 0"]
impl crate::Resettable for HashHr2Spec {}
