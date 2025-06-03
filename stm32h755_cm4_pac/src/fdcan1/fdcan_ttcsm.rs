#[doc = "Register `FDCAN_TTCSM` reader"]
pub type R = crate::R<FdcanTtcsmSpec>;
#[doc = "Field `CSM` reader - Cycle Sync Mark"]
pub type CsmR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Cycle Sync Mark"]
    #[inline(always)]
    pub fn csm(&self) -> CsmR {
        CsmR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT Cycle Sync Mark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttcsm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTtcsmSpec;
impl crate::RegisterSpec for FdcanTtcsmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttcsm::R`](R) reader structure"]
impl crate::Readable for FdcanTtcsmSpec {}
#[doc = "`reset()` method sets FDCAN_TTCSM to value 0"]
impl crate::Resettable for FdcanTtcsmSpec {}
