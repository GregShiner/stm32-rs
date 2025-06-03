#[doc = "Register `RX_LPI_TRAN_CNTR` reader"]
pub type R = crate::R<RxLpiTranCntrSpec>;
#[doc = "Field `RXLPITRC` reader - RXLPITRC"]
pub type RxlpitrcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RXLPITRC"]
    #[inline(always)]
    pub fn rxlpitrc(&self) -> RxlpitrcR {
        RxlpitrcR::new(self.bits)
    }
}
#[doc = "Rx LPI transition counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_lpi_tran_cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxLpiTranCntrSpec;
impl crate::RegisterSpec for RxLpiTranCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_lpi_tran_cntr::R`](R) reader structure"]
impl crate::Readable for RxLpiTranCntrSpec {}
#[doc = "`reset()` method sets RX_LPI_TRAN_CNTR to value 0"]
impl crate::Resettable for RxLpiTranCntrSpec {}
