#[doc = "Register `PECR` reader"]
pub type R = crate::R<PecrSpec>;
#[doc = "Field `PEC` reader - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0."]
pub type PecR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0."]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Access: No wait states\n\nYou can [`read`](crate::Reg::read) this register and get [`pecr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PecrSpec;
impl crate::RegisterSpec for PecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pecr::R`](R) reader structure"]
impl crate::Readable for PecrSpec {}
#[doc = "`reset()` method sets PECR to value 0"]
impl crate::Resettable for PecrSpec {}
