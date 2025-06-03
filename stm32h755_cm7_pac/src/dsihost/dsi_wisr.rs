#[doc = "Register `DSI_WISR` reader"]
pub type R = crate::R<DsiWisrSpec>;
#[doc = "Field `TEIF` reader - TEIF"]
pub type TeifR = crate::BitReader;
#[doc = "Field `ERIF` reader - ERIF"]
pub type ErifR = crate::BitReader;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `PLLLS` reader - PLLLS"]
pub type PlllsR = crate::BitReader;
#[doc = "Field `PLLLIF` reader - PLLLIF"]
pub type PlllifR = crate::BitReader;
#[doc = "Field `PLLUIF` reader - PLLUIF"]
pub type PlluifR = crate::BitReader;
#[doc = "Field `RRS` reader - RRS"]
pub type RrsR = crate::BitReader;
#[doc = "Field `RRIF` reader - RRIF"]
pub type RrifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TEIF"]
    #[inline(always)]
    pub fn teif(&self) -> TeifR {
        TeifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ERIF"]
    #[inline(always)]
    pub fn erif(&self) -> ErifR {
        ErifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - PLLLS"]
    #[inline(always)]
    pub fn pllls(&self) -> PlllsR {
        PlllsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLLLIF"]
    #[inline(always)]
    pub fn plllif(&self) -> PlllifR {
        PlllifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLLUIF"]
    #[inline(always)]
    pub fn plluif(&self) -> PlluifR {
        PlluifR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - RRS"]
    #[inline(always)]
    pub fn rrs(&self) -> RrsR {
        RrsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RRIF"]
    #[inline(always)]
    pub fn rrif(&self) -> RrifR {
        RrifR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "DSI wrapper interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiWisrSpec;
impl crate::RegisterSpec for DsiWisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wisr::R`](R) reader structure"]
impl crate::Readable for DsiWisrSpec {}
#[doc = "`reset()` method sets DSI_WISR to value 0"]
impl crate::Resettable for DsiWisrSpec {}
