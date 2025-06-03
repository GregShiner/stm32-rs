#[doc = "Register `MMC_RX_INTERRUPT` reader"]
pub type R = crate::R<MmcRxInterruptSpec>;
#[doc = "Field `RXCRCERPIS` reader - RXCRCERPIS"]
pub type RxcrcerpisR = crate::BitReader;
#[doc = "Field `RXALGNERPIS` reader - RXALGNERPIS"]
pub type RxalgnerpisR = crate::BitReader;
#[doc = "Field `RXUCGPIS` reader - RXUCGPIS"]
pub type RxucgpisR = crate::BitReader;
#[doc = "Field `RXLPIUSCIS` reader - RXLPIUSCIS"]
pub type RxlpiuscisR = crate::BitReader;
#[doc = "Field `RXLPITRCIS` reader - RXLPITRCIS"]
pub type RxlpitrcisR = crate::BitReader;
impl R {
    #[doc = "Bit 5 - RXCRCERPIS"]
    #[inline(always)]
    pub fn rxcrcerpis(&self) -> RxcrcerpisR {
        RxcrcerpisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXALGNERPIS"]
    #[inline(always)]
    pub fn rxalgnerpis(&self) -> RxalgnerpisR {
        RxalgnerpisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - RXUCGPIS"]
    #[inline(always)]
    pub fn rxucgpis(&self) -> RxucgpisR {
        RxucgpisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 26 - RXLPIUSCIS"]
    #[inline(always)]
    pub fn rxlpiuscis(&self) -> RxlpiuscisR {
        RxlpiuscisR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RXLPITRCIS"]
    #[inline(always)]
    pub fn rxlpitrcis(&self) -> RxlpitrcisR {
        RxlpitrcisR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "MMC Rx interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxInterruptSpec;
impl crate::RegisterSpec for MmcRxInterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rx_interrupt::R`](R) reader structure"]
impl crate::Readable for MmcRxInterruptSpec {}
#[doc = "`reset()` method sets MMC_RX_INTERRUPT to value 0"]
impl crate::Resettable for MmcRxInterruptSpec {}
