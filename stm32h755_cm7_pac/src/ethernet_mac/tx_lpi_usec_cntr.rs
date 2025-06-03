#[doc = "Register `TX_LPI_USEC_CNTR` reader"]
pub type R = crate::R<TxLpiUsecCntrSpec>;
#[doc = "Field `TXLPIUSC` reader - TXLPIUSC"]
pub type TxlpiuscR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXLPIUSC"]
    #[inline(always)]
    pub fn txlpiusc(&self) -> TxlpiuscR {
        TxlpiuscR::new(self.bits)
    }
}
#[doc = "Tx LPI microsecond timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_lpi_usec_cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxLpiUsecCntrSpec;
impl crate::RegisterSpec for TxLpiUsecCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_lpi_usec_cntr::R`](R) reader structure"]
impl crate::Readable for TxLpiUsecCntrSpec {}
#[doc = "`reset()` method sets TX_LPI_USEC_CNTR to value 0"]
impl crate::Resettable for TxLpiUsecCntrSpec {}
