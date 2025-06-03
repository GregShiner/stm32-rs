#[doc = "Register `MTLISR` reader"]
pub type R = crate::R<MtlisrSpec>;
#[doc = "Field `Q0IS` reader - Queue interrupt status"]
pub type Q0isR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Queue interrupt status"]
    #[inline(always)]
    pub fn q0is(&self) -> Q0isR {
        Q0isR::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtlisrSpec;
impl crate::RegisterSpec for MtlisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlisr::R`](R) reader structure"]
impl crate::Readable for MtlisrSpec {}
#[doc = "`reset()` method sets MTLISR to value 0"]
impl crate::Resettable for MtlisrSpec {}
