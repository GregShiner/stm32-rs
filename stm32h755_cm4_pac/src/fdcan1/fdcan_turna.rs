#[doc = "Register `FDCAN_TURNA` reader"]
pub type R = crate::R<FdcanTurnaSpec>;
#[doc = "Field `NAV` reader - Numerator Actual Value"]
pub type NavR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:17 - Numerator Actual Value"]
    #[inline(always)]
    pub fn nav(&self) -> NavR {
        NavR::new(self.bits & 0x0003_ffff)
    }
}
#[doc = "FDCAN TUR Numerator Actual Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_turna::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTurnaSpec;
impl crate::RegisterSpec for FdcanTurnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_turna::R`](R) reader structure"]
impl crate::Readable for FdcanTurnaSpec {}
#[doc = "`reset()` method sets FDCAN_TURNA to value 0"]
impl crate::Resettable for FdcanTurnaSpec {}
