#[doc = "Register `FDCAN_ENDN` reader"]
pub type R = crate::R<FdcanEndnSpec>;
#[doc = "Field `ETV` reader - Endiannes Test Value"]
pub type EtvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Endiannes Test Value"]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(self.bits)
    }
}
#[doc = "FDCAN Core Release Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_endn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanEndnSpec;
impl crate::RegisterSpec for FdcanEndnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_endn::R`](R) reader structure"]
impl crate::Readable for FdcanEndnSpec {}
#[doc = "`reset()` method sets FDCAN_ENDN to value 0"]
impl crate::Resettable for FdcanEndnSpec {}
