#[doc = "Register `MACATSSR` reader"]
pub type R = crate::R<MacatssrSpec>;
#[doc = "Field `AUXTSHI` reader - AUXTSHI"]
pub type AuxtshiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - AUXTSHI"]
    #[inline(always)]
    pub fn auxtshi(&self) -> AuxtshiR {
        AuxtshiR::new(self.bits)
    }
}
#[doc = "Auxiliary timestamp seconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macatssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacatssrSpec;
impl crate::RegisterSpec for MacatssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macatssr::R`](R) reader structure"]
impl crate::Readable for MacatssrSpec {}
#[doc = "`reset()` method sets MACATSSR to value 0"]
impl crate::Resettable for MacatssrSpec {}
