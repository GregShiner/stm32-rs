#[doc = "Register `KEYR2` reader"]
pub type R = crate::R<Keyr2Spec>;
#[doc = "Field `KEYR2` reader - Bank 2 access configuration unlock key"]
pub type Keyr2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bank 2 access configuration unlock key"]
    #[inline(always)]
    pub fn keyr2(&self) -> Keyr2R {
        Keyr2R::new(self.bits)
    }
}
#[doc = "FLASH key register for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`keyr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Keyr2Spec;
impl crate::RegisterSpec for Keyr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr2::R`](R) reader structure"]
impl crate::Readable for Keyr2Spec {}
#[doc = "`reset()` method sets KEYR2 to value 0"]
impl crate::Resettable for Keyr2Spec {}
