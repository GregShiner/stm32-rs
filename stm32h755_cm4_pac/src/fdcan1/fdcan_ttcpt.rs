#[doc = "Register `FDCAN_TTCPT` reader"]
pub type R = crate::R<FdcanTtcptSpec>;
#[doc = "Field `CT` reader - Cycle Count Value"]
pub type CtR = crate::FieldReader;
#[doc = "Field `SWV` reader - Stop Watch Value"]
pub type SwvR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:5 - Cycle Count Value"]
    #[inline(always)]
    pub fn ct(&self) -> CtR {
        CtR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - Stop Watch Value"]
    #[inline(always)]
    pub fn swv(&self) -> SwvR {
        SwvR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT Capture Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttcpt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTtcptSpec;
impl crate::RegisterSpec for FdcanTtcptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttcpt::R`](R) reader structure"]
impl crate::Readable for FdcanTtcptSpec {}
#[doc = "`reset()` method sets FDCAN_TTCPT to value 0"]
impl crate::Resettable for FdcanTtcptSpec {}
