#[doc = "Register `DSI_GPSR` reader"]
pub type R = crate::R<DsiGpsrSpec>;
#[doc = "Field `CMDFE` reader - CMDFE"]
pub type CmdfeR = crate::BitReader;
#[doc = "Field `CMDFF` reader - CMDFF"]
pub type CmdffR = crate::BitReader;
#[doc = "Field `PWRFE` reader - PWRFE"]
pub type PwrfeR = crate::BitReader;
#[doc = "Field `PWRFF` reader - PWRFF"]
pub type PwrffR = crate::BitReader;
#[doc = "Field `PRDFE` reader - PRDFE"]
pub type PrdfeR = crate::BitReader;
#[doc = "Field `PRDFF` reader - PRDFF"]
pub type PrdffR = crate::BitReader;
#[doc = "Field `RCB` reader - RCB"]
pub type RcbR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CMDFE"]
    #[inline(always)]
    pub fn cmdfe(&self) -> CmdfeR {
        CmdfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMDFF"]
    #[inline(always)]
    pub fn cmdff(&self) -> CmdffR {
        CmdffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWRFE"]
    #[inline(always)]
    pub fn pwrfe(&self) -> PwrfeR {
        PwrfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWRFF"]
    #[inline(always)]
    pub fn pwrff(&self) -> PwrffR {
        PwrffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRDFE"]
    #[inline(always)]
    pub fn prdfe(&self) -> PrdfeR {
        PrdfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRDFF"]
    #[inline(always)]
    pub fn prdff(&self) -> PrdffR {
        PrdffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RCB"]
    #[inline(always)]
    pub fn rcb(&self) -> RcbR {
        RcbR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "DSI Host generic packet status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_gpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiGpsrSpec;
impl crate::RegisterSpec for DsiGpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_gpsr::R`](R) reader structure"]
impl crate::Readable for DsiGpsrSpec {}
#[doc = "`reset()` method sets DSI_GPSR to value 0x15"]
impl crate::Resettable for DsiGpsrSpec {
    const RESET_VALUE: u32 = 0x15;
}
