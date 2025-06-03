#[doc = "Register `MMC_TX_INTERRUPT` reader"]
pub type R = crate::R<MmcTxInterruptSpec>;
#[doc = "Field `TXSCOLGPIS` reader - TXSCOLGPIS"]
pub type TxscolgpisR = crate::BitReader;
#[doc = "Field `TXMCOLGPIS` reader - TXMCOLGPIS"]
pub type TxmcolgpisR = crate::BitReader;
#[doc = "Field `TXGPKTIS` reader - TXGPKTIS"]
pub type TxgpktisR = crate::BitReader;
#[doc = "Field `TXLPIUSCIS` reader - TXLPIUSCIS"]
pub type TxlpiuscisR = crate::BitReader;
#[doc = "Field `TXLPITRCIS` reader - TXLPITRCIS"]
pub type TxlpitrcisR = crate::BitReader;
impl R {
    #[doc = "Bit 14 - TXSCOLGPIS"]
    #[inline(always)]
    pub fn txscolgpis(&self) -> TxscolgpisR {
        TxscolgpisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TXMCOLGPIS"]
    #[inline(always)]
    pub fn txmcolgpis(&self) -> TxmcolgpisR {
        TxmcolgpisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - TXGPKTIS"]
    #[inline(always)]
    pub fn txgpktis(&self) -> TxgpktisR {
        TxgpktisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - TXLPIUSCIS"]
    #[inline(always)]
    pub fn txlpiuscis(&self) -> TxlpiuscisR {
        TxlpiuscisR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXLPITRCIS"]
    #[inline(always)]
    pub fn txlpitrcis(&self) -> TxlpitrcisR {
        TxlpitrcisR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "MMC Tx interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcTxInterruptSpec;
impl crate::RegisterSpec for MmcTxInterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_tx_interrupt::R`](R) reader structure"]
impl crate::Readable for MmcTxInterruptSpec {}
#[doc = "`reset()` method sets MMC_TX_INTERRUPT to value 0"]
impl crate::Resettable for MmcTxInterruptSpec {}
