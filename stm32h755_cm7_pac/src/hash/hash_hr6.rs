#[doc = "Register `HASH_HR6` reader"]
pub type R = crate::R<HashHr6Spec>;
#[doc = "Field `H6` reader - H6"]
pub type H6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H6"]
    #[inline(always)]
    pub fn h6(&self) -> H6R {
        H6R::new(self.bits)
    }
}
#[doc = "read-only\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_hr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashHr6Spec;
impl crate::RegisterSpec for HashHr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hr6::R`](R) reader structure"]
impl crate::Readable for HashHr6Spec {}
#[doc = "`reset()` method sets HASH_HR6 to value 0"]
impl crate::Resettable for HashHr6Spec {}
