#[doc = "Register `RX_LPI_USEC_CNTR` reader"]
pub type R = crate::R<RxLpiUsecCntrSpec>;
#[doc = "Field `RXLPIUSC` reader - RXLPIUSC"]
pub type RxlpiuscR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RXLPIUSC"]
    #[inline(always)]
    pub fn rxlpiusc(&self) -> RxlpiuscR {
        RxlpiuscR::new(self.bits)
    }
}
#[doc = "Rx LPI microsecond counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_lpi_usec_cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxLpiUsecCntrSpec;
impl crate::RegisterSpec for RxLpiUsecCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_lpi_usec_cntr::R`](R) reader structure"]
impl crate::Readable for RxLpiUsecCntrSpec {}
#[doc = "`reset()` method sets RX_LPI_USEC_CNTR to value 0"]
impl crate::Resettable for RxLpiUsecCntrSpec {}
