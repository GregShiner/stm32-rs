#[doc = "Register `SIDR` reader"]
pub type R = crate::R<SidrSpec>;
#[doc = "Field `SID` reader - Size identification"]
pub type SidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Size identification"]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(self.bits)
    }
}
#[doc = "SPDIFRX size identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`sidr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SidrSpec;
impl crate::RegisterSpec for SidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sidr::R`](R) reader structure"]
impl crate::Readable for SidrSpec {}
#[doc = "`reset()` method sets SIDR to value 0xa3c5_dd01"]
impl crate::Resettable for SidrSpec {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
