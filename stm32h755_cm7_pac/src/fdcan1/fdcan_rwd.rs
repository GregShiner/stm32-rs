#[doc = "Register `FDCAN_RWD` reader"]
pub type R = crate::R<FdcanRwdSpec>;
#[doc = "Field `WDC` reader - Watchdog configuration"]
pub type WdcR = crate::FieldReader;
#[doc = "Field `WDV` reader - Watchdog value"]
pub type WdvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Watchdog configuration"]
    #[inline(always)]
    pub fn wdc(&self) -> WdcR {
        WdcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Watchdog value"]
    #[inline(always)]
    pub fn wdv(&self) -> WdvR {
        WdvR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "FDCAN RAM Watchdog Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rwd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRwdSpec;
impl crate::RegisterSpec for FdcanRwdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rwd::R`](R) reader structure"]
impl crate::Readable for FdcanRwdSpec {}
#[doc = "`reset()` method sets FDCAN_RWD to value 0"]
impl crate::Resettable for FdcanRwdSpec {}
