#[doc = "Register `HASH_HR4` reader"]
pub type R = crate::R<HashHr4Spec>;
#[doc = "Field `H4` reader - H4"]
pub type H4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H4"]
    #[inline(always)]
    pub fn h4(&self) -> H4R {
        H4R::new(self.bits)
    }
}
#[doc = "read-only\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_hr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashHr4Spec;
impl crate::RegisterSpec for HashHr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hr4::R`](R) reader structure"]
impl crate::Readable for HashHr4Spec {}
#[doc = "`reset()` method sets HASH_HR4 to value 0"]
impl crate::Resettable for HashHr4Spec {}
