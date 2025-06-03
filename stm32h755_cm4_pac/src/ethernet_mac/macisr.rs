#[doc = "Register `MACISR` reader"]
pub type R = crate::R<MacisrSpec>;
#[doc = "Field `PHYIS` reader - PHYIS"]
pub type PhyisR = crate::BitReader;
#[doc = "Field `PMTIS` reader - PMTIS"]
pub type PmtisR = crate::BitReader;
#[doc = "Field `LPIIS` reader - LPIIS"]
pub type LpiisR = crate::BitReader;
#[doc = "Field `MMCIS` reader - MMCIS"]
pub type MmcisR = crate::BitReader;
#[doc = "Field `MMCRXIS` reader - MMCRXIS"]
pub type MmcrxisR = crate::BitReader;
#[doc = "Field `MMCTXIS` reader - MMCTXIS"]
pub type MmctxisR = crate::BitReader;
#[doc = "Field `TSIS` reader - TSIS"]
pub type TsisR = crate::BitReader;
#[doc = "Field `TXSTSIS` reader - TXSTSIS"]
pub type TxstsisR = crate::BitReader;
#[doc = "Field `RXSTSIS` reader - RXSTSIS"]
pub type RxstsisR = crate::BitReader;
impl R {
    #[doc = "Bit 3 - PHYIS"]
    #[inline(always)]
    pub fn phyis(&self) -> PhyisR {
        PhyisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMTIS"]
    #[inline(always)]
    pub fn pmtis(&self) -> PmtisR {
        PmtisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPIIS"]
    #[inline(always)]
    pub fn lpiis(&self) -> LpiisR {
        LpiisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - MMCIS"]
    #[inline(always)]
    pub fn mmcis(&self) -> MmcisR {
        MmcisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MMCRXIS"]
    #[inline(always)]
    pub fn mmcrxis(&self) -> MmcrxisR {
        MmcrxisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MMCTXIS"]
    #[inline(always)]
    pub fn mmctxis(&self) -> MmctxisR {
        MmctxisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - TSIS"]
    #[inline(always)]
    pub fn tsis(&self) -> TsisR {
        TsisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TXSTSIS"]
    #[inline(always)]
    pub fn txstsis(&self) -> TxstsisR {
        TxstsisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RXSTSIS"]
    #[inline(always)]
    pub fn rxstsis(&self) -> RxstsisR {
        RxstsisR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`macisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacisrSpec;
impl crate::RegisterSpec for MacisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macisr::R`](R) reader structure"]
impl crate::Readable for MacisrSpec {}
#[doc = "`reset()` method sets MACISR to value 0"]
impl crate::Resettable for MacisrSpec {}
