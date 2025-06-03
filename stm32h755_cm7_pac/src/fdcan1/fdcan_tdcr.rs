#[doc = "Register `FDCAN_TDCR` reader"]
pub type R = crate::R<FdcanTdcrSpec>;
#[doc = "Field `TDCF` reader - Transmitter Delay Compensation Filter Window Length"]
pub type TdcfR = crate::FieldReader;
#[doc = "Field `TDCO` reader - Transmitter Delay Compensation Offset"]
pub type TdcoR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Transmitter Delay Compensation Filter Window Length"]
    #[inline(always)]
    pub fn tdcf(&self) -> TdcfR {
        TdcfR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Transmitter Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&self) -> TdcoR {
        TdcoR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
#[doc = "FDCAN Transmitter Delay Compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tdcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTdcrSpec;
impl crate::RegisterSpec for FdcanTdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tdcr::R`](R) reader structure"]
impl crate::Readable for FdcanTdcrSpec {}
#[doc = "`reset()` method sets FDCAN_TDCR to value 0"]
impl crate::Resettable for FdcanTdcrSpec {}
