#[doc = "Register `MACATSNR` reader"]
pub type R = crate::R<MacatsnrSpec>;
#[doc = "Field `AUXTSLO` reader - AUXTSLO"]
pub type AuxtsloR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - AUXTSLO"]
    #[inline(always)]
    pub fn auxtslo(&self) -> AuxtsloR {
        AuxtsloR::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "Auxiliary timestamp nanoseconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macatsnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacatsnrSpec;
impl crate::RegisterSpec for MacatsnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macatsnr::R`](R) reader structure"]
impl crate::Readable for MacatsnrSpec {}
#[doc = "`reset()` method sets MACATSNR to value 0"]
impl crate::Resettable for MacatsnrSpec {}
