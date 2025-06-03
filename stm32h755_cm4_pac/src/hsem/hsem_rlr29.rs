#[doc = "Register `HSEM_RLR29` reader"]
pub type R = crate::R<HsemRlr29Spec>;
#[doc = "Field `PROCID` reader - Semaphore ProcessID"]
pub type ProcidR = crate::FieldReader;
#[doc = "Field `COREID` reader - Semaphore COREID"]
pub type CoreidR = crate::FieldReader;
#[doc = "Field `LOCK` reader - Lock indication"]
pub type LockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> ProcidR {
        ProcidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Semaphore COREID"]
    #[inline(always)]
    pub fn coreid(&self) -> CoreidR {
        CoreidR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr29::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsemRlr29Spec;
impl crate::RegisterSpec for HsemRlr29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_rlr29::R`](R) reader structure"]
impl crate::Readable for HsemRlr29Spec {}
#[doc = "`reset()` method sets HSEM_RLR29 to value 0"]
impl crate::Resettable for HsemRlr29Spec {}
