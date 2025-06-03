#[doc = "Register `HASH_HR5` reader"]
pub type R = crate::R<HashHr5Spec>;
#[doc = "Field `H5` reader - H5"]
pub type H5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - H5"]
    #[inline(always)]
    pub fn h5(&self) -> H5R {
        H5R::new(self.bits)
    }
}
#[doc = "read-only\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_hr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashHr5Spec;
impl crate::RegisterSpec for HashHr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_hr5::R`](R) reader structure"]
impl crate::Readable for HashHr5Spec {}
#[doc = "`reset()` method sets HASH_HR5 to value 0"]
impl crate::Resettable for HashHr5Spec {}
