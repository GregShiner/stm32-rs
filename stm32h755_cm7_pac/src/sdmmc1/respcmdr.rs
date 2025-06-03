#[doc = "Register `RESPCMDR` reader"]
pub type R = crate::R<RespcmdrSpec>;
#[doc = "Field `RESPCMD` reader - Response command index"]
pub type RespcmdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Response command index"]
    #[inline(always)]
    pub fn respcmd(&self) -> RespcmdR {
        RespcmdR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "SDMMC command response register\n\nYou can [`read`](crate::Reg::read) this register and get [`respcmdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RespcmdrSpec;
impl crate::RegisterSpec for RespcmdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`respcmdr::R`](R) reader structure"]
impl crate::Readable for RespcmdrSpec {}
#[doc = "`reset()` method sets RESPCMDR to value 0xa3c5_dd01"]
impl crate::Resettable for RespcmdrSpec {
    const RESET_VALUE: u32 = 0xa3c5_dd01;
}
