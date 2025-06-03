#[doc = "Register `TX_LPI_TRAN_CNTR` reader"]
pub type R = crate::R<TxLpiTranCntrSpec>;
#[doc = "Field `TXLPITRC` reader - TXLPITRC"]
pub type TxlpitrcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXLPITRC"]
    #[inline(always)]
    pub fn txlpitrc(&self) -> TxlpitrcR {
        TxlpitrcR::new(self.bits)
    }
}
#[doc = "Tx LPI transition counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_lpi_tran_cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxLpiTranCntrSpec;
impl crate::RegisterSpec for TxLpiTranCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_lpi_tran_cntr::R`](R) reader structure"]
impl crate::Readable for TxLpiTranCntrSpec {}
#[doc = "`reset()` method sets TX_LPI_TRAN_CNTR to value 0"]
impl crate::Resettable for TxLpiTranCntrSpec {}
