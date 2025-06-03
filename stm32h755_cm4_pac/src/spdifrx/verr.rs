#[doc = "Register `VERR` reader"]
pub type R = crate::R<VerrSpec>;
#[doc = "Field `MINREV` reader - Minor revision"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MAJREV` reader - Major revision"]
pub type MajrevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Minor revision"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major revision"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SPDIFRX version register\n\nYou can [`read`](crate::Reg::read) this register and get [`verr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VerrSpec;
impl crate::RegisterSpec for VerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`verr::R`](R) reader structure"]
impl crate::Readable for VerrSpec {}
#[doc = "`reset()` method sets VERR to value 0x12"]
impl crate::Resettable for VerrSpec {
    const RESET_VALUE: u32 = 0x12;
}
