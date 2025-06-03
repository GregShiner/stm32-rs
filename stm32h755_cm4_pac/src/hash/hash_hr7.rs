#[doc = "Register `HASH_HR7` reader"]
pub type R = crate::R<HashHr7Spec>;
#[doc = "Field `H7` reader - H7"]
pub type H7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H7"]
    #[inline(always)]
    pub fn h7(&self) -> H7R {
        H7R::new(self.bits)
    }
}
#[doc = "read-only\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_hr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashHr7Spec;
impl crate::RegisterSpec for HashHr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hr7::R`](R) reader structure"]
impl crate::Readable for HashHr7Spec {}
#[doc = "`reset()` method sets HASH_HR7 to value 0"]
impl crate::Resettable for HashHr7Spec {}
